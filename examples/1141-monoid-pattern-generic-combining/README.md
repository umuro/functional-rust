# Example 1141: Monoid Pattern — Generic Combining

**Difficulty:** ⭐⭐
**Category:** Type Classes & Traits
**OCaml Source:** Real World OCaml / first-class modules pattern

## Problem Statement

Define a `Monoid` abstraction — a type with an identity element and an associative binary operation — and implement a generic `concat_all` that reduces any list using that abstraction. Show concrete instances for sum, product, string concatenation, and boolean conjunction.

## Learning Outcomes

- How OCaml's `module type` signatures map directly to Rust `trait` definitions
- How OCaml's first-class modules (`(module M : MONOID with type t = a)`) translate to Rust trait bounds (`<M: Monoid>`)
- How `List.fold_left` maps to `Iterator::fold` with `M::empty()` as the accumulator
- How newtype wrappers (`Sum(i64)`, `Product(i64)`) disambiguate multiple `Monoid` impls for the same underlying type

## OCaml Approach

OCaml uses a `module type MONOID` as a typeclass specification, and passes concrete module implementations as first-class values to `concat_all`. The function is polymorphic over any module satisfying the signature, using `List.fold_left` with `M.empty` as the zero value.

## Rust Approach

Rust uses a `trait Monoid` with two methods: `empty() -> Self` (the identity) and `combine(self, other: Self) -> Self` (the operation). `concat_all` is generic over `M: Monoid` and uses `Iterator::fold`. Newtype wrappers (`Sum`, `Product`) let the same base type (`i64`) carry different monoidal structure.

## Key Differences

1. **Module vs trait:** OCaml bundles type + values into a module; Rust separates type identity from trait implementation.
2. **First-class modules vs generics:** OCaml passes modules as runtime values; Rust resolves monomorphization at compile time via trait bounds.
3. **Identity element:** OCaml's `M.empty` is a module field; Rust's `M::empty()` is an associated function — both are zero-argument.
4. **Newtype pattern:** Rust needs wrapper types (`Sum(i64)`) to implement the same trait differently for the same primitive; OCaml uses separate `module Sum` definitions.
