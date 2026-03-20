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

## Rust Application

Define `enum AppError { Io(std::io::Error), Parse(ParseIntError) }`. Implement `From<std::io::Error> for AppError` and `From<ParseIntError> for AppError`. Then in a function returning `Result<T, AppError>`, use `?` on both `std::fs::read_to_string(...)` (IoError) and `str.parse::<i32>()` (ParseIntError) — the conversions happen automatically.

## OCaml Approach

OCaml's approach uses variant types: `type app_error = Io of string | Parse of string`. Helper constructors wrap each error: `let wrap_io r = Result.map_error (fun e -> Io (Printexc.to_string e)) r`. Chaining: `let* data = wrap_io (read_file path) in let* n = Result.map_error (fun e -> Parse e) (parse_int data) in Ok n`. There is no automatic `From`-style conversion; wrapping is always explicit.

## Key Differences

1. **Automatic vs explicit**: Rust's `From` + `?` converts automatically at the `?` site. OCaml requires explicit `Result.map_error` or wrapping helper functions at each error site.
2. **`thiserror` crate**: The `thiserror` crate generates `From` implementations via `#[from]` attribute on enum fields. OCaml has no equivalent; Dune macros or PPX would be needed for comparable automation.
3. **`Box<dyn Error>`**: Rust's `Box<dyn std::error::Error>` accepts any error type without explicit conversion — useful for prototyping. OCaml's equivalent is catching exceptions or using `string` as error type.
4. **Type safety**: Rust's explicit `From` implementations make error type conversions checked at compile time. `Box<dyn Error>` sacrifices this for convenience.

## Exercises

1. **Three-way error**: Define `AppError` with three variants for IoError, ParseIntError, and a custom `DomainError`. Implement `From` for all three and write a function using all three error types.
2. **`thiserror` pattern**: Without using the `thiserror` crate, manually write what `#[derive(thiserror::Error)]` would generate for your `AppError` enum. Include `Display` and `std::error::Error` impls.
3. **Error hierarchy**: Design an error type hierarchy for a web server: `DatabaseError`, `AuthError`, `ValidationError` all converting into `ApiError`. Draw the conversion graph.
