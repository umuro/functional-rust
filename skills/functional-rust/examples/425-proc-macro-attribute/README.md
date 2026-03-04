# 425: Attribute Macros #[my_attr]

**Difficulty:** 4  **Level:** Expert

Transform or replace entire items at compile time — the mechanism behind `#[tokio::main]`, `#[test]`, and `#[route("/api")]`.

## The Problem This Solves

Derive macros add code alongside an item. Sometimes you need to *transform* the item itself — wrap a function body, modify a struct's fields, add setup/teardown around a function. Attribute macros receive the full item and return whatever code should replace it, giving you complete control.

`#[tokio::main]` takes your `async fn main()` and wraps it in `tokio::runtime::Builder::new_multi_thread().build().unwrap().block_on(async { ... })`. `#[instrument]` (from `tracing`) wraps function bodies with span setup and teardown. `#[route("/api/users", method = "GET")]` registers handler functions with a web framework. All of these are attribute macros — they receive the function and return a transformed version.

Unlike derive macros (which only add items), attribute macros replace the annotated item entirely. If you return the input unchanged, you're a no-op. If you return something different, the original code is gone and your output is what the compiler sees.

## The Intuition

An attribute macro receives an item (function, struct, mod) and returns code to replace it — full transformation power, not just addition.

## How It Works in Rust

```rust
// In proc-macro crate:
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// Simple: wrap function with logging
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let body = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;

    quote! {
        #vis #sig {
            println!("[LOG] Entering {}", stringify!(#name));
            let result = (|| #body)();
            println!("[LOG] Exiting {}", stringify!(#name));
            result
        }
    }.into()
}

// Usage:
#[log_call]
fn expensive_computation(x: i32) -> i32 {
    x * x + 1
}
// Compiles to the logged version — original body still runs,
// wrapped with println! before and after.

// Reading attribute arguments
#[proc_macro_attribute]
pub fn retry(args: TokenStream, item: TokenStream) -> TokenStream {
    let retries: usize = args.to_string().parse().unwrap_or(3);
    let input = parse_macro_input!(item as ItemFn);
    // ... generate retry loop wrapping the function body
}

// Usage: #[retry(5)] fn flaky_operation() { ... }
```

1. `#[proc_macro_attribute]` — two arguments: `args` (inside the `#[attr(args)]`) and `item` (the annotated item).
2. Parse `item` as `syn::ItemFn`, `syn::ItemStruct`, etc. depending on what you expect.
3. Deconstruct the item, transform, and `quote!` the result.
4. Return the replacement `TokenStream` — the original item is replaced entirely.

## What This Unlocks

- **Function wrapping**: Add logging, timing, retry logic, or authorization checks around any function body.
- **Framework integration**: Web route registration, test harness setup, RPC method binding.
- **Conditional generation**: Inspect attribute arguments to change what code is generated.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Item transformation | PPX `[@@deriving ...]` / `[%extension]` | `#[proc_macro_attribute]` |
| Function wrapping | `[%log_call]` via custom PPX | `#[log_call]` attribute macro |
| Access to body | `Ppxlib` expression rewriting | `syn::ItemFn` — access `sig`, `block`, `vis` |
| Attribute arguments | `[@@attr arg]` syntax | `args: TokenStream` — parse as needed |
| Framework routing | Hand-written registration | `#[route("/path")]` attribute macro |
