📖 **[View on hightechmind.io →](https://hightechmind.io/rust/260-functor-comparable-set)**

---

# Example 260: Functor Comparable Set

**Difficulty:** ⭐⭐⭐
**Category:** Functors and Modules
**OCaml Source:** Real World OCaml — Functors chapter

## Problem Statement

Build a generic, deduplicated, ordered set from any type that supports comparison, mirroring OCaml's `Set.Make` functor pattern that produces a new module from a `COMPARABLE` module argument.

## Learning Outcomes

- How OCaml *functors* (module-level functions parameterised by modules) map to Rust *generic structs* with trait bounds
- Using `Ord` as the Rust analogue of OCaml's `COMPARABLE` module type
- Maintaining a sorted, deduplicated `Vec` with `binary_search` for O(log n) lookup
- Builder-style method chaining (`insert` returns `Self`) to replicate OCaml's pipe-operator idiom

## OCaml Approach

OCaml defines a `COMPARABLE` module signature with a `compare` function, then uses a functor `MakeSet(C : COMPARABLE)` to produce a new module that contains an ordered set for type `C.t`. The resulting `IntSet` and `StringSet` modules are completely separate types, each with their own `empty`, `mem`, `add`, and `to_list` functions.

## Rust Approach

Rust replaces the functor with a generic struct `ComparableSet<T: Ord>`. The `Ord` trait bound plays the role of the `COMPARABLE` module type — any type implementing `Ord` (including all primitives and `String`) can be used. A second struct `FunctorSet<T: Ord>` replicates OCaml's original list-based strategy (O(n) insert, sort on `to_list`) to show the direct translation.

## Key Differences

1. **Abstraction mechanism:** OCaml uses *functor application* (`MakeSet(Int)`) to produce a new module at the module level; Rust uses *monomorphisation* of a generic struct at compile time.
2. **Trait vs module type:** `COMPARABLE` is an OCaml module type specifying `compare`; Rust uses the built-in `Ord` trait which every numeric type and `String` already implements.
3. **Mutability:** OCaml's `add` returns a new value (functional update); Rust's `insert` consumes `self` and returns `Self`, encoding the same immutable-update pattern without hidden clones.
4. **Performance:** The idiomatic Rust `ComparableSet` uses a sorted `Vec` with `binary_search` for O(log n) membership test; the OCaml original uses `List.exists` — O(n) — and sorts on `to_list`.
