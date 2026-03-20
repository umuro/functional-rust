📖 **[View on hightechmind.io →](https://hightechmind.io/rust/313-try-trait)**

---

# 313: The Try Trait — What ? Actually Does

## Problem Statement

The `?` operator desugars to a call to the `Try` trait (unstable) or the earlier `From` + early-return pattern. This example demonstrates the concept using a `Validated<T, E>` type that accumulates multiple errors instead of short-circuiting — illustrating that the `?` behavior is customizable. Understanding what `?` actually does enables implementing custom types that participate in Rust's error-handling ergonomics.

## Learning Outcomes

- Understand `?` as desugaring to: extract value or convert error and return early
- Implement a `Validated` type that accumulates errors instead of short-circuiting
- Recognize that `?` semantics are defined by the return type, not the operator itself
- See how `Result` and `Option` implement the early-return contract that `?` relies on

## Rust Application

The `Validated` type shows the contrast with `Result`'s short-circuit behavior:

```rust
#[derive(Debug, PartialEq)]
pub enum Validated<T, E> {
    Ok(T),
    Err(Vec<E>),
}

impl<T, E> Validated<T, E> {
    // combine_errors: merge error lists from multiple validations
    pub fn and<U>(self, other: Validated<U, E>) -> Validated<(T, U), E> {
        match (self, other) {
            (Validated::Ok(t), Validated::Ok(u)) => Validated::Ok((t, u)),
            (Validated::Err(mut e1), Validated::Err(e2)) => {
                e1.extend(e2); Validated::Err(e1)  // Accumulate ALL errors
            }
            (Validated::Err(e), _) | (_, Validated::Err(e)) => Validated::Err(e),
        }
    }
}
```

## OCaml Approach

OCaml's `let*` desugars to `bind` — the behavior is determined by the monad, not the syntax. A `Validated` monad in OCaml accumulates errors in its `bind` (applicative) form:

```ocaml
(* Applicative validation: both branches evaluated, errors accumulated *)
let validate_both v1 v2 = match (v1, v2) with
  | (Valid x, Valid y) -> Valid (x, y)
  | (Invalid e1, Invalid e2) -> Invalid (e1 @ e2)
  | (Invalid e, _) | (_, Invalid e) -> Invalid e
```

## Key Differences

1. **Monad vs applicative**: Short-circuit (`Result`, `Option`) is monadic; error accumulation (`Validated`) is applicative — fundamentally different composition strategies.
2. **`?` limitation**: Rust's `?` is monadic (short-circuit only); accumulation requires explicit `and()` or similar applicative operations.
3. **Form validation**: Accumulation is the right strategy for form validation — show all errors at once, not one at a time.
4. **Cats/Haskell**: Haskell's `Validation` type in `validation` crate / `Data.Validation` directly mirrors this; PureScript, Elm, and other FP languages have similar types.

## Exercises

1. Implement a form validator using `Validated<T, String>` that validates a name (non-empty) and age (18-100) simultaneously, returning all errors if both fail.
2. Add an `and_then` method to `Validated` that behaves identically to `Result::and_then` (short-circuits on `Err`) — show when to use each.
3. Convert between `Validated<T, Vec<E>>` and `Result<T, Vec<E>>` — what information is preserved or lost in each direction?
