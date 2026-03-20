📖 **[View on hightechmind.io →](https://hightechmind.io/rust/423-proc-macro-intro)**

---

# 423: Procedural Macro Introduction
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

`macro_rules!` handles syntactic patterns but cannot inspect type information or generate identifiers dynamically based on field names. Procedural macros (proc macros) operate on the full Rust token stream at compile time as external Rust programs: they receive a `TokenStream`, parse it using `syn`, and emit generated code using `quote`. This enables `#[derive(Serialize)]` to generate different code for each struct's specific field names and types — impossible with `macro_rules!`.

Proc macros power the entire Rust ecosystem's most powerful abstractions: `serde`, `tokio::main`, `actix::web::get`, `clap::Parser`, and thousands of derive macros.

## Learning Outcomes

- Understand the three types of proc macros: derive, attribute, function-like
- Learn the proc macro development model: separate crate, `TokenStream` in/out
- See conceptually what a `#[derive(MyDebug)]` proc macro generates for `Point`
- Understand how `syn` parses token streams into AST nodes and `quote!` generates code
- Learn why proc macros must be in a separate crate with `proc-macro = true`

## Rust Application

In `src/lib.rs`, the code shows the output of what proc macros conceptually generate. `Point` with its `Debug` impl demonstrates what `#[derive(MyDebug)]` would produce: `f.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()`. `example_function` shows what a `#[log_calls]` attribute macro would transform — wrapping the body with enter/exit logging. These are the target outputs that real proc macros generate via `syn` parsing and `quote!` expansion.

## OCaml Approach

OCaml's PPX framework is the direct equivalent: a PPX plugin is a standalone OCaml program that receives the parsed OCaml AST (as `Parsetree` values), transforms it, and returns the modified AST. `ppx_deriving` is OCaml's `derive` equivalent. The AST is richer than Rust's token stream (already parsed) but requires knowledge of OCaml's `Parsetree` module. Dune integrates PPX as build-time preprocessors.

## Key Differences

1. **Token vs. AST**: Rust proc macros receive raw token streams; OCaml PPX receives the parsed AST. Rust is more flexible but requires manual parsing; OCaml's AST is structured but verbose.
2. **Separate crate**: Rust proc macros must live in a separate crate with `proc-macro = true`; OCaml PPX plugins are separate executables configured in dune.
3. **syn/quote ecosystem**: Rust uses `syn` for parsing and `quote!` for generation; OCaml uses `Ppxlib` for AST traversal and `Ast_builder` for generation.
4. **Error reporting**: Rust proc macros use `compile_error!` macro or `syn::Error::to_compile_error()`; OCaml uses `Location.error_extensionf` for positioned errors.

## Exercises

1. **Understand the output**: Take the `Point` struct and `Debug` impl in `src/lib.rs`. Trace through what `syn` would parse (struct name, field names, field types) and how `quote!` would produce the `impl Debug for Point` block.
2. **Attribute macro sketch**: Write a `// TODO: proc-macro` function that takes `fn add(a: i32, b: i32) -> i32` and describes in comments what the `#[log_calls]` attribute macro would need to generate — including the logging calls and the original function body.
3. **Research project**: Find three proc macros you use regularly (`tokio::main`, `serde::Deserialize`, `clap::Parser`). For each, describe in a code comment what token stream input they receive and what code they generate.
