use std::mem;

use oxc_allocator::{Address, Vec as ArenaVec};
use oxc_ast::{ast::*, match_expression};
use oxc_span::GetSpan;

use crate::{
    Buffer, Format, FormatResult, FormatTrailingCommas, TrailingSeparator, format_args,
    formatter::{
        BufferExtensions, Comments, FormatElement, FormatError, Formatter, VecBuffer,
        format_element,
        prelude::{
            FormatElements, FormatOnce, FormatWith, MemoizeFormat, Tag, empty_line, expand_parent,
            format_once, format_with, get_lines_before, group, soft_block_indent,
            soft_line_break_or_space, space,
        },
        separated::FormatSeparatedIter,
        trivia::{DanglingIndentMode, format_dangling_comments},
        write,
    },
    generated::ast_nodes::{AstNode, AstNodes},
    utils::{
        call_expression::is_test_call_expression, is_long_curried_call,
        member_chain::simple_argument::SimpleArgument, write_arguments_multi_line,
    },
    write,
    write::{
        FormatFunctionOptions,
        arrow_function_expression::is_multiline_template_starting_on_same_line,
        parameter_list::has_only_simple_parameters,
    },
};

use super::{
    array_element_list::can_concisely_print_array_list,
    arrow_function_expression::{
        FormatJsArrowFunctionExpression, FormatJsArrowFunctionExpressionOptions,
        FunctionBodyCacheMode, GroupedCallArgumentLayout,
    },
    function,
};

impl<'a> Format<'a> for AstNode<'a, ArenaVec<'a, Argument<'a>>> {
    fn fmt(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let l_paren_token = "(";
        let r_paren_token = ")";
        if self.is_empty() {
            return write!(
                f,
                [
                    l_paren_token,
                    // `call/* comment1 */(/* comment2 */)` Both comments are dangling comments.
                    format_dangling_comments(self.parent.span()).with_soft_block_indent(),
                    r_paren_token
                ]
            );
        }

        let call_expression =
            if let AstNodes::CallExpression(call) = self.parent { Some(call) } else { None };

        let (is_commonjs_or_amd_call, is_test_call) = if let Some(call) = call_expression {
            (is_commonjs_or_amd_call(self, call), is_test_call_expression(call))
        } else {
            (false, false)
        };

        let is_first_arg_string_literal_or_template = self.len() != 2
            || matches!(
                self.as_ref().first(),
                Some(
                    Argument::StringLiteral(_)
                        | Argument::TemplateLiteral(_)
                        | Argument::TaggedTemplateExpression(_)
                )
            );

        if is_commonjs_or_amd_call
            || is_multiline_template_only_args(self, f.source_text())
            || is_react_hook_with_deps_array(self, f.comments())
            || (is_test_call && is_first_arg_string_literal_or_template)
        {
            return write!(
                f,
                [
                    l_paren_token,
                    format_with(|f| {
                        f.join_with(space())
                            .entries(
                                FormatSeparatedIter::new(self.iter(), ",")
                                    .with_trailing_separator(TrailingSeparator::Omit),
                            )
                            .finish()
                    }),
                    r_paren_token
                ]
            );
        }

        let mut has_empty_line = false;

        let has_empty_line = self.iter().any(|arg| get_lines_before(arg.span(), f) > 1);

        if has_empty_line || is_function_composition_args(self) {
            return format_all_args_broken_out(self, true, f);
        }

        if let Some(group_layout) = arguments_grouped_layout(self, f) {
            write_grouped_arguments(self, group_layout, f)
        } else if call_expression.is_some_and(|call| is_long_curried_call(call)) {
            write!(
                f,
                [
                    l_paren_token,
                    soft_block_indent(&format_once(|f| {
                        write_arguments_multi_line(
                            FormatSeparatedIter::new(self.iter(), ",")
                                .with_trailing_separator(TrailingSeparator::Omit),
                            f,
                        )
                    })),
                    r_paren_token,
                ]
            )
        } else {
            format_all_args_broken_out(self, false, f)
        }
    }
}

/// Tests if a call has multiple anonymous function like (arrow or function expression) arguments.
///
/// ## Examples
///
/// ```javascript
/// compose(sortBy(x => x), flatten, map(x => [x, x*2]));
/// ```
pub fn is_function_composition_args(args: &[Argument<'_>]) -> bool {
    if args.len() <= 1 {
        return false;
    }

    let mut has_seen_function_like = false;

    for arg in args {
        match arg {
            Argument::FunctionExpression(_) | Argument::ArrowFunctionExpression(_) => {
                if has_seen_function_like {
                    return true;
                }
                has_seen_function_like = true;
            }
            Argument::CallExpression(call) => {
                if call.arguments.iter().any(|arg| {
                    matches!(
                        arg,
                        Argument::FunctionExpression(_) | Argument::ArrowFunctionExpression(_)
                    )
                }) {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

fn format_all_elements_broken_out<'a, 'b>(
    elements: impl Iterator<Item = (FormatResult<Option<FormatElement<'a>>>, usize)>,
    node: &'b AstNode<'a, ArenaVec<'a, Argument<'a>>>,
    expand: bool,
    mut buffer: impl Buffer<'a>,
) -> FormatResult<()> {
    let is_inside_import = matches!(node.parent, AstNodes::ImportExpression(_));
    write!(
        buffer,
        [group(&format_args!(
            "(",
            soft_block_indent(&format_once(move |f| {
                for (index, (element, lines_before)) in elements.into_iter().enumerate() {
                    if let Ok(Some(element)) = element {
                        if index > 0 {
                            match lines_before {
                                0 | 1 => write!(f, [soft_line_break_or_space()])?,
                                _ => write!(f, [empty_line()])?,
                            }
                        }

                        f.write_element(element)?;
                    }
                }

                write!(f, [(!is_inside_import).then_some(FormatTrailingCommas::All)])
            })),
            ")",
        ))
        .should_expand(expand)]
    )
}

fn format_all_args_broken_out<'a, 'b>(
    node: &'b AstNode<'a, ArenaVec<'a, Argument<'a>>>,
    expand: bool,
    mut buffer: impl Buffer<'a>,
) -> FormatResult<()> {
    let is_inside_import = matches!(node.parent, AstNodes::ImportExpression(_));
    let last_index = node.len() - 1;
    write!(
        buffer,
        [group(&format_args!(
            "(",
            soft_block_indent(&format_once(move |f| {
                for (index, argument) in node.iter().enumerate() {
                    if index > 0 {
                        match get_lines_before(argument.span(), f) {
                            0 | 1 => write!(f, [soft_line_break_or_space()])?,
                            _ => write!(f, [empty_line()])?,
                        }
                    }

                    write!(f, [argument, (index != last_index).then_some(",")])?;
                }

                write!(f, [(!is_inside_import).then_some(FormatTrailingCommas::All)])
            })),
            ")",
        ))
        .should_expand(expand)]
    )
}

pub fn arguments_grouped_layout(
    args: &AstNode<ArenaVec<Argument>>,
    f: &Formatter<'_, '_>,
) -> Option<GroupedCallArgumentLayout> {
    if should_group_first_argument(args, f) {
        Some(GroupedCallArgumentLayout::GroupedFirstArgument)
    } else if should_group_last_argument(args, f) {
        Some(GroupedCallArgumentLayout::GroupedLastArgument)
    } else {
        None
    }
}

/// Checks if the first argument requires grouping
fn should_group_first_argument(args: &AstNode<ArenaVec<Argument>>, f: &Formatter<'_, '_>) -> bool {
    let mut iter = args.iter();
    match (iter.next().and_then(|a| a.as_expression()), iter.next().and_then(|a| a.as_expression()))
    {
        (Some(first), Some(second)) if iter.next().is_none() => {
            match &first {
                Expression::FunctionExpression(_) => {}
                // Arrow expressions that are a plain expression or are a chain
                // don't get grouped as the first argument, since they'll either
                // fit entirely on the line or break fully. Only a single arrow
                // with a block body can be grouped to collapse the braces.
                Expression::ArrowFunctionExpression(arrow) => {
                    if arrow.expression {
                        return false;
                    }
                }
                _ => return false,
            }

            if matches!(
                second,
                Expression::ArrowFunctionExpression(_)
                    | Expression::FunctionExpression(_)
                    | Expression::ConditionalExpression(_)
            ) {
                return false;
            }

            let call_like_span = args.parent.span();
            !f.comments().has_comments(call_like_span.start, first.span(), second.span().start)
                && !can_group_expression_argument(second, f)
                && is_relatively_short_argument(second)
        }
        _ => false,
    }
}

/// Checks if the last argument should be grouped.
fn should_group_last_argument(args: &AstNode<ArenaVec<Argument>>, f: &Formatter<'_, '_>) -> bool {
    let mut iter = args.as_ref().iter();
    let last = iter.next_back();

    match last.and_then(|arg| arg.as_expression()) {
        Some(last) => {
            let penultimate = iter.next_back();
            if let Some(penultimate) = &penultimate {
                // TODO: check if both last and penultimate are same kind of expression.
                // if penultimate.syntax().kind() == last.syntax().kind() {
                //     return Ok(false);
                // }
            }

            let call_like_span = args.parent.span();
            let previous_span = penultimate.map_or(call_like_span.start, |a| a.span().end);
            if f.comments().has_comments(previous_span, last.span(), call_like_span.end) {
                return false;
            }

            if !can_group_expression_argument(last, f) {
                return false;
            }

            match last {
                Expression::ArrayExpression(array) if args.len() > 1 => {
                    // Not for `useEffect`
                    if args.len() == 2
                        && matches!(penultimate, Some(Argument::ArrowFunctionExpression(_)))
                    {
                        return false;
                    }

                    if can_concisely_print_array_list(array.span, &array.elements, f) {
                        return false;
                    }

                    true
                }
                _ => true,
            }
        }
        _ => false,
    }
}

/// Check if `ty` is a relatively simple type annotation, allowing a few
/// additional cases through. The simplicity is determined as
/// either being a keyword type or any reference type with no additional type
/// parameters. For example:
/// ```
///     number          => true
///     unknown         => true
///     HTMLElement     => true
///     string | object => false
///     Foo<string>     => false
/// ```
/// This function also introspects into array and generic types to extract the
/// core type, but only to a limited extent:
/// ```
///     string[]        => string
///     string[][]      => string
///     string[][][]    => string
///     Foo<string>[][] => string
///     Foo<string[]>[] => string[]
///     Foo<string[][]> => string[][]
/// ```
fn is_simple_ts_type(ty: &TSType<'_>) -> bool {
    // Reach up to two-levels deep into array types:
    //     string[]     => string
    //     string[][]   => string
    //     string[][][] => string[]
    let extracted_array_type = match ty {
        TSType::TSArrayType(array) => match &array.element_type {
            TSType::TSArrayType(inner_array) => Some(&inner_array.element_type),
            element_type => Some(element_type),
            _ => None,
        },
        _ => None,
    };

    // Then, extract the first type parameter of a Generic as long as it's the
    // only parameter:
    //     Foo<number> => number
    //     Foo<number, string> => Foo<number, string>
    let extracted_generic_type = match &extracted_array_type {
        Some(TSType::TSTypeReference(generic)) => {
            if let Some(type_arguments) = &generic.type_arguments {
                let argument_list = &type_arguments.params;
                if argument_list.len() == 1 { argument_list.first() } else { extracted_array_type }
            } else {
                extracted_array_type
            }
        }
        _ => extracted_array_type,
    };

    let resolved_type = extracted_generic_type.unwrap_or(ty);
    match resolved_type {
        // Any keyword or literal types
        TSType::TSLiteralType(_)
        | TSType::TSTemplateLiteralType(_)
        | TSType::TSThisType(_ | _)
        | TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSObjectKeyword(_)
        | TSType::TSAnyKeyword(_)
        | TSType::TSNeverKeyword(_)
        | TSType::TSNullKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSStringKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSUndefinedKeyword(_)
        | TSType::TSUnknownKeyword(_)
        | TSType::TSVoidKeyword(_) => true,

        // Any reference with no generic type arguments
        TSType::TSTypeReference(reference) => reference.type_arguments.is_none(),

        _ => false,
    }
}

/// Checks if `argument` is "short" enough to be groupable. This aims to be
/// logically similar to Prettier's [`isHopefullyShortCallArgument`](https://github.com/prettier/prettier/blob/093745f0ec429d3db47c1edd823357e0ef24e226/src/language-js/print/call-arguments.js#L279),
fn is_relatively_short_argument(argument: &Expression<'_>) -> bool {
    match argument {
        Expression::BinaryExpression(binary) => {
            SimpleArgument::from(&binary.left).is_simple()
                && SimpleArgument::from(&binary.right).is_simple()
        }
        Expression::LogicalExpression(logical) => {
            SimpleArgument::from(&logical.left).is_simple()
                && SimpleArgument::from(&logical.right).is_simple()
        }
        Expression::TSAsExpression(expr) => {
            is_simple_ts_type(&expr.type_annotation)
                && SimpleArgument::from(&expr.expression).is_simple()
        }
        Expression::TSSatisfiesExpression(expr) => {
            is_simple_ts_type(&expr.type_annotation)
                && SimpleArgument::from(&expr.expression).is_simple()
        }
        Expression::RegExpLiteral(_) => true,
        Expression::CallExpression(call) => match call.arguments.len() {
            0 => true,
            1 => SimpleArgument::from(argument).is_simple(),
            _ => false,
        },
        _ => SimpleArgument::from(argument).is_simple(),
    }
}

/// Checks if `argument` benefits from grouping in call arguments.
fn can_group_expression_argument(argument: &Expression<'_>, f: &Formatter<'_, '_>) -> bool {
    match argument {
        Expression::ObjectExpression(object_expression) => {
            !object_expression.properties.is_empty()
                || f.comments().has_comments_in_span(object_expression.span)
        }

        Expression::ArrayExpression(array_expression) => {
            !array_expression.elements.is_empty()
                || f.comments().has_comments_in_span(array_expression.span)
        }
        Expression::TSTypeAssertion(assertion_expression) => {
            can_group_expression_argument(&assertion_expression.expression, f)
        }

        Expression::TSAsExpression(as_expression) => {
            can_group_expression_argument(&as_expression.expression, f)
        }

        Expression::TSSatisfiesExpression(satisfies_expression) => {
            can_group_expression_argument(&satisfies_expression.expression, f)
        }

        Expression::ArrowFunctionExpression(arrow_function) => {
            can_group_arrow_function_expression_argument(arrow_function, false, f)
        }
        Expression::FunctionExpression(_) => true,
        _ => false,
    }
}

fn can_group_arrow_function_expression_argument(
    arrow_function: &ArrowFunctionExpression,
    is_arrow_recursion: bool,
    f: &Formatter<'_, '_>,
) -> bool {
    let body = &arrow_function.body;
    let return_type_annotation = &arrow_function.return_type;

    // Handles cases like:
    //
    // app.get("/", (req, res): void => {
    //     res.send("Hello World!");
    // });
    //
    // export class Thing implements OtherThing {
    //   do: (type: Type) => Provider<Prop> = memoize(
    //     (type: ObjectType): Provider<Opts> => {}
    //   );
    // }
    let can_group_type = return_type_annotation.as_ref().is_none_or(|any_type| {
        match &any_type.type_annotation {
            TSType::TSTypeReference(_) => {
                if arrow_function.expression {
                    return false;
                }
                body.statements.iter().any(|statement| match statement {
                    Statement::EmptyStatement(s) => {
                        // When the body contains an empty statement, comments in
                        // the body will get attached to that statement rather than
                        // the body itself, so they need to be checked for comments
                        // as well to ensure that the body is still considered
                        // groupable when those empty statements are removed by the
                        // printer.
                        // TODO: it seems no difference if we comment out this line
                        // comments.has_comments(s.span)
                        true
                    }
                    _ => true,
                }) || (body.statements.is_empty() && f.comments().has_dangling_comments(body.span))
            }
            _ => true,
        }
    });

    if !can_group_type {
        return false;
    }

    arrow_function.get_expression().is_none_or(|expr| match expr {
        Expression::ObjectExpression(_)
        | Expression::ArrayExpression(_)
        | Expression::JSXElement(_)
        | Expression::JSXFragment(_) => true,
        Expression::ArrowFunctionExpression(inner_arrow_function) => {
            can_group_arrow_function_expression_argument(inner_arrow_function, true, f)
        }
        Expression::CallExpression(_) | Expression::ConditionalExpression(_) => !is_arrow_recursion,
        _ => false,
    })
}

fn write_grouped_arguments<'a>(
    node: &AstNode<'a, ArenaVec<'a, Argument<'a>>>,
    group_layout: GroupedCallArgumentLayout,
    f: &mut Formatter<'_, 'a>,
) -> FormatResult<()> {
    let last_index = node.len() - 1;
    let only_one_argument = last_index == 0;
    let mut non_grouped_breaks = false;
    let mut grouped_breaks = false;
    let mut has_cached = false;

    // Pre-format the arguments to determine if they can be grouped.
    let mut elements = node
        .iter()
        .enumerate()
        .map(|(index, argument)| {
            let is_grouped_argument = (group_layout.is_grouped_first() && index == 0)
                || (group_layout.is_grouped_last() && index == last_index);

            let format_argument = format_once(|f| {
                if is_grouped_argument {
                    match argument.as_ast_nodes() {
                        AstNodes::Function(function)
                            if !group_layout.is_grouped_first()
                                && (!only_one_argument
                                    || function_has_only_simple_parameters(&function.params)) =>
                        {
                            has_cached = true;
                            return function.fmt_with_options(
                                FormatFunctionOptions {
                                    cache_mode: FunctionBodyCacheMode::Cache,
                                    ..Default::default()
                                },
                                f,
                            );
                        }
                        AstNodes::ArrowFunctionExpression(arrow) => {
                            has_cached = true;
                            return arrow.fmt_with_options(
                                FormatJsArrowFunctionExpressionOptions {
                                    cache_mode: FunctionBodyCacheMode::Cache,
                                    ..FormatJsArrowFunctionExpressionOptions::default()
                                },
                                f,
                            );
                        }
                        _ => {}
                    }
                }
                argument.fmt(f)
            });

            // We have to get the lines before the argument has been formatted, because it relies on
            // the comments before the argument. After formatting, the comments might marked as printed,
            // which would lead to a wrong line count.
            let lines_before = get_lines_before(argument.span(), f);

            let interned = f.intern(
                &format_once(|f| {
                    format_argument.fmt(f)?;
                    write!(f, (last_index != index).then_some(","))
                })
                .memoized(),
            );

            let break_type =
                if is_grouped_argument { &mut grouped_breaks } else { &mut non_grouped_breaks };

            *break_type = *break_type
                || interned
                    .as_ref()
                    .is_ok_and(|i| i.as_ref().is_some_and(FormatElement::will_break));

            (interned, lines_before)
        })
        .collect::<Vec<_>>();

    // If any of the not grouped elements break, then fall back to the variant where
    // all arguments are printed in expanded mode.
    if non_grouped_breaks {
        return format_all_elements_broken_out(elements.into_iter(), node, true, f);
    }

    // We now cache the delimiter tokens. This is needed because `[crate::best_fitting]` will try to
    // print each version first
    let l_paren_token = "(".memoized();
    let r_paren_token = ")".memoized();

    // First write the most expanded variant because it needs `arguments`.
    let most_expanded = {
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        format_all_elements_broken_out(elements.iter().cloned(), node, true, &mut buffer);

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec()
    };

    // Now reformat the first or last argument if they happen to be a function or arrow function expression.
    // Function and arrow function expression apply a custom formatting that removes soft line breaks from the parameters,
    // type parameters, and return type annotation.
    //
    // This implementation caches the function body of the "normal" formatted function or arrow function expression
    // to avoid quadratic complexity if the functions' body contains another call expression with an arrow or function expression
    // as first or last argument.
    let mut grouped = elements;
    if has_cached {
        match group_layout {
            GroupedCallArgumentLayout::GroupedFirstArgument => {
                let argument = node.first().unwrap();
                let mut first = grouped.first_mut().unwrap();
                first.0 = f.intern(&format_once(|f| {
                    FormatGroupedFirstArgument { argument }.fmt(f)?;
                    write!(f, (last_index != 0).then_some(","))
                }));
            }
            GroupedCallArgumentLayout::GroupedLastArgument => {
                let argument = node.last().unwrap();
                let mut last = grouped.last_mut().unwrap();
                last.0 = f.intern(&format_once(|f| {
                    FormatGroupedLastArgument { argument, is_only: only_one_argument }.fmt(f)
                }));
            }
        }
    }

    // Write the most flat variant with the first or last argument grouped.
    let most_flat = {
        // let snapshot = f.state_snapshot();
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        let result = write!(
            buffer,
            [
                l_paren_token,
                format_with(|f| {
                    f.join_with(soft_line_break_or_space())
                        .entries(grouped.iter().map(|(element, _)| {
                            format_once(|f| {
                                if let Some(element) = element.clone()? {
                                    f.write_element(element)
                                } else {
                                    Ok(())
                                }
                            })
                        }))
                        .finish()
                }),
                r_paren_token,
            ]
        );

        // Turns out, using the grouped layout isn't a good fit because some parameters of the
        // grouped function or arrow expression break. In that case, fall back to the all args expanded
        // formatting.
        // This back tracking is required because testing if the grouped argument breaks would also return `true`
        // if any content of the function body breaks. But, as far as this is concerned, it's only interested if
        // any content in the signature breaks.
        if matches!(result, Err(FormatError::PoorLayout)) {
            drop(buffer);
            // f.restore_state_snapshot(snapshot);

            let mut most_expanded_iter = most_expanded.into_iter();
            // Skip over the Start/EndEntry items.
            most_expanded_iter.next_back();

            // `skip(1)` is skipping the `StartEntry` tag
            return f.write_elements(most_expanded_iter.skip(1));
        }

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec().into_boxed_slice()
    };

    // Write the second variant that forces the group of the first/last argument to expand.
    let middle_variant = {
        let mut buffer = VecBuffer::new(f.state_mut());

        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        write!(
            buffer,
            [
                l_paren_token,
                format_once(|f| {
                    let mut joiner = f.join_with(soft_line_break_or_space());

                    for (i, (element, _)) in grouped.into_iter().enumerate() {
                        if (group_layout.is_grouped_first() && i == 0)
                            || (group_layout.is_grouped_last() && i == last_index)
                        {
                            joiner.entry(
                                &group(&format_once(|f| {
                                    if let Some(arg_element) = element? {
                                        f.write_element(arg_element)
                                    } else {
                                        Ok(())
                                    }
                                }))
                                .should_expand(true),
                            );
                        } else {
                            joiner.entry(&format_once(|f| {
                                if let Some(arg_element) = element? {
                                    f.write_element(arg_element)
                                } else {
                                    Ok(())
                                }
                            }));
                        }
                    }

                    joiner.finish()
                }),
                r_paren_token
            ]
        )?;

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec().into_boxed_slice()
    };

    // If the grouped content breaks, then we can skip the most_flat variant,
    // since we already know that it won't be fitting on a single line.
    let variants = if grouped_breaks {
        write!(f, [expand_parent()])?;
        vec![middle_variant, most_expanded.into_boxed_slice()]
    } else {
        vec![most_flat, middle_variant, most_expanded.into_boxed_slice()]
    };

    // SAFETY: Safe because variants is guaranteed to contain exactly 3 entries:
    // * most flat
    // * middle
    // * most expanded
    // ... and best fitting only requires the most flat/and expanded.
    unsafe {
        f.write_element(FormatElement::BestFitting(
            format_element::BestFittingElement::from_vec_unchecked(variants),
        ))
    }
}

/// Helper for formatting the first grouped argument (see [should_group_first_argument]).
struct FormatGroupedFirstArgument<'a, 'b> {
    argument: &'b AstNode<'a, Argument<'a>>,
}

impl<'a> Format<'a> for FormatGroupedFirstArgument<'a, '_> {
    fn fmt(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        match self.argument.as_ast_nodes() {
            // Call the arrow function formatting but explicitly passes the call argument layout down
            // so that the arrow function formatting removes any soft line breaks between parameters and the return type.
            AstNodes::ArrowFunctionExpression(arrow) => with_token_tracking_disabled(f, |f| {
                arrow.fmt_with_options(
                    FormatJsArrowFunctionExpressionOptions {
                        cache_mode: FunctionBodyCacheMode::Cache,
                        call_arg_layout: Some(GroupedCallArgumentLayout::GroupedFirstArgument),
                        ..FormatJsArrowFunctionExpressionOptions::default()
                    },
                    f,
                )
            }),

            // For all other nodes, use the normal formatting (which already has been cached)
            _ => self.argument.fmt(f),
        }
    }
}

/// Helper for formatting the last grouped argument (see [should_group_last_argument]).
struct FormatGroupedLastArgument<'a, 'b> {
    /// The argument to format
    argument: &'b AstNode<'a, Argument<'a>>,
    /// Is this the only argument in the arguments list
    is_only: bool,
}

impl<'a> Format<'a> for FormatGroupedLastArgument<'a, '_> {
    fn fmt(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        // For function and arrow expressions, re-format the node and pass the argument that it is the
        // last grouped argument. This changes the formatting of parameters, type parameters, and return types
        // to remove any soft line breaks.
        match self.argument.as_ast_nodes() {
            AstNodes::Function(function)
                if !self.is_only || function_has_only_simple_parameters(&function.params) =>
            {
                with_token_tracking_disabled(f, |f| {
                    function.fmt_with_options(
                        FormatFunctionOptions {
                            cache_mode: FunctionBodyCacheMode::Cache,
                            call_argument_layout: Some(
                                GroupedCallArgumentLayout::GroupedLastArgument,
                            ),
                        },
                        f,
                    )
                })
            }

            AstNodes::ArrowFunctionExpression(arrow) => with_token_tracking_disabled(f, |f| {
                arrow.fmt_with_options(
                    FormatJsArrowFunctionExpressionOptions {
                        cache_mode: FunctionBodyCacheMode::Cache,
                        call_arg_layout: Some(GroupedCallArgumentLayout::GroupedLastArgument),
                        ..FormatJsArrowFunctionExpressionOptions::default()
                    },
                    f,
                )
            }),
            _ => self.argument.fmt(f),
        }
    }
}

/// Disable the token tracking because it is necessary to format function/arrow expressions slightly different.
fn with_token_tracking_disabled<'a, F: FnOnce(&mut Formatter<'_, 'a>) -> R, R>(
    f: &mut Formatter<'_, 'a>,
    callback: F,
) -> R {
    // let was_disabled = f.state().is_token_tracking_disabled();
    // f.state_mut().set_token_tracking_disabled(true);

    // f.state_mut().set_token_tracking_disabled(was_disabled);

    callback(f)
}

fn function_has_only_simple_parameters(params: &FormalParameters<'_>) -> bool {
    has_only_simple_parameters(params, false)
}

/// Tests if this is a call to commonjs [`require`](https://nodejs.org/api/modules.html#requireid)
/// or amd's [`define`](https://github.com/amdjs/amdjs-api/wiki/AMD#define-function-) function.
fn is_commonjs_or_amd_call(
    arguments: &[Argument<'_>],
    call: &AstNode<'_, CallExpression<'_>>,
) -> bool {
    let Expression::Identifier(ident) = &call.callee else {
        return false;
    };

    match ident.name.as_str() {
        "require" => {
            match arguments.len() {
                0 => false,
                // `require` can be called with any expression that resolves to a
                // string. This check is only an escape hatch to allow a complex
                // expression to break rather than group onto the previous line.
                //
                // EX: `require(path.join(__dirname, 'relative/path'))`
                // Without condition:
                //   require(path.join(
                //     __dirname,
                //     'relative/path'));
                // With condition:
                //   require(
                //     path.join(__dirname, 'relative/path')
                //   );
                1 => matches!(arguments.first(), Some(Argument::StringLiteral(_))),
                _ => true,
            }
        }
        "define" => {
            let in_statement = matches!(call.parent, AstNodes::ExpressionStatement(_));
            if in_statement {
                match arguments.len() {
                    1 => true,
                    2 => matches!(arguments.first(), Some(Argument::ArrayExpression(_))),
                    3 => {
                        let mut iter = arguments.iter();
                        let first = iter.next();
                        let second = iter.next();
                        matches!(
                            (first, second),
                            (Some(Argument::StringLiteral(_)), Some(Argument::ArrayExpression(_)))
                        )
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Returns `true` if `arguments` contains a single [multiline template literal argument that starts on its own ](is_multiline_template_starting_on_same_line).
fn is_multiline_template_only_args(arguments: &[Argument], source_text: &str) -> bool {
    if arguments.len() != 1 {
        return false;
    }

    match arguments.first().unwrap() {
        Argument::TemplateLiteral(template) => {
            is_multiline_template_starting_on_same_line(template.span.start, template, source_text)
        }
        Argument::TaggedTemplateExpression(template) => {
            is_multiline_template_starting_on_same_line(
                template.span.start,
                &template.quasi,
                source_text,
            )
        }
        _ => false,
    }
}

/// This function is used to check if the code is a hook-like code:
///
/// ```js
/// useMemo(() => {}, [])
/// ```
fn is_react_hook_with_deps_array(
    arguments: &AstNode<ArenaVec<Argument>>,
    comments: &Comments,
) -> bool {
    if arguments.len() > 3 || arguments.len() < 2 {
        return false;
    }

    let mut args = arguments.as_ref().iter();
    if arguments.len() == 3 {
        args.next();
    }

    match (args.next(), args.next()) {
        (
            Some(Argument::ArrowFunctionExpression(callback)),
            Some(Argument::ArrayExpression(deps)),
        ) => {
            if !callback.params.is_empty() {
                return false;
            }

            if callback.expression {
                return false;
            }

            // Is there a comment that isn't around the callback or deps?
            !comments.filter_comments_in_span(arguments.parent.span()).any(|comment| {
                !callback.span.contains_inclusive(comment.span)
                    && !deps.span.contains_inclusive(comment.span)
            })
        }
        _ => false,
    }
}
