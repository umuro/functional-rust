📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1016-error-context)**

---

# 1016-error-context — Error Context
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

When an error bubbles up through several layers of a call stack, the raw error message often lacks enough information to diagnose the problem. "file not found" tells you what failed but not why the file was being read or what operation was in progress. Adding context at each propagation point — "loading config -> reading file -> file not found" — produces error messages that pinpoint the root cause without a debugger.

The `anyhow` crate provides `.context("...")` and `.with_context(|| ...)` as an extension trait on `Result`. This example builds the same mechanism from scratch using a wrapper struct, so the mechanics are transparent.

## Learning Outcomes

- Design an `ErrorWithContext` struct that carries a message and a context chain
- Implement a `Context` extension trait for `Result<T, ErrorWithContext>`
- Understand the difference between eager `.context(str)` and lazy `.with_context(|| str)`
- Walk the context chain to produce a human-readable error display
- Understand how `anyhow::Context` generalises this to any error type

## Rust Application

`src/lib.rs` defines `ErrorWithContext` with a `Vec<String>` context stack. The `Context` trait adds `.context(str)` and `.with_context(f)` methods to `Result<T, ErrorWithContext>`. Each layer wraps the error by pushing a context string, and `Display` renders the chain as `"outer -> inner -> root cause"`. This mirrors how `anyhow` builds its context chain.

The pattern is ubiquitous in server applications: `tracing`, `anyhow`, and `snafu` all provide variants of this mechanism.

## OCaml Approach

OCaml's `Base.Error` type carries a lazy tree of error messages. The `error_s` and `tag` functions annotate errors with context:

```ocaml
let with_context label f =
  match f () with
  | Ok v -> Ok v
  | Error e -> Error (Error.tag e ~tag:label)
```

Libraries like `Core_kernel` provide `Or_error.tag` for exactly this pattern. Unlike Rust's struct approach, OCaml's `Error` is a lazy `Info.t` tree that is only rendered when displayed.

## Key Differences

1. **Lazy vs eager**: Rust's `with_context` takes a closure to avoid string formatting when no error occurs; OCaml's `Error.tag` is lazy by default via `Info.t`.
2. **Extension trait**: Rust's `Context` trait is implemented on `Result<T, E>` using a blanket impl; OCaml uses module functions.
3. **Chain direction**: Rust builds context by prepending to a `Vec` and reversing on display; OCaml's `Error` tree is structured differently but renders similarly.
4. **`anyhow` vs custom**: Production Rust uses `anyhow::Context` which works with any error type via `Box<dyn Error>`; OCaml's `Or_error` is the equivalent standard library choice.

## Exercises

1. Add a `context_if(predicate: bool, msg: &str)` combinator that only attaches context when the predicate is true.
2. Implement a `source()` chain walker that prints all context levels in a numbered list.
3. Refactor the example to use `anyhow::Result` and `anyhow::Context` from the `anyhow` crate and compare the implementation complexity.
