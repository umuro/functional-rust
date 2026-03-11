# Example 1109: Monoid Pattern — Generic Combining

**Difficulty:** ⭐⭐
**Category:** Typeclasses & Traits | Newtype Pattern | Generic Abstractions
**OCaml Source:** Classic functional programming — algebraic structure

## Problem Statement

Define a `Monoid` abstraction (identity element + associative binary operation) and implement `concat_all`, a function that reduces any list to a single value using a monoid instance — without knowing in advance whether it is summing integers, multiplying them, concatenating strings, or folding booleans.

## Learning Outcomes

- How OCaml's `module type` (typeclass) maps directly to a Rust `trait`
- Why the **newtype pattern** is necessary in Rust when one type has multiple valid Monoid instances (e.g., `i32` for both Sum and Product)
- How `IntoIterator` generalises `List.fold_left` to work with any sequence type
- How monoid laws (identity, associativity) are conventions that traits cannot enforce, but tests can verify

## OCaml Approach

OCaml uses a **module type** (`MONOID`) to describe the interface, then passes concrete modules (`Sum`, `Product`, `Concat`, `All`) as first-class values to `concat_all`. This is OCaml's form of ad-hoc polymorphism: the same function works for every module satisfying the signature, and different modules can supply different semantics for the same underlying type (`int` for both `Sum` and `Product`).

## Rust Approach

Rust uses a **trait** (`Monoid`) in place of the module type. `concat_all` is generic over `M: Monoid` and calls `M::empty()` and `M::combine()`. Because Rust allows only one trait implementation per type, the **newtype pattern** (`Sum(i32)`, `Product(i32)`, `All(bool)`) gives each behaviour its own distinct type identity. `String` has only one natural concatenation monoid, so it is implemented directly without a newtype.

## Key Differences

1. **Module type vs trait:** OCaml's `module type MONOID` and Rust's `trait Monoid` express identical constraints — identity + combine — but the dispatch mechanism differs (explicit module argument vs type-inferred trait bound).
2. **Multiple instances per type:** OCaml freely permits `module Sum` and `module Product` over the same `int`; Rust requires distinct newtypes (`Sum(i32)` vs `Product(i32)`) because a type can have at most one implementation of any given trait.
3. **Fold vs recursion:** `List.fold_left` is idiomatic OCaml; Rust's `Iterator::fold` is its exact counterpart, but works over any `IntoIterator`, not only lists.
4. **Ownership in `combine`:** OCaml's `combine : t -> t -> t` passes by value naturally; Rust's `fn combine(self, other: Self) -> Self` expresses the same ownership semantics — both arguments are consumed.
