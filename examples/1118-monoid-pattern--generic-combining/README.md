# Example 1118: Monoid Pattern — Generic Combining

**Difficulty:** ⭐⭐
**Category:** Type Classes & Abstractions
**OCaml Source:** Real World OCaml — first-class modules as type classes

## Problem Statement

Implement a generic `concat_all` function that folds any list using its monoid instance
(identity element + associative binary operation), parameterized over Sum, Product,
string Concat, and boolean All.

## Learning Outcomes

- How Rust traits encode type classes (here: `Monoid`) that OCaml encodes with module signatures
- Using `fold` as the canonical way to collapse a collection via a monoid
- Newtype wrappers (`Sum`, `Product`, …) to provide multiple monoid instances for the same base type
- Trait bounds in generic functions replace OCaml's first-class module passing

## OCaml Approach

OCaml defines a `MONOID` module signature and accepts first-class modules at call sites:
`concat_all (module Sum) [1;2;3;4;5]`. Separate modules (`Sum`, `Product`, `Concat`, `All`)
implement the signature, and `List.fold_left` does the combining.

## Rust Approach

Rust defines a `Monoid` trait with `empty()` and `combine()`. Newtype structs
(`Sum(i64)`, `Product(i64)`, `Concat(String)`, `All(bool)`) each `impl Monoid`.
`concat_all<T: Monoid>` uses `Iterator::fold` with `T::empty()` as the seed—no
runtime dispatch, no allocation, zero overhead.

## Key Differences

1. **Module vs Trait:** OCaml passes the implementation as a first-class module argument; Rust resolves it via the generic type parameter at compile time.
2. **Multiple instances per type:** OCaml creates distinct modules; Rust uses newtype wrappers to avoid conflicting impls on `i64`.
3. **Identity element:** OCaml stores `empty` as a module value; Rust uses an associated function `fn empty() -> Self`.
4. **Recursion vs iterator:** OCaml's `fold_left` is a library function; Rust's `Iterator::fold` works on any `IntoIterator`, accepting slices, arrays, or lazy chains.
