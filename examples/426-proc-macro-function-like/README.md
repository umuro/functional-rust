📖 **[View on hightechmind.io →](https://hightechmind.io/rust/426-proc-macro-function-like)**

---

# 426: Function-like Proc Macros

**Difficulty:** 4  **Level:** Expert

Macros that look like function calls but run at compile time with full AST access — for DSLs, compile-time validation, and code generation from structured input.

## The Problem This Solves

`macro_rules!` works on token patterns but can't understand their meaning. `sql!("SELECT * FROM users WHERE id = ?")` needs to parse SQL, validate it against a schema, and generate type-safe query code. `html! { <div class="main">...</div> }` needs to parse HTML syntax and generate `VNode` constructor calls. These require semantic understanding of structured input, not just token shuffling.

Function-like proc macros look like `name!(...)` — identical syntax to declarative macros — but the implementation is a Rust function with full access to `syn`, `quote`, and anything else you can import. The entire content inside the parens arrives as a `TokenStream` for you to interpret however you need.

Real examples: `regex!("[a-z]+")` compiles the regex at compile time; `include_proto!("schema.proto")` generates Rust types from protobuf; `query!("SELECT id FROM users")` in `sqlx` validates SQL and generates typed structs. All of these are function-like proc macros.

## The Intuition

A function-like proc macro receives everything inside `macro!(...)` as a token stream and returns any Rust code — it's a compile-time code generator with full Rust as its implementation language.

## How It Works in Rust

```rust
// In proc-macro crate:
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, LitInt};

// Simple: generate a constant from a string literal
#[proc_macro]
pub fn make_greeting(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as LitStr);
    let greeting = format!("Hello, {}!", name.value());
    quote! { #greeting }.into()
}

// Usage: const MSG: &str = make_greeting!("World");
// Expands to: const MSG: &str = "Hello, World!";

// Custom syntax: parse key=value pairs
#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
    // Parse custom DSL: config!(host = "localhost", port = 8080)
    // Validate at compile time, generate Config struct construction
    // ...
    input  // simplified
}

// Compile-time regex compilation
#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let pattern = parse_macro_input!(input as LitStr);
    let pat = pattern.value();
    // Validate regex at compile time — build error if invalid!
    if let Err(e) = regex_syntax::Parser::new().parse(&pat) {
        return syn::Error::new(pattern.span(), e.to_string())
            .to_compile_error().into();
    }
    quote! { ::regex::Regex::new(#pat).unwrap() }.into()
}
```

1. `#[proc_macro]` on a public function in a proc-macro crate.
2. Function signature: `pub fn name(input: TokenStream) -> TokenStream`.
3. Parse `input` with `syn` — could be a literal, a custom syntax, or any token sequence.
4. Validate at compile time — emit `syn::Error::to_compile_error()` for user-friendly build errors.
5. Generate output with `quote!`.

## What This Unlocks

- **Compile-time DSLs**: SQL, HTML, regex, configuration formats — validate and generate at build time.
- **Type-safe interfaces**: Generate typed structs from schemas at compile time (protobuf, SQL, GraphQL).
- **Custom syntax**: Any token stream you can parse — create embedded mini-languages.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time DSL | Camlp4 / `[%extension_point]` via PPX | `#[proc_macro]` function-like macro |
| Custom syntax | Camlp4 grammar extensions | Parse `TokenStream` with `syn` |
| Compile-time validation | PPX with `Location.raise_errorf` | `syn::Error::new(span, msg).to_compile_error()` |
| Meta-programming entry point | PPX driver (`ppxlib.runner`) | `#[proc_macro]` — registered by crate type |
| Usage syntax | `[%myext ...]` | `myext!(...)` — indistinguishable from built-ins |
