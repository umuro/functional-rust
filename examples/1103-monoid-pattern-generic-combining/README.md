# Example 1103: Monoid Pattern — Trait-Based Combining

**Difficulty:** ⭐⭐
**Category:** Type Classes & Abstractions
**OCaml Source:** `example.ml` — first-class module `MONOID` with `concat_all` via `List.fold_left`

## Problem Statement

Define a `Monoid` abstraction — a type with an identity element and an associative binary operation — and implement `concat_all` that reduces any collection of monoid values into a single result. The pattern captures a broad family of algorithms (summation, string concatenation, boolean conjunction, list merging) under one interface, demonstrating how algebraic abstractions eliminate repetition across types.

## Learning Outcomes

- How OCaml's `module type MONOID` (a named signature) translates to a Rust `trait` with associated functions
- How `concat_all<M: Monoid>` expresses the same generic fold over any monoid-implementing type using Rust's trait bounds
- Why Rust infers the monoid implementation from the element type while OCaml requires passing the module explicitly at the call site
- How `i32` (addition monoid, identity = 0) and `String` (concatenation monoid, identity = `""`) share exactly the same `concat_all` call path
- What the mathematical monoid laws (identity and associativity) guarantee about `concat_all` behavior on any conforming type

## OCaml Approach

OCaml defines `module type MONOID` as a signature with `type t`, `val empty : t`, and `val combine : t -> t -> t`. Concrete monoids are plain modules (`Sum`, `Product`, `Concat`, `All`) satisfying that signature. The generic function `concat_all` uses a locally abstract type `(type a)` to accept a first-class module at the call site: `concat_all (module Sum) [1;2;3;4;5]`. The implementation body is just `List.fold_left M.combine M.empty lst` — identical in structure to the Rust version. OCaml's approach makes the monoid dictionary explicit and first-class; callers choose which monoid to use by passing the module directly.

## Rust Application

`trait Monoid { fn empty() -> Self; fn combine(self, other: Self) -> Self; }` is the entire interface. `concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M` implements the fold as `items.into_iter().fold(M::empty(), M::combine)` — a one-liner that directly mirrors the mathematical definition. Implementations for `i32` and `String` live in the test module. The monoid implementation is resolved statically from the element type; no explicit dictionary is needed at the call site. This is Rust's standard zero-cost abstraction: the compiler monomorphizes one concrete version per type at compile time.

## Key Differences

1. **Polymorphism mechanism:** OCaml passes modules as first-class values at runtime (explicit dictionary); Rust resolves trait implementations at compile time via monomorphization (zero runtime overhead)
2. **Identity access:** OCaml uses `M.empty` via the module record; Rust uses `M::empty()` as an associated function — both are per-type, but Rust's is resolved statically
3. **Call site syntax:** OCaml explicitly passes `(module Sum)` to choose the monoid; Rust infers `M` from the collection's element type, so no annotation is needed when the type is unambiguous
4. **Higher-kinded abstraction:** OCaml's `(type a) (module M : MONOID with type t = a)` expresses type-level constraints inline; Rust uses `where M: Monoid` in a separate clause, achieving the same constraint with different syntax

## Exercises

1. Implement `Monoid` for `Vec<T>` where `empty()` returns `vec![]` and `combine` is concatenation — verify `concat_all` merges a list of vecs correctly
2. Implement a `Product` newtype over `i32` (identity = 1, combine = multiplication) and confirm `concat_all([1i32, 2, 3, 4, 5])` gives 120
3. Implement `Monoid` for `bool` with two distinct monoids: one using `&&` (identity = `true`) and one using `||` (identity = `false`) — use newtypes to distinguish them
4. Prove the monoid laws hold for your `String` implementation: check that `combine(empty(), s) == s`, `combine(s, empty()) == s`, and associativity for three strings
