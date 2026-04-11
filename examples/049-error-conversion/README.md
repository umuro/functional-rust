📖 **[View on hightechmind.io →](https://hightechmind.io/rust/049-error-conversion)**

---

# 049 — Error Conversion

## Problem Statement

Real applications deal with multiple error types from different libraries: `std::io::Error` from file I/O, `serde_json::Error` from JSON parsing, `reqwest::Error` from HTTP — all in the same function. To use `?` with mixed error types, you need a unified error type and `From` implementations for each source error.

Error conversion is where Rust's error handling design becomes concrete. The `From` trait is the mechanism: `impl From<IoError> for AppError { ... }` enables automatic conversion via `?`. This is analogous to OCaml's polymorphic variant unification — combining multiple error cases into a single type.

## Learning Outcomes

- Implement `From<SourceError>` for a unified `AppError` type
- Use `?` with multiple different error types in the same function via `From` conversion
- Use `map_err` for one-off error conversions without defining `From`
- Understand the trade-off: `From` is clean but requires boilerplate; `Box<dyn Error>` is quick but less typed
- Use the `thiserror` crate pattern for reducing boilerplate

- Implement `From<SpecificError> for AppError` to enable automatic `?`-based conversion
- Use `Box<dyn std::error::Error>` as a catch-all for functions that combine multiple error types

## Rust Application

Define `enum AppError { Io(std::io::Error), Parse(ParseIntError) }`. Implement `From<std::io::Error> for AppError` and `From<ParseIntError> for AppError`. Then in a function returning `Result<T, AppError>`, use `?` on both `std::fs::read_to_string(...)` (IoError) and `str.parse::<i32>()` (ParseIntError) — the conversions happen automatically.

## OCaml Approach

OCaml's approach uses variant types: `type app_error = Io of string | Parse of string`. Helper constructors wrap each error: `let wrap_io r = Result.map_error (fun e -> Io (Printexc.to_string e)) r`. Chaining: `let* data = wrap_io (read_file path) in let* n = Result.map_error (fun e -> Parse e) (parse_int data) in Ok n`. There is no automatic `From`-style conversion; wrapping is always explicit.

## Key Differences

1. **Automatic vs explicit**: Rust's `From` + `?` converts automatically at the `?` site. OCaml requires explicit `Result.map_error` or wrapping helper functions at each error site.
2. **`thiserror` crate**: The `thiserror` crate generates `From` implementations via `#[from]` attribute on enum fields. OCaml has no equivalent; Dune macros or PPX would be needed for comparable automation.
3. **`Box<dyn Error>`**: Rust's `Box<dyn std::error::Error>` accepts any error type without explicit conversion — useful for prototyping. OCaml's equivalent is catching exceptions or using `string` as error type.
4. **Type safety**: Rust's explicit `From` implementations make error type conversions checked at compile time. `Box<dyn Error>` sacrifices this for convenience.

1. **`From` trait enables `?`:** When `?` converts from error type `A` to error type `B`, it uses `B::from(a)`. Implementing `From<A> for B` makes this automatic. This is why library error types implement `From` for all constituent error types.
2. **`Box<dyn Error>` for type erasure:** Returning `Box<dyn Error>` accepts any error type — the concrete type is erased. Useful in applications but poor for libraries (callers can't pattern-match on the error).
3. **`thiserror` and `anyhow`:** The `thiserror` crate automates `From` and `Display` implementations. The `anyhow` crate uses `Box<dyn Error>` with added context. Both are idiomatic in real Rust code.
4. **OCaml lacks `?`:** OCaml's error conversion is always manual — match and wrap explicitly, or use a ppx for syntactic sugar.

## Exercises

1. **Three-way error**: Define `AppError` with three variants for IoError, ParseIntError, and a custom `DomainError`. Implement `From` for all three and write a function using all three error types.
2. **`thiserror` pattern**: Without using the `thiserror` crate, manually write what `#[derive(thiserror::Error)]` would generate for your `AppError` enum. Include `Display` and `std::error::Error` impls.
3. **Error hierarchy**: Design an error type hierarchy for a web server: `DatabaseError`, `AuthError`, `ValidationError` all converting into `ApiError`. Draw the conversion graph.

4. **`thiserror` macro**: Rewrite a custom error type using the `thiserror` crate's `#[derive(Error)]` macro. Compare the amount of boilerplate with the manual implementation.
5. **Context wrapping**: Implement `with_context<T, E: Display>(result: Result<T, E>, context: &str) -> Result<T, String>` that wraps an error with additional context, producing a `String` error that includes both the context and original message.
