📖 **[View on hightechmind.io →](https://hightechmind.io/rust/299-error-context)**

---

# 299: Adding Context to Errors

## Problem Statement

Bare error messages like "file not found" or "parse failed" are unhelpful without context about what was being attempted. Context wrapping adds layers of "where" and "why" information around errors: "while loading config: while reading /etc/app.conf: file not found". This is the `anyhow::context()` pattern — each operation wraps a lower-level error in a higher-level description, building an error chain that reads as a call-stack narrative.

## Learning Outcomes

- Implement a generic `Context<E>` wrapper that adds a message to any error
- Use `source()` to expose the wrapped error for chain traversal
- Understand the layered error context pattern as a stack of descriptive messages
- Build a helper function for ergonomic context addition without boilerplate at each call site

## Rust Application

A `Context<E>` struct wraps any error type with a descriptive message:

```rust
#[derive(Debug)]
pub struct Context<E> {
    pub message: String,
    pub source: E,
}

impl<E: Error + 'static> Error for Context<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.source) }
}

// Extension trait for ergonomic usage:
fn load_config(path: &str) -> Result<String, Context<io::Error>> {
    read_file(path).map_err(|e| Context {
        message: format!("while loading config from '{}'", path),
        source: e,
    })
}
```

## OCaml Approach

OCaml's `Result.map_error` can wrap errors with context strings, but there is no standard chaining mechanism. Libraries like `Error_monad` provide dedicated context operations:

```ocaml
let with_context msg = Result.map_error (fun e ->
  Printf.sprintf "%s: %s" msg (string_of_error e))
```

This flattens the chain into a single string rather than preserving the original error as a structured value.

## Key Differences

1. **Structured vs string**: Rust's `Context<E>` preserves the original error as a typed value accessible via `source()`; OCaml typically flattens context into a combined error string.
2. **Chain traversal**: Rust's `source()` chain enables iterating through all context layers programmatically; OCaml's string approach loses structure.
3. **Type precision**: `Context<ParseError>` preserves the exact error type; `Box<dyn Error>` in `anyhow` erases it for flexibility.
4. **Ecosystem**: `anyhow::Context` trait provides `.context("msg")` and `.with_context(|| msg)` as ergonomic extension methods — the idiomatic production approach.

## Exercises

1. Implement a `ResultExt` trait with `.context(msg)` and `.with_context(|| msg)` methods on any `Result<T, E: Error>`.
2. Build a three-level operation (read → parse → validate) where each level wraps errors in context messages, then traverse and print the full chain.
3. Compare the output of traversing a structured `Context<E>` chain with a flat string-concatenated approach for the same error scenario.
