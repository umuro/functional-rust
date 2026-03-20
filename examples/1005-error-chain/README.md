[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1005 — Error Chaining

## Problem Statement

Add context to errors as they propagate up the call stack. When a low-level `read_file` returns `Err(IoError::NotFound)`, the higher-level `load_config` wraps it in `AppError { context: "loading /path", source: e }`. Implement a `WithContext` extension trait for ergonomic chaining. Compare with OCaml's manual wrapping pattern.

## Learning Outcomes

- Use `.map_err(|e| AppError { context: "…", source: e })` to add context to errors
- Implement a `WithContext<T>` trait with `fn with_context(self, ctx: impl FnOnce() -> String)`
- Use a lazy closure `impl FnOnce() -> String` for context to avoid allocation on success paths
- Implement `std::error::Error::source` to expose the original error for chain inspection
- Map Rust's `map_err` to OCaml's `match … Error e -> Error { context; cause = e }`
- Recognise the `anyhow::Context` pattern as the production implementation of this idea

## Rust Application

`load_config` calls `read_file(path).map_err(|e| AppError { context: format!("loading {}", path), source: e })`. The `WithContext` trait adds `.with_context(|| format!("…"))` to any `Result<T, IoError>`. Using `impl FnOnce() -> String` instead of `String` avoids the format! call on the success path. `AppError::source()` can return the inner `IoError` (if `IoError` also implements `std::error::Error`) for full chain traversal.

## OCaml Approach

OCaml's `load_with_context path` matches on `read_file path`: on `Error e`, it returns `Error { context = …; cause = e }`. There is no lazy context — `Printf.sprintf` is always called. The `with_context` helper can be defined as `let with_context ctx = Result.map_error (fun cause -> { context = ctx; cause })`. This is the same pattern as Rust's `map_err`, just without trait integration.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Add context | `.map_err(\|e\| AppError { context, source: e })` | `Result.map_error (fun cause -> { context; cause })` |
| Lazy context | `impl FnOnce() -> String` | Manual `if error { sprintf … }` |
| Extension trait | `impl WithContext<T> for Result<T, IoError>` | Function `with_context` |
| Error chain | `Error::source()` → inner error | Manual `e.cause` field access |
| Production lib | `anyhow::Context` | `Result.error_to_exn` or custom |
| Verbosity | Medium | Low |

Error chaining is how production Rust code provides actionable error messages: "failed to start server: failed to read config: file not found: /etc/app.toml". Each layer adds context. The `anyhow` crate automates this pattern; understanding the manual version clarifies the underlying mechanics.

## Exercises

1. Add a `wrap_io_err(op: &str) -> impl FnOnce(IoError) -> AppError` factory function for reusable context strings.
2. Implement `Error::source` for `AppError` by making `IoError` also implement `std::error::Error`.
3. Write a `print_error_chain(e: &dyn Error)` function that traverses the `.source()` chain and prints each level.
4. Use the `anyhow` crate's `.context("…")` method and compare it with the manual `WithContext` trait.
5. In OCaml, implement a `chain_error` functor that adds context to any `('a, 'b) result` error type, parameterised over the error type.
