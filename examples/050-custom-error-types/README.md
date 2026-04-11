📖 **[View on hightechmind.io →](https://hightechmind.io/rust/050-custom-error-types)**

---

# 050 — Custom Error Types

## Problem Statement

Custom error types make error handling self-documenting. Instead of returning `String` errors (which lose structure) or `Box<dyn Error>` (which loses type safety), a custom enum provides exhaustive matching, compiler-checked error handling, and rich error messages. This is how Rust's standard library and major crates (`serde`, `tokio`, `axum`) define their errors.

The `std::error::Error` trait requires implementing `Display` and optionally `source()` (for error chaining). The `Display` implementation provides human-readable messages. Combined with `From` conversions (example 049), custom error types are the foundation of production Rust error handling.

## Learning Outcomes

- Define a custom error enum with multiple variants carrying context data
- Implement `std::fmt::Display` for human-readable error messages
- Implement `std::error::Error` to integrate with the Rust ecosystem
- Use custom errors with `?` and `map_err` in real functions
- Understand the `source()` method for error chaining (wrapping underlying causes)

- Define a custom error enum with one variant per failure mode and implement `fmt::Display` for human-readable messages
- Implement `std::error::Error` and optionally `source()` for error chaining with the underlying cause

## Rust Application

Define `enum ValidationError { NegativeAge(i32), UnreasonableAge(i32), EmptyName }`. Implement `Display`: `ValidationError::NegativeAge(n) => write!(f, "negative age: {}", n)`. Implement `std::error::Error` with an empty body (auto-derives `source` returning `None`). Functions return `Result<T, ValidationError>`. Callers can `match` on the specific variant for recovery or use `?` to propagate.

## OCaml Approach

OCaml's equivalent: define `type validation_error = NegativeAge of int | UnreasonableAge of int | EmptyName`. The `Display` equivalent is a `to_string` function: `let error_to_string = function NegativeAge n -> Printf.sprintf "negative age: %d" n | ...`. OCaml has no `Error` trait — errors are just values. The `Printexc` module handles exception pretty-printing separately.

## Key Differences

1. **Trait implementation**: Rust requires implementing `Display` and `Error` traits to integrate with the ecosystem. OCaml errors are plain algebraic types — no trait/interface needed, but also no standard display mechanism.
2. **Error chaining**: Rust's `Error::source()` returns the underlying cause. OCaml achieves this by embedding the cause in the error variant: `type error = IoFailed of exn | ...`.
3. **Derive macros**: With `thiserror::derive(Error)`, Rust can derive `Display` and `Error` from attribute annotations. OCaml has no standard equivalent; writing `to_string` functions manually is typical.
4. **Pattern matching**: Both languages allow exhaustive matching on error variants. Rust's `match` is checked at compile time — adding a new variant requires handling it everywhere it is matched.

1. **`enum` for error variants:** A custom error type is an enum — each variant represents a different failure mode. This allows callers to pattern-match on specific errors and handle them differently.
2. **`Display` for human-readable messages:** Implementing `fmt::Display` for the error type provides the human-readable message shown in logs and user interfaces. `Debug` is for developers; `Display` is for users.
3. **`Error` trait:** Implementing `std::error::Error` marks the type as an error type. It requires `Debug + Display`. The `source()` method provides error chaining — linking to the underlying cause.
4. **OCaml custom errors:** OCaml uses variant types: `type error = InvalidInput | NetworkError of string | ParseError of int * string`. Pattern matching handles each case. OCaml doesn't have a standard `Error` trait — error handling style varies by library.

## Exercises

1. **Error source chain**: Add a `NetworkError { message: String, cause: Box<dyn std::error::Error> }` variant to your error type and implement `source()` to return `Some(cause.as_ref())`. Write a function that wraps an underlying error.
2. **Display with context**: Write error messages that include both the bad value and what was expected: `"expected age 0-150, got 200"`. Use `write!(f, ...)` with multiple fields from the variant.
3. **Error catalogue**: Define a complete `ApiError` type for a web API with variants for `NotFound { resource: String }`, `Unauthorized`, `RateLimit { retry_after_secs: u64 }`, and `Internal(Box<dyn std::error::Error>)`. Implement `Display`, `Error`, and HTTP status code mapping.

4. **Error hierarchy**: Implement a three-level error hierarchy: `DbError`, `ApiError` (which wraps `DbError`), and `AppError` (which wraps `ApiError`). Implement `From` conversions so `?` works at each level.
5. **Error reporting**: Add a `fn suggestions(&self) -> Vec<String>` method to your custom error type that returns actionable suggestions for each error variant — e.g., for `InvalidInput` suggest checking the input format.
