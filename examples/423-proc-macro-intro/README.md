📖 **[View on hightechmind.io →](https://hightechmind.io/rust/423-proc-macro-intro)**

---

# 423: Procedural Macros Overview

**Difficulty:** 4  **Level:** Expert

Compile-time code transformation using Rust functions — the machinery behind `#[derive(Serialize)]`, `#[tokio::main]`, and `sql!()`.

## The Problem This Solves

`macro_rules!` is powerful but pattern-based — it can only rearrange tokens mechanically. For real code generation you need to *understand* the structure of Rust code: inspect struct fields, read attribute arguments, generate conditional implementations based on generic parameters. This requires operating on the AST, not just token patterns.

Procedural macros are Rust functions that run at compile time. They receive a `TokenStream` (the raw tokens of the annotated code) and return a `TokenStream` (the generated code to replace or augment it). The compiler plugs the result back in as if you'd written it by hand. `#[derive(Serialize)]` reads your struct's fields and generates a `Serialize` implementation. `#[tokio::main]` wraps your `async fn main` in the runtime setup boilerplate. `sql!("SELECT ...")` verifies SQL syntax at compile time.

There are three types: **derive macros** add trait implementations, **attribute macros** modify or replace items, and **function-like macros** look like `macro_name!(...)` but run with full AST access. Each lives in its own crate with `proc-macro = true` in `Cargo.toml`.

## The Intuition

A procedural macro is a Rust function that takes code as input and returns code as output — all at compile time, giving you a compiler plugin without needing compiler internals.

## How It Works in Rust

```rust
// In a proc-macro crate (Cargo.toml: [lib] proc-macro = true):
use proc_macro::TokenStream;

// --- DERIVE MACRO ---
// Usage: #[derive(Describe)]
#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    // input = struct/enum definition tokens
    // output = additional impl block tokens
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    quote::quote! {
        impl Describe for #name {
            fn describe(&self) -> String { stringify!(#name).to_string() }
        }
    }.into()
}

// --- ATTRIBUTE MACRO ---
// Usage: #[my_attr]
#[proc_macro_attribute]
pub fn my_attr(args: TokenStream, item: TokenStream) -> TokenStream {
    // args = attribute arguments, item = the annotated item
    // return transformed item
    item  // pass through unchanged in simplest case
}

// --- FUNCTION-LIKE MACRO ---
// Usage: my_macro!(...)
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // input = everything inside the parens
    input
}
```

1. Create a separate crate, add `[lib] proc-macro = true` to `Cargo.toml`.
2. Add `syn` (parse input) and `quote` (generate output) dependencies.
3. Write a function annotated with `#[proc_macro_derive]`, `#[proc_macro_attribute]`, or `#[proc_macro]`.
4. Use it from another crate by adding the proc-macro crate as a dependency.

## What This Unlocks

- **Derive macros**: Auto-implement traits for any struct/enum — `Serialize`, `Debug`, custom domain traits.
- **Attribute macros**: Transform functions (`#[tokio::main]`, `#[test]`, `#[route("/api")]`).
- **Function-like macros**: Compile-time DSLs — SQL validation, regex compilation, HTML templates.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time code generation | PPX (preprocessor extensions) | Procedural macros (`proc-macro = true`) |
| AST access | `Ppxlib.Ast` full AST | `syn` crate parses `TokenStream` to AST |
| Code generation | `Ppxlib.Ast_builder` | `quote!` macro generates `TokenStream` |
| Three macro types | PPX derivers, transformers, extensions | derive, attribute, function-like |
| Separate crate required | PPX is a separate library | Yes — proc macros must be in their own crate |
