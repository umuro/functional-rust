📖 **[View on hightechmind.io →](https://hightechmind.io/rust/314-validated-accumulation)**

---

# 314: Validated — Accumulating All Errors
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

User registration forms, configuration validation, and batch processing all share a need: show all errors at once, not just the first one. When a form has 10 invalid fields, showing only the first error forces the user to submit nine more times. The `Validated` type addresses this with applicative composition: validate all fields independently, then combine results — accumulating every error if multiple validations fail simultaneously.

## Learning Outcomes

- Understand the difference between monadic (`Result`) and applicative (`Validated`) error handling
- Implement `Validated<T, E>` with `valid()`, `invalid()`, `map()`, and `combine()` operations
- Use `Validated` to validate multiple independent fields simultaneously
- Recognize when accumulation (show all errors) vs short-circuit (stop at first) is the right strategy

## Rust Application

Validation of multiple fields simultaneously collects all errors:

```rust
#[derive(Debug, PartialEq)]
pub enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),
}

pub fn validate_age(age: i32) -> Validated<i32, String> {
    if age >= 0 && age <= 150 { Validated::valid(age) }
    else { Validated::invalid(format!("age {} out of range", age)) }
}

pub fn validate_name(name: &str) -> Validated<String, String> {
    if !name.is_empty() { Validated::valid(name.to_string()) }
    else { Validated::invalid("name cannot be empty".to_string()) }
}

// Both validations run; both errors collected if both fail:
pub fn validate_user(name: &str, age: i32) -> Validated<(String, i32), String> {
    validate_name(name).and(validate_age(age))
}
```

## OCaml Approach

OCaml's `Ppx_let` and applicative functors support this pattern. `Lwt.both` and similar functions provide concurrent validation with error accumulation:

```ocaml
type ('a, 'e) validated = Valid of 'a | Invalid of 'e list

let and_validate v1 v2 = match (v1, v2) with
  | (Valid x, Valid y) -> Valid (x, y)
  | (Invalid es1, Invalid es2) -> Invalid (es1 @ es2)
  | (Invalid es, _) | (_, Invalid es) -> Invalid es
```

## Key Differences

1. **Applicative vs monadic**: `Validated` is applicative (both sides computed); `Result` is monadic (short-circuits on `Err`).
2. **Semantic choice**: Short-circuit when errors are dependent (step 2 requires step 1); accumulate when errors are independent (all form fields).
3. **Production use**: The Rust `garde` and `validator` crates use accumulation for struct validation — all field errors are collected and returned together.
4. **Conversion**: `Validated<T, E>` can be converted to `Result<T, Vec<E>>` by taking the first error or collecting all errors into a summary.

## Exercises

1. Extend the user validator with a third field (email must contain `@`) and verify that invalid name + invalid email reports both errors.
2. Implement a `traverse` function: `fn traverse<T, U, E>(items: Vec<T>, f: impl Fn(T) -> Validated<U, E>) -> Validated<Vec<U>, E>` that validates all items and accumulates all errors.
3. Compare the output of `Validated` vs `Result` on the same validation logic — show that `Result` stops at first failure while `Validated` collects all.
