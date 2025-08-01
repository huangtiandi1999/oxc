# Changelog

All notable changes to this package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0).

## [0.79.0] - 2025-07-30

### 🐛 Bug Fixes

- 3cdac4c isolated-declarations: Optional parameter property misses `undefined` type (#12579) (Dunqing)
- 4f9cb0b isolated-declarations: Crash when transforming a class with a parameter property in a private constructor (#12578) (Dunqing)


## [0.78.0] - 2025-07-24

### 🐛 Bug Fixes

- 9a9d78d isolated-declarations: Transformed parameter property has an incorrect type annotation (#12450) (Dunqing)
- 89da027 isolated-declarations: Incorrect error when exported type is locally shadowed by an unexported variable (#12466) (Dunqing)




## [0.77.1] - 2025-07-16

### 🚀 Features

- 9b14fbc ast: Add `ThisExpression` to `TSTypeName` (#12156) (Boshen)



## [0.76.0] - 2025-07-08

### 💥 BREAKING CHANGES

- 8b30a5b codegen: [**BREAKING**] Introduce `CommentOptions` (#12114) (Boshen)


## [0.75.1] - 2025-07-03

### 🐛 Bug Fixes

- 42976a8 isolated-declarations: Produce incorrect types when getter/setter with different types (#12046) (Dunqing)



## [0.74.0] - 2025-06-23

### 🐛 Bug Fixes

- 957a3d5 isolated-declarations: Missing parameter properties of `constructor` overload (#11852) (Dunqing)
- b8b3530 isolated-declarations: Don't transform constructor params if they don’t have accessbility (#11842) (Dunqing)



## [0.73.1] - 2025-06-17

### 🚜 Refactor

- acc1b22 isolated-declarations: Shorten Span construction (#11688) (Ulrich Stark)



# Changelog

All notable changes to this package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project does not adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) until v1.0.0.

## [0.72.3] - 2025-06-06

### Performance

- 25167f2 parser: Parse ts type signature without rewind (#11443) (Boshen)

## [0.72.2] - 2025-05-31

### Bug Fixes

- e93caa4 isolated-declarations: Omit empty variable declarations (#11372) (magic-akari)
- daaa8f5 parser: Correctly parse decorators of property declaration (#11370) (magic-akari)

### Testing

- 03e8aa9 isolated-declarations: Fix multiline comment mismatch in windows (#11373) (Dunqing)

## [0.72.1] - 2025-05-28

### Bug Fixes

- 3249ab6 isolated-declarations: Correctly emit string property keys for non-identifiable name (#11329) (magic-akari)

## [0.72.0] - 2025-05-24

### Features

- 03390ad allocator: `TakeIn` trait with `AllocatorAccessor` (#11201) (Boshen)
- 123e63c isolated-declarations: Report an error for object methods whose return type cannot be inferred (#11230) (Dunqing)

### Bug Fixes

- 7126adb isolated-declarations: Type of class setter/getter cannot be inferred when the key is a global `Symbol.xxx` expression (#11236) (Dunqing)
- ac9638e isolated-declarations: Object property key was generated incorrectly (#11231) (Dunqing)
- 9ec8500 isolated-declarations: Incorrect type of object property accessor (#11229) (Dunqing)

## [0.71.0] - 2025-05-20

- 5d9344f rust: [**BREAKING**] Clippy avoid-breaking-exported-api = false (#11088) (Boshen)

### Refactor


## [0.70.0] - 2025-05-15

### Features

- 1673ffb codegen: Rework printing normal / legal / annotation comments (#10997) (Boshen)

### Bug Fixes

- 9a4b135 isolated-declarations: Lost leading comments of `export default` function/class/interface (#10990) (Dunqing)

## [0.69.0] - 2025-05-09

- 8a3bba8 ast: [**BREAKING**] Fix field order for `PropertyDefinition` (#10902) (overlookmotel)

### Bug Fixes

- 2c09243 ast: Fix field order for `AccessorProperty` (#10878) (overlookmotel)

## [0.68.0] - 2025-05-03

- a0a37e0 ast: [**BREAKING**] `AstBuilder` methods require an `Atom` with correct lifetime (#10735) (overlookmotel)

- 315143a codegen: [**BREAKING**] Remove useless `CodeGenerator` type alias (#10702) (Boshen)

### Performance

- 44f1952 isolated_declaration: Avoid copying string data (#10732) (overlookmotel)

### Refactor

- 40896d4 isolated_declarations: Shorten code (#10774) (overlookmotel)

## [0.66.0] - 2025-04-23

### Bug Fixes

- 7284f16 isolated-declarations: Leading comments of `ExportDefaultDeclaration` and `TSExportAssignment` appear in incorrect places (#10559) (Dunqing)
- 4c316a5 isolated_declarations: Fix broken snapshot files (#10561) (Boshen)

## [0.64.0] - 2025-04-17

- 521de23 ast: [**BREAKING**] Add `computed` property to `TSEnumMember` and `TSEnumMemberName::TemplateString` (#10092) (Yuji Sugiura)

- 49732ff ast: [**BREAKING**] Re-introduce `TSEnumBody` AST node (#10284) (Yuji Sugiura)

### Features

- 4c246fb ast: Add `override` field in `AccessorProperty` (#10415) (Yuji Sugiura)

### Refactor

- 6e6c777 ast: Add `TSEnumMemberName` variant to replace `computed` field (#10346) (Yuji Sugiura)

## [0.63.0] - 2025-04-08

### Bug Fixes

- d691701 various: Unwrap `Result` of `write!` macro (#10228) (overlookmotel)

### Refactor

- ca8f174 codegen: Do not print useless comma for TSEnumMember (#10213) (Yuji Sugiura)
- bcdbd38 transformer, minifier: Replace `AstBuilder::move_xxxx` methods with `TakeIn` trait (#10170) (Dunqing)

### Styling

- 66a0001 all: Remove unnecessary semi-colons (#10198) (overlookmotel)

## [0.62.0] - 2025-04-01

### Refactor

- 4020567 isolated_declarations: Use `FormalParameter::has_modifier` to detect parameter properties (#10098) (Ulrich Stark 🦀)

## [0.61.0] - 2025-03-20

### Refactor

- b2f3d23 isolated_declarations: Remove unused `self` params (#9868) (overlookmotel)

## [0.59.0] - 2025-03-18

- ce6808a parser: [**BREAKING**] Rename `type_parameters` to `type_arguments` where needed  (#9815) (hi-ogawa)

### Bug Fixes


## [0.58.0] - 2025-03-13

### Refactor

- a61a50b isolated_declarations: Do not store temp values in arena (#9733) (overlookmotel)

## [0.56.0] - 2025-03-06

- 48a0394 ast: [**BREAKING**] Add `scope_id` to `TSFunctionType` (#9559) (camc314)

### Features


## [0.54.0] - 2025-03-04

- a5cde10 visit_ast: [**BREAKING**] Add `oxc_visit_ast` crate (#9428) (Boshen)

### Features


### Performance

- 27a8e50 isolated_declarations: Reserve sufficient capacity in statements `Vec` (#9394) (overlookmotel)

### Refactor

- c880481 isolated_declarations: Rename var (#9390) (overlookmotel)
- 37e41f0 isolated_declarations: Use aliases `ArenaBox` / `ArenaVec` (#9389) (overlookmotel)

## [0.53.0] - 2025-02-26

### Refactor

- 7427900 ast: Re-order `ExportDefaultDeclaration` fields (#9348) (overlookmotel)

## [0.52.0] - 2025-02-21

### Bug Fixes

- 9e3571f isolated-declarations: Private accessors keep their types (#9132) (CPunisher)

### Refactor

- ef856f5 oxc: Apply `clippy::needless_pass_by_ref_mut` (#9253) (Boshen)

## [0.49.0] - 2025-02-10

- bbb075d ast: [**BREAKING**] Name `AstBuilder` enum builders after variant name not type name (#8890) (overlookmotel)

### Refactor


### Styling

- a4a8e7d all: Replace `#[allow]` with `#[expect]` (#8930) (overlookmotel)

## [0.48.2] - 2025-02-02

### Bug Fixes

- 0928a19 codegen: Emit this parameters of class methods (#8834) (michaelm)

### Documentation

- 57b7ca8 ast: Add documentation for all remaining JS AST methods (#8820) (Cam McHenry)

## [0.48.0] - 2025-01-24

### Features

- 99607d3 codegen: Print comments in `TSTypeLiteral` (#8679) (Boshen)

### Refactor

- a3dc4c3 crates: Clean up snapshot files (#8680) (Boshen)
- e66da9f isolated_declarations, linter, minifier, prettier, semantic, transformer: Remove unnecessary `ref` / `ref mut` syntax (#8643) (overlookmotel)
- b8d9a51 span: Deal only in owned `Atom`s (#8641) (overlookmotel)
- ac4f98e span: Derive `Copy` on `Atom` (#8596) (branchseer)

### Testing

- 39dbd2d codegen: Fix snapshot file (#8685) (Boshen)

## [0.46.0] - 2025-01-14

### Bug Fixes

- 4071878 isolated-declarations: Retain `declare` declarations when they are exported (#8477) (Dunqing)
- 7ee7634 isolated-declarations: Import statement disappears when import binding is referenced in nested `typeof` (#8476) (Dunqing)
- 7252cb0 isolated-declarations: Unexpected error when global `Symbol` as property key (#8475) (Dunqing)

## [0.45.0] - 2025-01-11

### Refactor

- aea9551 ast: Simplify `get_identifier_reference` of `TSType` and `TSTypeName` (#8273) (Dunqing)

## [0.42.0] - 2024-12-18

### Refactor

- 3858221 global: Sort imports (#7883) (overlookmotel)

### Styling

- 7fb9d47 rust: `cargo +nightly fmt` (#7877) (Boshen)

## [0.41.0] - 2024-12-13

- fb325dc ast: [**BREAKING**] `span` field must be the first element (#7821) (Boshen)

### Refactor


## [0.40.0] - 2024-12-10

- 72eab6c parser: [**BREAKING**] Stage 3 `import source` and `import defer` (#7706) (Boshen)

- ebc80f6 ast: [**BREAKING**] Change 'raw' from &str to Option<Atom> (#7547) (Song Gao)

### Features


### Refactor

- 3c1b2bf isolated_declarations: Use `NONE` in AST builder calls (#7752) (overlookmotel)

## [0.39.0] - 2024-12-04

- b0e1c03 ast: [**BREAKING**] Add `StringLiteral::raw` field (#7393) (Boshen)

### Features


## [0.38.0] - 2024-11-26

### Refactor

- 0ff94fa isolated-declarations: Use `CloneIn` instead of `ast.copy` (#7459) (Dunqing)

## [0.37.0] - 2024-11-21

- 44375a5 ast: [**BREAKING**] Rename `TSEnumMemberName` enum variants (#7250) (overlookmotel)

### Features

- 39afb48 allocator: Introduce `Vec::from_array_in` (#7331) (overlookmotel)
- 82773cb codegen: Remove underscore from bigint (#7367) (Boshen)

### Refactor

- 1938a1d isolated_declarations: Do not copy `Vec` unnecessarily (#7332) (overlookmotel)

## [0.36.0] - 2024-11-09

- 0e4adc1 ast: [**BREAKING**] Remove invalid expressions from `TSEnumMemberName` (#7219) (Boshen)

- d1d1874 ast: [**BREAKING**] Change `comment.span` to real position that contain `//` and `/*` (#7154) (Boshen)

### Features

- b74686c isolated-declarations: Support transform TSExportAssignment declaration (#7204) (Dunqing)

## [0.35.0] - 2024-11-04

### Features

- 6d97af4 rust: Use `oxc-miette` (#6938) (Boshen)

### Refactor

- cea0e6b isolated_declarations: Do not use `AstBuilder::*_from_*` methods (#7071) (overlookmotel)

## [0.34.0] - 2024-10-26

### Refactor

- 423d54c rust: Remove the annoying `clippy::wildcard_imports` (#6860) (Boshen)

## [0.33.0] - 2024-10-24

### Refactor

- 2e2b748 isolated-declarations: Protect internal transform methods (#6723) (DonIsaac)

## [0.32.0] - 2024-10-19

- 7645e5c codegen: [**BREAKING**] Remove CommentOptions API (#6451) (Boshen)

- 5200960 oxc: [**BREAKING**] Remove passing `Trivias` around (#6446) (Boshen)

- 2808973 ast: [**BREAKING**] Add `Program::comments` (#6445) (Boshen)

### Features

- 15dfc1d isolated-declarations: Impl `Default` for options (#6372) (DonIsaac)

### Bug Fixes

- 2673397 isolated_declarations: Fix potential memory leak (#6622) (overlookmotel)

### Refactor

- 073b02a ast: Type params field before params in TS function declaration types (#6391) (overlookmotel)
- 856cab5 ecmascript: Move ToInt32 from `oxc_syntax` to `oxc_ecmascript` (#6471) (Boshen)
- a504f96 isolated-declarations: Mark return struct as non exhaustive (#6374) (DonIsaac)

## [0.31.0] - 2024-10-08

- 020bb80 codegen: [**BREAKING**] Change to `CodegenReturn::code` and `CodegenReturn::map` (#6310) (Boshen)

### Features

- 9e62396 syntax_operations: Add crate `oxc_ecmascript` (#6202) (Boshen)

### Bug Fixes

- e9eeae0 isolated-declarations: False positive for function with a type asserted parameters (#6181) (Dunqing)

### Refactor


## [0.30.3] - 2024-09-27

### Bug Fixes

- a8338dd isolated-declarations: Accidentally collected references of original ast (#6102) (Dunqing)

## [0.30.2] - 2024-09-27

### Bug Fixes

- 418ae25 isolated-declarations: Report uninferrable types in arrays (#6084) (michaelm)
- c8682e9 semantic,codegen,transformer: Handle definite `!` operator in variable declarator (#6019) (Boshen)

### Performance

- 6b7d3ed isolated-declarations: Should clone transformed AST rather than original AST (#6078) (Dunqing)

## [0.30.1] - 2024-09-24

### Bug Fixes

- 9ca202a codegen: Preserve newlines between comments (#6014) (Boshen)
- 97a2c41 isolated-declarations: False positive for class private getter with non-inferrable return type (#5987) (michaelm)

## [0.30.0] - 2024-09-23

### Features

- 4a62703 isolated-declarations: Handle `export` in the `namespace` correctly (#5950) (Dunqing)
- 84a5816 isolated_declarations: Add `stripInternal` (#5878) (Boshen)
- dfbde2c isolated_declarations: Print jsdoc comments (#5858) (Boshen)

### Bug Fixes

- 5901d2a codegen: Various spacing issues (#5820) (Boshen)
- fd1c46c isolated-declarations: Infer failed if there are two setter/getter methods that need to be inferred (#5967) (Dunqing)
- 6df82ee isolated-declarations: False positive for class private method that has arguments without type annotations (#5964) (Dunqing)
- 6a9e71d isolated-declarations: Wrap TSFunctionType in parentheses if it is inside the `TSUnionType` (#5963) (Dunqing)
- ea32d5b isolated-declarations: Should print constructor assignments first (#5934) (Dunqing)
- 0f96b59 isolated-declarations: Missing print comments in class's private method (#5931) (Dunqing)
- 8780c54 isolated-declarations: Do not union a undefined when the param type is any or unknown (#5930) (Dunqing)
- f07ff14 isolated-declarations: Should not transform signature that has type annotation (#5927) (Dunqing)
- b6a9178 isolated-declarations: Don't collect references when `ExportNamedDeclaration` has source (#5926) (Dunqing)
- 756a571 isolated-declarations: Missing empty export when has an export declare (#5925) (Dunqing)
- e148c80 isolated_declarations: Try fix fixtures (Boshen)
- 9b3f763 isolated_declarations: Try fix new line issue (Boshen)
- ee748b0 isolated_declarations: Fix fixture spacing (Boshen)

### Performance

- cd34f07 isolated-declarations: Combine type/value bindings and type/value references into one (#5968) (Dunqing)

### Refactor

- c84bd28 isolated-declarations: Simplify to infer the getter and setter methods (#5966) (Dunqing)
- 67b4220 isolated-declarations: Simplify handling VariableDeclaration transform (#5916) (Dunqing)
- 2fd5c2a isolated-declarations: Pre-filter statements that do not need to be transformed (#5909) (Dunqing)
- 1c1353b transformer: Use AstBuilder instead of using struct constructor (#5778) (Boshen)

### Testing

- d6cbbe7 isolated-declarations: Arrow function unions in return signature (#5973) (DonIsaac)

## [0.29.0] - 2024-09-13

### Features

- 953fe17 ast: Provide `NONE` type for AST builder calls (#5737) (overlookmotel)

## [0.28.0] - 2024-09-11

- ee4fb42 ast: [**BREAKING**] Reduce size of `WithClause` by `Box`ing it (#5677) (Boshen)

- 4a8aec1 span: [**BREAKING**] Change `SourceType::js` to `SourceType::cjs` and `SourceType::mjs` (#5606) (Boshen)

### Features


### Bug Fixes

- b9bf544 isolated-declarations: False positive for setter method in `interface` (#5681) (Dunqing)
- 6e8409a isolated-declarations: Bindings referenced in `TSModuleDeclaration` are removed incorrectly (#5680) (Dunqing)

### Performance


### Testing
- dc92489 Add trailing line breaks to conformance fixtures (#5541) (overlookmotel)

## [0.26.0] - 2024-09-03

- 234a24c ast: [**BREAKING**] Merge `UsingDeclaration` into `VariableDeclaration` (#5270) (Kevin Deng 三咲智子)

### Features

- 180b1a1 ast: Add `Function::name()` (#5361) (DonIsaac)
- 5505749 ast: Add `accessibility` field to `AccessorProperty` (#5290) (Dunqing)
- 49cd5db ast,parser: Add `definite` flag to `AccessorProperty` node (#5182) (DonIsaac)
- c2fa725 ast,parser: Parse `TSTypeAnnotations` on `AccessorProperty` (#5179) (DonIsaac)

### Bug Fixes


### Refactor

- 946c867 ast: Box `TSThisParameter` (#5325) (overlookmotel)

## [0.25.0] - 2024-08-23

### Bug Fixes

- 185eb20 isolated_declarations: Namespaces that are default exported should be considered for expando functions (#4935) (michaelm)

## [0.24.3] - 2024-08-18

### Bug Fixes

- d3bbc62 isolated-declarations: Declare modifier of PropertyDefinition should not be retained (#4941) (Dunqing)
- 8e80f59 isolated_declarations: Class properties should still be lifted from private constructors (#4934) (michaelm)
- b3ec9e5 isolated_declarations: Always emit module declarations that perform augmentation (#4919) (michaelm)
- 0fb0b71 isolated_declarations: Always emit module declarations (#4911) (michaelm)
- 4a16916 isolated_declarations: Support expando functions (#4910) (michaelm)

### Refactor

- 1eb59d2 ast, isolated_declarations, transformer: Mark `AstBuilder::copy` as an unsafe function (#4907) (overlookmotel)

## [0.24.0] - 2024-08-08

### Features

- e12bd1e ast: Allow conversion from TSAccessibility into &'static str (#4711) (DonIsaac)

### Refactor

- 475266d ast: Use correct lifetimes for name-related methods (#4712) (DonIsaac)

## [0.23.0] - 2024-08-01

### Bug Fixes

- d5c4b19 parser: Fix enum member parsing (#4543) (DonIsaac)

## [0.22.0] - 2024-07-23

### Bug Fixes

- aece1df ast: Visit `Program`s `hashbang` field first (#4368) (overlookmotel)
- 3d88f20 codegen: Print shorthand for all `{ x }` variants (#4374) (Boshen)

### Performance
- a207923 Replace some CompactStr usages with Cows (#4377) (DonIsaac)

### Refactor

- 0e1ea90 isolated-declarations: Remove useless code from scope (#4420) (Dunqing)

## [0.21.0] - 2024-07-18

### Features

- 83c2c62 codegen: Add option for choosing quotes; remove slow `choose_quot` method (#4219) (Boshen)
- 20cdb1f semantic: Align class scope with typescript (#4195) (Dunqing)

### Bug Fixes

- 3df9e69 mangler: No shorthand `BindingProperty`; handle var hoisting and export variables (#4319) (Boshen)

### Refactor

- 2c7bb9f ast: Pass final `ScopeFlags` into `visit_function` (#4283) (overlookmotel)
- ace4f1f semantic: Update the order of `visit_function` and `Visit` fields in the builder to be consistent (#4248) (Dunqing)

## [0.20.0] - 2024-07-11

### Features

- 67fe75e ast, ast_codegen: Pass the `scope_id` to the `enter_scope` event. (#4168) (rzvxa)

### Bug Fixes

- 48947a2 ast: Put `decorators` before everything else. (#4143) (rzvxa)

## [0.19.0] - 2024-07-09

- b936162 ast/ast_builder: [**BREAKING**] Shorter allocator utility method names. (#4122) (rzvxa)

### Refactor


## [0.18.0] - 2024-07-09

- d347aed ast: [**BREAKING**] Generate `ast_builder.rs`. (#3890) (rzvxa)

### Features


### Bug Fixes

- cb1af04 isolated-declarations: Remove the `async` and `generator` keywords from `MethodDefinition` (#4130) (Dunqing)

## [0.17.2] - 2024-07-08

### Bug Fixes

- 5c31236 isolated-declarations: Keep literal value for readonly property (#4106) (Dunqing)
- e67c7d1 isolated-declarations: Do not infer type for private parameters (#4105) (Dunqing)
- 3fcad5e isolated_declarations: Remove nested AssignmentPatterns from inside parameters (#4077) (michaelm)
- f8d77e4 isolated_declarations: Infer type of template literal expressions as string (#4068) (michaelm)

### Performance

- 7ed27b7 isolated-declarations: Use `FxHashSet` instead of `Vec` to speed up the `contain` (#4074) (Dunqing)

## [0.17.1] - 2024-07-06

### Bug Fixes

- adee728 isolated_declarations: Don't report an error for parameters if they are ObjectPattern or ArrayPattern with an explicit type (#4065) (michaelm)
- 1b8f208 isolated_declarations: Correct emit for private static methods (#4064) (michaelm)

### Refactor

- 65aee19 isolated-declarations: Reorganize scope tree (#4070) (Luca Bruno)

## [0.17.0] - 2024-07-05

- c98d8aa ast: [**BREAKING**] Rename `visit_arrow_expression` to `visit_arrow_function_expression`. (#3995) (rzvxa)

### Features

- 7768d23 isolated-declarations: Support optional class methods (#4035) (Egor Blinov)

### Bug Fixes

- 3d29e9c isolated-declarations: Eliminate imports incorrectly when they are used in `TSInferType` (#4043) (Dunqing)
- 02ea19a isolated-declarations: Should emit `export {}` when only having `ImportDeclaration` (#4026) (Dunqing)
- 7c915f4 isolated-declarations: Binding elements with export should report an error (#4025) (Dunqing)
- 05a047c isolated-declarations: Method following an abstract method gets dropped (#4024) (Dunqing)
- c043bec isolated_declarations: Add mapped-type constraint to the scope (#4037) (Egor Blinov)
- b007553 isolated_declarations: Fix readonly specifier on class constructor params (#4030) (Egor Blinov)
- da62839 isolated_declarations: Inferring literal types for readonly class fileds (#4027) (Egor Blinov)

### Refactor


## [0.16.2] - 2024-06-30

### Features

- dc6d45e ast,codegen: Add `TSParenthesizedType` and print type parentheses correctly (#3979) (Boshen)

### Bug Fixes

- bd1141d isolated-declarations: If declarations is referenced in `declare global` then keep it (#3982) (Dunqing)

## [0.16.1] - 2024-06-29

### Bug Fixes

- 51e54f9 codegen: Should print `TSModuleDeclarationKind` instead of just `module` (#3957) (Dunqing)
- 31e4c3b isolated-declarations: `declare global {}` should be kept even if it is not exported (#3956) (Dunqing)

## [0.16.0] - 2024-06-26

- 6796891 ast: [**BREAKING**] Rename all instances of `BigintLiteral` to `BigIntLiteral`. (#3898) (rzvxa)

- 1f85f1a ast: [**BREAKING**] Revert adding `span` field to the `BindingPattern` type. (#3899) (rzvxa)

- ae09a97 ast: [**BREAKING**] Remove `Modifiers` from ts nodes (#3846) (Boshen)

- 1af5ed3 ast: [**BREAKING**] Replace `Modifiers` with `declare` and `const` on `EnumDeclaration` (#3845) (Boshen)

- 0673677 ast: [**BREAKING**] Replace `Modifiers` with `declare` on `Function` (#3844) (Boshen)

- ee6ec4e ast: [**BREAKING**] Replace `Modifiers` with `declare` and `abstract` on `Class` (#3841) (Boshen)

- 9b38119 ast: [**BREAKING**] Replace `Modifiers` with `declare` on `VariableDeclaration` (#3839) (Boshen)

- cfcef24 ast: [**BREAKING**] Add `directives` field to `TSModuleBlock` (#3830) (Boshen)

- 4456034 ast: [**BREAKING**] Add `IdentifierReference` to `ExportSpecifier` (#3820) (Boshen)

### Features

- 497769c ast: Add some visitor functions (#3785) (Dunqing)
- 2821e0e codegen: Print readonly keyword for TSIndexSignature (#3791) (Dunqing)
- 97575d8 codegen: Print TSClassImplements and TSThisParameter (#3786) (Dunqing)
- 5e2baf3 isolated-declarations: Report error for expando functions (#3872) (Dunqing)
- 2cdb34f isolated-declarations: Support for class function overloads (#3811) (Dunqing)
- 231b8f0 isolated-declarations: Support for export default function overloads (#3809) (Dunqing)
- a37138f isolated-declarations: Improve the inference template literal (#3797) (Dunqing)
- b0d7355 isolated-declarations: Transform const expression correctly (#3793) (Dunqing)
- b38c34d isolated-declarations: Support inferring ParenthesizedExpression (#3769) (Dunqing)
- 4134de8 isolated-declarations: Add ts error code to the error message (#3755) (Dunqing)
- 94202de isolated-declarations: Add `export {}` when needed (#3754) (Dunqing)
- e95d8e3 isolated-declarations: Shrink span for arrow function that needs an explicit return type (#3752) (Dunqing)
- df9971d isolated-declarations: Improve inferring the return type from function (#3750) (Dunqing)
- 4aea2b1 isolated-declarations: Improve inferring the type of accessor (#3749) (Dunqing)
- 9ea30c4 isolated-declarations: Treat AssignmentPattern as optional (#3748) (Dunqing)

### Bug Fixes

- 2766594 codegen: Print type parameters for MethodDefinition (#3922) (Dunqing)
- 27f0531 isolated-declarations: Private constructor reaching unreachable (#3921) (Dunqing)
- 59ce38b isolated-declarations: Inferring of UnrayExpression incorrectly (#3920) (Dunqing)
- 58e54f4 isolated-declarations: Report an error for parameters if they are  ObjectPattern or ArrayPattern without an explicit type (#3810) (Dunqing)
- cb8a272 isolated-declarations: Cannot infer nested `as const` (#3807) (Dunqing)
- d8ecce5 isolated-declarations: Infer BigInt number as `bigint` type (#3806) (Dunqing)
- 4e241fc isolated-declarations: Missing `const` after transformed const enum (#3805) (Dunqing)
- 683c7b0 isolated-declarations: Shouldn’t add declare in declaration with export default (#3804) (Dunqing)
- 7d47fc3 isolated-declarations: Should stripe async and generator keyword after transformed (#3790) (Dunqing)
- 8ce794d isolated-declarations: Inferring an incorrect return type when there is an arrow function inside a function (#3768) (Dunqing)
- d29316a isolated-declarations: Transform incorrectly when there are multiple functions with the same name (#3753) (Dunqing)
- bf1c250 isolated-declarations: False positives for non-exported binding elements (#3751) (Dunqing)

### Performance
- 4f7ff7e Do not pass `&Atom` to functions (#3818) (overlookmotel)

### Refactor

- 363d3d5 ast: Add span field to the `BindingPattern` type. (#3855) (rzvxa)
- 2f5d50e isolated-declarations: Remove `Modifiers` (#3847) (Boshen)

## [0.15.0] - 2024-06-18

- 5c38a0f codegen: [**BREAKING**] New code gen API (#3740) (Boshen)

### Features

- ee627c3 isolated-declarations: Create unique name for `_default` (#3730) (Dunqing)
- 81e9526 isolated-declarations: Inferring set accessor parameter type from get accessor return type (#3725) (Dunqing)
- 77d5533 isolated-declarations: Report errors that are consistent with typescript. (#3720) (Dunqing)
- 0b8098a napi: Isolated-declaration (#3718) (Boshen)

### Bug Fixes

- f1b793f isolated-declarations: Function overloads reaching unreachable (#3739) (Dunqing)
- 0fbecdc isolated-declarations: Should be added to references, not bindings (#3726) (Dunqing)

### Refactor

- 3c59735 isolated-declarations: Remove `TransformDtsCtx` (#3719) (Boshen)
- 815260e isolated-declarations: Decouple codegen (#3715) (Boshen)

