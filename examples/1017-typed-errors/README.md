📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1017-typed-errors)**

---

# 1017-typed-errors — Typed Error Hierarchies
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

As applications grow, different subsystems produce different categories of errors. A web service has authentication errors, database errors, network errors, and business logic errors. Representing all of these as `String` or `Box<dyn Error>` loses type information that callers could use to take specific recovery actions — retry on a timeout, redirect on auth failure, or surface a 400 vs 500 HTTP status code.

Typed error enums let callers pattern-match on the error variant, enabling precise handling. The `thiserror` crate automates the boilerplate, but the underlying pattern is pure Rust trait implementations.

## Learning Outcomes

- Design an error enum hierarchy with subsystem-specific variants
- Implement `Display` and `std::error::Error` for each error type
- Use `From<SubsystemError>` to convert subsystem errors into top-level errors with `?`
- Understand when typed errors are better than `anyhow::Error`
- Pattern-match on error variants in call sites for specific recovery logic

## Rust Application

`src/lib.rs` defines `DbError`, `AuthError`, and `ApiError` as separate enums, each implementing `Display` and `Error`. A top-level `AppError` enum wraps all three with variant wrappers (`AppError::Db(DbError)`, etc.) and `From` impls for each. Functions in each subsystem return their specific error type; the top-level handler uses `?` with auto-conversion.

This architecture is standard in production Rust services: `axum`, `actix-web`, and `tonic` all recommend typed error enums for handler return types.

## OCaml Approach

OCaml uses polymorphic variants or module-scoped exception types for typed error hierarchies:

```ocaml
type db_error = ConnectionFailed | QueryFailed of string
type auth_error = InvalidToken | Expired
type app_error = Db of db_error | Auth of auth_error

let handle = function
  | Db ConnectionFailed -> retry ()
  | Auth Expired -> refresh_token ()
  | _ -> internal_error ()
```

`Base.Or_error` provides `Error.t` which can be tagged and introspected, but the pattern-matching approach above is more common for typed hierarchies.

## Key Differences

1. **`From` trait**: Rust's `From<SubsystemError> for AppError` enables automatic conversion with `?`; OCaml requires explicit variant wrapping.
2. **Display vs Show**: Rust's `Display` trait formats errors for human consumption; OCaml typically uses `to_string` methods or `Format.fprintf` in a polymorphic variant.
3. **Error chaining**: Rust's `Error::source()` method provides a standard way to walk the cause chain; OCaml's `Base.Error` uses a lazy tree structure.
4. **`thiserror` automation**: The `thiserror` crate generates `Display`, `Error`, and `From` impls via `#[derive]`; OCaml has `ppx_sexp_conv` for serialisation but no equivalent.

## Exercises

1. Add a `ValidationError(String)` variant to `AppError` and a mock function that returns it. Pattern-match on it in a handler that returns a 400 status code string.
2. Implement `Error::source()` for `AppError` so each variant returns its inner error as the cause.
3. Refactor the example to use `thiserror::Error` derive macro and verify the generated code matches the manual implementation.
