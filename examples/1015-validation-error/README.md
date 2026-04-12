📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1015-validation-error)**

---

# 1015-validation-error — Validation Errors
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Form validation, data ingestion, and configuration parsing all share a common need: report every error at once rather than stopping at the first one. A user who submits a form with three invalid fields should not have to submit-fix-submit-fix three times. This "accumulate all errors" pattern requires a different data structure than `Result<T, E>`, which can carry only one error at a time.

The standard approach is to validate each field independently, collect errors into a `Vec<FieldError>`, and return them together. Libraries like Haskell's `Validation` type, OCaml's `Base.Or_error`, and Rust crates like `validator` formalize this pattern.

## Learning Outcomes

- Design a `FieldError` type that carries both the field name and a human-readable message
- Write independent per-field validators that return `Vec<FieldError>`
- Accumulate errors from multiple validators into a single report
- Distinguish fail-fast (`Result`) from accumulate-all (`Vec<FieldError>`) error handling
- Know when each strategy is appropriate in production systems

## Rust Application

`src/lib.rs` defines `FieldError { field, message }` and separate validators `validate_name` and `validate_age`, each returning `Vec<FieldError>`. The top-level validation function extends an error vector with results from each field validator. If the final vector is empty, the input is valid; otherwise, all errors are returned together.

This pattern appears in web frameworks (Actix, Axum), CLI argument validators, and data pipeline preprocessors.

## OCaml Approach

OCaml's `Base` library provides `Validate.t` for this pattern. Without a library, the same logic uses `List.concat_map`:

```ocaml
type field_error = { field: string; message: string }

let validate_all validators input =
  List.concat_map (fun v -> v input) validators

let is_valid errors = errors = []
```

Functional libraries often use an `Applicative` functor over a `Validation` type to compose validators without short-circuiting, analogous to `Result`'s `Applicative` instance in Haskell.

## Key Differences

1. **Fail-fast vs accumulate**: `Result` short-circuits at first error; `Vec<FieldError>` accumulates all. The choice affects API design fundamentally.
2. **Composition**: Rust validators return `Vec` and are composed by extending vectors; OCaml functional validators can be composed with applicative combinators.
3. **Library support**: The Rust ecosystem has `validator`, `garde`, and `nutype` for declarative validation; OCaml has `Base.Validate` and `ppx_jane` derivations.
4. **Type-level guarantee**: Rust's type system can encode "validated" vs "unvalidated" data using the newtype pattern; OCaml uses the same technique with opaque types.

## Exercises

1. Add an `email` field to the validated struct with a validator that checks for the presence of `@` and a non-empty domain.
2. Write a `validate_all` function that takes a list of validators and returns `Ok(())` if all pass, or `Err(Vec<FieldError>)` if any fail.
3. Implement a `Validated<T>` newtype wrapper that can only be constructed by a successful validation, preventing accidental use of unvalidated data.
