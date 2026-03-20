# Example 1084: Monoid Pattern

## Problem Statement
Demonstrates the monoid algebraic structure — a type with an identity element and an associative binary operation — and how to implement it generically to reduce collections uniformly.

## Learning Outcomes
- Understand what makes a monoid (identity + associativity) and why it matters for generic reduction
- Implement a trait-based abstraction that works across multiple concrete types
- Use `fold` with a monoid identity to collapse collections without special-casing empty inputs

## Rust Application
Rust uses a `Monoid` trait with `empty()` and `combine()`, applied via a generic `reduce_monoid` function that folds over any slice whose element type implements the trait. Newtype wrappers (`Sum`, `Product`) keep implementations distinct without name collisions.

## OCaml Approach
OCaml expresses monoids as first-class modules satisfying a `MONOID` signature, passing them explicitly at the call site via locally abstract types. This avoids newtypes entirely — the integer type is reused directly under different module implementations.

## Key Differences
1. **Abstraction mechanism:** OCaml uses parameterized modules (first-class modules) vs Rust uses traits with generic type parameters
2. **Type disambiguation:** OCaml scopes implementations by module namespace vs Rust requires newtype wrappers to implement the same trait for the same underlying type
3. **Identity element:** OCaml expresses `empty` as a module-level value vs Rust requires an associated function `empty() -> Self`

## Exercises
1. Implement a `Max` monoid over integers using `i32::MIN` as the identity and extend `reduce_monoid` to find the maximum of a slice
2. Implement a `StringConcat` monoid and verify that `reduce_monoid` on an empty slice returns an empty string
3. Add a `combine_all` function that takes two slices of the same monoid type and interleaves their reductions, then verify associativity holds
