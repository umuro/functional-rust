📖 **[View on hightechmind.io →](https://hightechmind.io/rust/054-applicative-validation)**

---

# 054 — Applicative Validation

## Problem Statement

Standard `Result` short-circuits at the first error — if name validation fails, age and email are never checked. For user-facing forms, you want to collect ALL errors and report them together. Applicative validation (inspired by Haskell's `Validation` type) accumulates errors rather than short-circuiting.

This pattern is essential in form validation, API request validation, configuration file validation, and data import pipelines. Instead of "the form has an error" (Result), you get "the form has 3 errors: name too long, email invalid, age negative". Used in Haskell's `validation` crate, Scala's `Validated`, and Rust's `garde`/`validator` crates.

## Learning Outcomes

- Understand the difference between fail-fast (Result) and accumulate-all (Validation)
- Implement individual validators returning `Result<T, Vec<E>>`
- Combine validators by merging error vectors from both sides
- Understand the applicative functor pattern: `f <*> a` where both f and a may have errors
- Recognize that validation is not a monad (no `and_then`) because the second step does not depend on the first

## Rust Application

Individual validators return `Result<T, Vec<ValidationError>>`. `validate_all` combines them: `match (validate_name(name), validate_age(age), validate_email(email)) { (Ok(n), Ok(a), Ok(e)) => Ok(Person { name: n, age: a, email: e }), _ => { let mut errors = vec![...]; ... Err(errors) } }`. The key is merging error vectors from all failing validators rather than returning the first error.

## OCaml Approach

OCaml defines a `validation` type: `type ('a, 'e) validation = Ok of 'a | Errors of 'e list`. The applicative `combine`: `let combine v1 v2 f = match v1, v2 with | Ok a, Ok b -> Ok (f a b) | Ok _, Errors e | Errors e, Ok _ -> Errors e | Errors e1, Errors e2 -> Errors (e1 @ e2)`. The key: combine merges error lists from both sides, even when both fail.

## Key Differences

1. **Not a monad**: `Validation` is an applicative functor but not a monad. The second computation does not depend on the first's result — all run independently. `Result` (a monad) can model dependency. This is the fundamental difference.
2. **Error merging**: `Validation` merges error lists on both failure cases. `Result::and_then` only runs the second step if the first succeeds — it cannot accumulate errors.
3. **`Vec<E>` errors**: Both implementations use `Vec<ValidationError>` for the error accumulator. The individual validators return single errors wrapped in `vec![...]` for uniformity.
4. **Crates**: The `garde` and `validator` crates implement applicative validation for Rust structs via derive macros. Understanding the manual implementation explains what these macros generate.

## Exercises

1. **Form validation**: Write a `validate_registration(form: &RegistrationForm) -> Result<User, Vec<String>>` that validates username length, password strength, and email format simultaneously.
2. **Parallel vs sequential**: Write the same validation using sequential `and_then` (stops at first error) and parallel `Validation` (collects all errors). Demonstrate with an input that has 3 errors.
3. **Custom accumulator**: Instead of `Vec<ValidationError>`, use `HashMap<String, Vec<String>>` as the error type, where keys are field names. This gives per-field error messages.
