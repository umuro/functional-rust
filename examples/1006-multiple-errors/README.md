[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1006 — Multiple Error Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Handle functions that return different error types in the same call chain. Compare two approaches: `Box<dyn Error>` (flexible, type-erased) and a typed `AppError` enum (exhaustive, structured). Implement `From` conversions for the enum approach to enable `?` operator chaining. Compare with OCaml's unified variant and polymorphic variants.

## Learning Outcomes

- Use `Box<dyn std::error::Error>` as a universal error type that accepts any `Error` implementor
- Understand that `?` on `ParseIntError` in a `-> Result<T, Box<dyn Error>>` function auto-boxes via `From`
- Build a typed `AppError` enum with `From` impls for each sub-error type
- Compare the trade-offs: `Box<dyn Error>` (flexible/simple) vs enum (exhaustive/structured)
- Map Rust's approach to OCaml's unified variant enum and polymorphic variants
- Choose the right approach: `Box<dyn Error>` for applications, typed enum for libraries

## Rust Application

`Box<dyn std::error::Error>` accepts any error type that implements `Error` — the `?` operator boxes it via the blanket `impl From<E: Error> for Box<dyn Error>`. The typed `AppError` enum requires explicit `impl From<IoError> for AppError`, `impl From<ParseIntError> for AppError`, etc. Once `From` is implemented, `do_io_typed()?` in a `-> Result<T, AppError>` function automatically wraps the `IoError`. The enum approach enables exhaustive match on error variants; `Box<dyn Error>` does not.

## OCaml Approach

OCaml's standard approach is a unified `app_error` variant: `type app_error = Io of io_error | Parse of parse_error | Net of net_error`. Functions explicitly wrap errors: `Result.map_error (fun e -> Io e)`. Polymorphic variants (`[> \`FileNotFound \| \`ReadError of string]`) provide open, extensible error types without a central enum — more flexible but harder to reason about exhaustively.

## Key Differences

| Aspect | Rust `Box<dyn Error>` | Rust typed enum | OCaml unified variant | OCaml poly variants |
|--------|-----------------------|-----------------|----------------------|---------------------|
| Exhaustiveness | No | Yes | Yes | No |
| Conversion | Auto (blanket From) | Explicit `From` impls | Manual wrapping | Structural subtyping |
| Pattern match | Downcast needed | Direct match | Direct match | Flexible |
| Library use | Not recommended | Recommended | Recommended | Possible |
| Verbosity | Low | Medium | Low | Low |

The general rule: use typed enums for library crates (callers need to match on errors), use `Box<dyn Error>` or `anyhow::Error` for application code where errors are logged rather than matched.

## Exercises

1. Use `anyhow::anyhow!("message")` and `anyhow::Context::context` to rewrite `process_boxed` without defining any error types.
2. Add a fourth `AppError::Config(String)` variant and implement its `From` conversion.
3. Write `fn collect_errors(results: Vec<Result<i32, AppError>>) -> (Vec<i32>, Vec<AppError>)` that separates successes and errors.
4. Implement `fmt::Display` for `AppError` using `.source()` to print the full chain.
5. In OCaml, extend `app_error` with a `WithContext { context: string; inner: app_error }` variant and write a `display_chain` function that prints the full error hierarchy.
