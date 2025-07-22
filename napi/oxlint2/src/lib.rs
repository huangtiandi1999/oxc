use std::{
    process::{ExitCode, Termination},
    sync::{Arc, atomic::Ordering, mpsc::channel},
};

use napi::{
    Status,
    bindgen_prelude::{Promise, Uint8Array},
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use napi_derive::napi;

use oxc_allocator::{Allocator, free_fixed_size_allocator};
use oxlint::{
    ExternalLinter, ExternalLinterCb, ExternalLinterLoadPluginCb, LintResult, PluginLoadResult,
    lint as oxlint_lint,
};

mod generated {
    pub mod raw_transfer_constants;
}
use generated::raw_transfer_constants::{BLOCK_ALIGN, BUFFER_SIZE};

#[napi]
pub type JsRunCb = ThreadsafeFunction<
    (String, u32, Option<Uint8Array>, Vec<u32>),
    String, /* Vec<LintResult> */
    (String, u32, Option<Uint8Array>, Vec<u32>),
    Status,
    false,
>;

#[napi]
pub type JsLoadPluginCb = ThreadsafeFunction<
    String, /* PluginName */
    Promise<String /* PluginLoadResult */>,
    String, /* PluginName */
    Status,
    false,
>;

fn wrap_run(cb: JsRunCb) -> ExternalLinterCb {
    let cb = Arc::new(cb);
    Arc::new(move |file_path: String, rule_ids: Vec<u32>, allocator: &Allocator| {
        let cb = Arc::clone(&cb);

        let (tx, rx) = channel();

        // Each buffer is sent over to JS only once.
        // JS side stores them in an array, and holds them until process ends.
        // A flag in `FixedSizeAllocatorMetadata` records whether the buffer has already been transferred
        // to JS or not. If it hasn't, send it. Otherwise, just send the ID of the buffer which is the
        // index of that buffer in the array on JS side, and JS side will get the buffer from the array.
        // This means there's only even 1 instance of a buffer on Rust side, and 1 on JS side,
        // which makes it simpler to avoid use-after-free or double-free problems.

        // SAFETY: This crate enables the `fixed_size` feature on `oxc_allocator`, so all AST `Allocator`s
        // are created via `FixedSizeAllocator`. We only create an immutable ref from this pointer.
        let metadata_ptr = unsafe { allocator.fixed_size_metadata_ptr() };
        let (buffer_id, already_sent_to_js) = {
            // SAFETY: Fixed-size allocators always have a valid `FixedSizeAllocatorMetadata`
            // stored at the pointer returned by `Allocator::fixed_size_metadata_ptr`.
            let metadata = unsafe { metadata_ptr.as_ref() };
            // TODO: Is `Ordering::SeqCst` excessive here?
            let already_sent_to_js = metadata.is_double_owned.swap(true, Ordering::SeqCst);

            (metadata.id, already_sent_to_js)
        };

        let buffer = if already_sent_to_js {
            // Buffer has already been sent to JS. Don't send it again.
            None
        } else {
            // Buffer has not already been sent to JS. Send it.

            // Get pointer to start of allocator chunk.
            // Note: `Allocator::data_ptr` would not provide the right pointer, because source text
            // gets written to start of the allocator chunk, and `data_ptr` gets moved to after it.
            // SAFETY: Fixed-size allocators have their chunk aligned on `BLOCK_ALIGN`,
            // and size less than `BLOCK_ALIGN`. So we can get pointer to start of `Allocator` chunk
            // by rounding down to next multiple of `BLOCK_ALIGN`. That can't go out of bounds of
            // the backing allocation.
            let chunk_ptr = unsafe {
                let ptr = metadata_ptr.cast::<u8>();
                let offset = ptr.as_ptr() as usize % BLOCK_ALIGN;
                ptr.sub(offset)
            };

            // SAFETY:
            // Range of memory starting at `chunk_ptr` and encompassing `BUFFER_SIZE` is all within
            // the allocation backing the `Allocator`.
            //
            // We can't prove that no mutable references to data in the buffer exist,
            // but there shouldn't be any, because linter doesn't mutate the AST.
            // Anyway, I (@overlookmotel) am not sure if the aliasing rules apply to code in another
            // language. Probably not, as JS code is outside the domain of the "Rust abstract machine".
            // As long as we don't mutate data in the buffer on JS side, it should be fine.
            //
            // On the other side, while many immutable references to data in the buffer exist
            // (`AstKind`s for every AST node), JS side does not mutate the data in the buffer,
            // so that shouldn't break the guarantees of `&` references.
            //
            // This is all a bit wavy, but such is the way with sharing memory outside of Rust.
            let buffer = unsafe {
                Uint8Array::with_external_data(
                    chunk_ptr.as_ptr(),
                    BUFFER_SIZE,
                    move |_ptr, _len| free_fixed_size_allocator(metadata_ptr),
                )
            };
            Some(buffer)
        };

        // Send data to JS
        let status = cb.call_with_return_value(
            (file_path, buffer_id, buffer, rule_ids),
            ThreadsafeFunctionCallMode::NonBlocking,
            move |result, _env| {
                let _ = match &result {
                    Ok(r) => match serde_json::from_str::<Vec<LintResult>>(r) {
                        Ok(v) => tx.send(Ok(v)),
                        Err(_e) => tx.send(Err("Failed to deserialize lint result".to_string())),
                    },
                    Err(e) => tx.send(Err(e.to_string())),
                };

                result.map(|_| ())
            },
        );

        if status != Status::Ok {
            return Err(format!("Failed to schedule callback: {status:?}").into());
        }

        match rx.recv() {
            Ok(Ok(x)) => Ok(x),
            Ok(Err(e)) => Err(format!("Callback reported error: {e}").into()),
            Err(e) => Err(format!("Callback did not respond: {e}").into()),
        }
    })
}

fn wrap_load_plugin(cb: JsLoadPluginCb) -> ExternalLinterLoadPluginCb {
    let cb = Arc::new(cb);
    Arc::new(move |plugin_name| {
        Box::pin({
            let cb = Arc::clone(&cb);
            async move {
                let result = cb.call_async(plugin_name).await?.into_future().await?;
                let plugin_load_result: PluginLoadResult = serde_json::from_str(&result)?;
                Ok(plugin_load_result)
            }
        })
    })
}

#[expect(clippy::allow_attributes)]
#[allow(clippy::trailing_empty_array, clippy::unused_async)] // https://github.com/napi-rs/napi-rs/issues/2758
#[napi]
pub async fn lint(load_plugin: JsLoadPluginCb, run: JsRunCb) -> bool {
    let rust_load_plugin = wrap_load_plugin(load_plugin);
    let rust_run = wrap_run(run);

    oxlint_lint(Some(ExternalLinter::new(rust_run, rust_load_plugin))).report() == ExitCode::SUCCESS
}
