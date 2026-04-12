📖 **[View on hightechmind.io →](https://hightechmind.io/rust/230-semigroup)**

---

# Example 230: Semigroup
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Model a semigroup — a type equipped with an associative binary operation — and show how several familiar operations (min, max, concatenation, first) are all instances of the same abstraction.

## Learning Outcomes

- How to encode type-class-style abstractions as Rust traits
- Why Rust newtypes are needed when multiple semigroup instances exist for the same primitive type
- How `split_first` + `fold` cleanly expresses "reduce a non-empty sequence" without panicking
- Why the absence of an identity element (unlike `Monoid`) means `sconcat` returns `Option<S>`

## OCaml Approach

OCaml uses a module signature `SEMIGROUP` with a single `append` value, then functor-style first-class modules passed to `sconcat`. The `List.fold_left` inside `sconcat` does the left-associative reduction, and a missing identity means an empty list causes `failwith`.

## Rust Approach

Rust uses a trait `Semigroup` with a single `fn append(self, other: Self) -> Self`. Each instance (Min, Max, First, NonEmptyList) is a newtype wrapper, avoiding coherence conflicts that would arise from implementing the trait directly on `i64`. `sconcat` returns `Option<S>` instead of panicking — idiomatic Rust prefers explicit failure over exceptions.

## Key Differences

1. **Module system vs traits:** OCaml first-class modules (`(module MinSemigroup)`) become Rust generic type parameters (`<S: Semigroup>`).
2. **Failure mode:** OCaml `failwith` on empty list vs Rust `Option::None` — Rust makes partial functions explicit in the type.
3. **Multiple instances per type:** OCaml uses distinct named modules; Rust uses newtypes (`Min`, `Max`) to avoid orphan/coherence violations.
4. **Ownership:** `append(self, other: Self)` consumes both values — idiomatic for value types; `Clone` bound lets combinators work on slices.

## Exercises

1. Implement a `NonEmptyVec<T>` type and give it a `Semigroup` instance (concatenation); verify that unlike `Vec`, the semigroup has no identity element so it cannot form a monoid.
2. Define a `Max<T: Ord>` and `Min<T: Ord>` semigroup (not monoid — no identity for unbounded types) and implement `sconcat` over a non-empty list to find the maximum/minimum.
3. Implement the free semigroup: a non-empty list type, and prove that any semigroup homomorphism from it is uniquely determined by the image of each generator.
