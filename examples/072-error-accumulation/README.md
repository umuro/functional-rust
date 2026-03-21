📖 **[View on hightechmind.io →](https://hightechmind.io/rust/072-error-accumulation)**

---

# 072 — Error Accumulation (Validation Type)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

This example implements a custom `Validation<T, E>` type that accumulates errors rather than short-circuiting (see also example 054 for the practical use case). The distinction is fundamental: `Result` is a monad (errors short-circuit), `Validation` is an applicative functor (errors accumulate). You cannot derive `Validation` from `Result`.

The `Validation` pattern comes from Haskell's `Data.Validation` and Scala's `cats.data.Validated`. It is essential when all errors are independent and should all be reported, such as form validation, configuration parsing, and CSV row validation.

## Learning Outcomes

- Define a `Validation<T, E>` enum as a first-class type (not using `Result`)
- Implement `map` (functor) for transforming the success value
- Implement `apply` (applicative) for combining two `Validation` values, merging error lists
- Understand why `Validation` is not a monad (second step cannot depend on first result)
- Connect to the applicative functor laws

## Rust Application

`Validation<T, E>` has variants `Ok(T)` and `Errors(Vec<E>)`. `map(f)` applies `f` to the success value. The `apply` method combines a `Validation<F, E>` (containing a function) with a `Validation<T, E>` (containing an argument): if both are `Ok`, apply the function; if either or both are `Errors`, merge the error vectors. This is the key: errors from BOTH sides are combined.

## OCaml Approach

OCaml's `type ('a, 'e) validation = Ok of 'a | Errors of 'e list`. Applicative `combine v1 v2 = match v1, v2 with | Ok a, Ok b -> Ok (a, b) | Ok _, Errors e | Errors e, Ok _ -> Errors e | Errors e1, Errors e2 -> Errors (e1 @ e2)`. The key property: `Errors(e1) combine Errors(e2) = Errors(e1 @ e2)` — both error lists are kept.

## Key Differences

1. **Not a `Result`**: `Validation` and `Result` have different semantics despite similar structure. `Result::and_then` is sequential (second step sees first result). `Validation::apply` is parallel (both sides run independently).
2. **`Vec<E>` vs `E`**: `Validation` collects errors in `Vec<E>`. `Result` carries a single `E`. The vector accumulation is what enables reporting multiple errors.
3. **No `and_then`**: `Validation` intentionally does not have `and_then` (monadic bind) — implementing it would require making the second step depend on the first, losing the ability to accumulate errors from the second step when the first fails.
4. **`apply` complexity**: The `apply` method requires `Self: Into<Validation<F, E>>` — a somewhat awkward bound in Rust due to the function being inside a `Validation`. Practice using it with concrete types first.

## Exercises

1. **Validated constructor**: Write `validated_person(name: &str, age: i32, email: &str) -> Validation<Person, ValidationError>` that validates all three fields simultaneously and accumulates errors.
2. **Map2**: Write `map2<A, B, C, E>(va: Validation<A, E>, vb: Validation<B, E>, f: impl FnOnce(A, B) -> C) -> Validation<C, E>` as a higher-level combinator over `apply`.
3. **List validation**: Write `validate_all<T, E, F>(items: &[T], validate: F) -> Validation<Vec<T>, E>` where `validate: Fn(&T) -> Validation<T, E>`. Return all validation errors across all items.
