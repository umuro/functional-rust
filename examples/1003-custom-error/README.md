[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1003 — Custom Error Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Define custom error enums and implement `fmt::Display` and `std::error::Error` for them. Create `ValidationError` for age and name validation, and `DetailedError` for field-level errors with context. Compare with OCaml's exception-based error handling and variant-based `Result` errors.

## Learning Outcomes

- Implement `fmt::Display` on an error enum with descriptive per-variant messages
- Implement `std::error::Error` (often just `impl std::error::Error for MyError {}`)
- Return `Result<T, ValidationError>` from validation functions
- Understand the relationship between `Display`, `Debug`, and `Error`
- Map Rust's enum-based errors to OCaml's `exception` and type variant approaches
- Recognise when to use a simple enum vs a struct error with context fields

## Rust Application

`ValidationError` is an enum with three variants, each with `impl fmt::Display` providing human-readable messages. `impl std::error::Error for ValidationError {}` is an empty impl (it just marks the type as an error). `validate_age` and `validate_name` return `Result<_, ValidationError>` with early returns using `Err(…)`. `DetailedError` is a struct with `field` and `message` strings — useful when errors need structured context beyond what a variant name provides.

## OCaml Approach

OCaml offers two approaches: `exception Invalid_age of string` for traditional exception-based flow, and `type validation_error = NegativeAge of int | UnreasonableAge of int | EmptyName` for `Result`-based flow. Both are idiomatic. The exception approach uses `raise`/`try … with` syntax. The `Result` approach mirrors Rust exactly. OCaml's type inference makes `Result` functions more concise since the error type is inferred.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Error type | `enum ValidationError` | `type validation_error` or `exception` |
| Display | `impl fmt::Display` | `string_of_validation_error` function |
| `Error` trait | `impl std::error::Error {}` | No equivalent trait |
| `?` operator | Propagates `Result` | No equivalent (use bind or `let*`) |
| Context | Struct `DetailedError { field, message }` | Record or variant with fields |
| Panic | `panic!("…")` | `failwith "…"` / `assert false` |

The `Display` + `Error` combination is the Rust ecosystem contract for error types. Libraries like `anyhow` and `thiserror` build on this foundation — `thiserror` derives `Display` from format strings, eliminating the boilerplate.

## Exercises

1. Add a `from` method: `ValidationError::from_str(s: &str) -> Option<ValidationError>` that parses an error message back to a variant.
2. Implement `std::error::Error::source` to chain errors: add a `ValidationError::ChainedError(Box<dyn Error>)` variant.
3. Implement `From<ValidationError> for String` so `.to_string()` calls produce the display message.
4. Use `thiserror::Error` derive macro to eliminate the `Display` and `Error` boilerplate.
5. In OCaml, implement a `Validation` module that accumulates multiple errors (not just the first) using `Result.bind` and `List`.
