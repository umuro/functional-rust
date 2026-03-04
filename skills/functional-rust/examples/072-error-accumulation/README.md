# 072: Error Accumulation

**Difficulty:** 2  **Level:** Intermediate

Collect *all* validation errors instead of stopping at the first one.

## The Problem This Solves

`Result<T, E>` short-circuits: the `?` operator returns on the first `Err`. This is perfect for sequential logic where each step depends on the previous. But for form validation, it's terrible UX: submit a form with five errors, get one error back, fix it, submit again, get a different error. Five round trips to find five problems.

Validation requires a different abstraction: run all checks independently, collect every failure, and return either a fully valid value or the complete list of errors. This is the `Validation` type (also called `Validated` in Haskell/Scala) — an applicative functor that accumulates errors rather than short-circuiting.

The key insight: `Result` uses monadic composition (sequential, short-circuit). `Validation` uses applicative composition (parallel, accumulating). Same concept of "might fail", different failure strategy.

## The Intuition

Think of `Result` as a pipeline with an emergency stop: any error halts everything. Think of `Validation` as a checklist: every box gets checked regardless, then you see the full list of problems.

Both hold either a success value or an error. The difference is what happens when you combine two failures: `Result` keeps only the first, `Validation` merges both error lists.

```
Result:     Err("name empty") then Err("age invalid") → Err("name empty")   [stops]
Validation: Err("name empty") + Err("age invalid")    → Errors(["name empty", "age invalid"])   [collects]
```

## How It Works in Rust

```rust
#[derive(Debug, PartialEq)]
pub enum Validation<T, E> {
    Ok(T),
    Errors(Vec<E>),   // note: Vec, not a single E
}

// Validators return Validation, not Result:
pub fn validate_name(s: &str) -> Validation<&str, String> {
    if !s.is_empty() { Validation::Ok(s) }
    else { Validation::Errors(vec!["name cannot be empty".to_string()]) }
}

// Collect all errors by iterating validators:
let results = [validate_name(""), validate_age(15), validate_email("bad")];
let all_errors: Vec<String> = results
    .into_iter()
    .flat_map(|v| match v {
        Validation::Errors(es) => es,
        Validation::Ok(_)      => vec![],
    })
    .collect();
// all_errors == ["name cannot be empty", "age 15 out of range", "email must contain @"]

// Functor: transform success value, pass errors through
impl<T, E> Validation<T, E> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validation<U, E> {
        match self {
            Validation::Ok(x)     => Validation::Ok(f(x)),
            Validation::Errors(es) => Validation::Errors(es),
        }
    }
}
```

The critical difference from `Result`: when combining two `Errors` variants (in `apply`), the error lists are merged with `extend`. Both failures are preserved.

## What This Unlocks

- **Form validation** — report every field error in one response rather than one per submission.
- **Batch processing with full error reports** — validate an entire CSV file, collect all invalid rows.
- **Understanding applicative vs monad** — `Validation` is applicative (parallel composition); `Result` is monadic (sequential). Knowing when to use each is a key FP design skill.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type | `type ('a, 'e) validation = Ok of 'a \| Errors of 'e list` | `enum Validation<T, E> { Ok(T), Errors(Vec<E>) }` |
| Combining errors | `Errors e1 @ e2` (list append) | `e1.extend(e2)` (Vec extend) |
| vs `Result` | OCaml's `result` also short-circuits | `Result<T, E>` short-circuits; `Validation` accumulates |
| Applicative apply | Module-level functor pattern | `apply` method on `Validation` |
| Monad vs applicative | `>>=` vs `<*>` distinction | `.and_then` (monad) vs `.apply` (applicative) |
