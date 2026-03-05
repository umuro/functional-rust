# Example 995: Set.Make — Immutable Sorted Set

**Difficulty:** ⭐⭐
**Category:** stdlib-set
**OCaml Source:** OCaml standard library `Set.Make` functor

## Problem Statement

Create and manipulate immutable, ordered sets supporting union, intersection,
difference, membership, add, remove, filter, map, and fold — mirroring the
OCaml `Set.Make` functor interface in idiomatic Rust.

## Learning Outcomes

- `BTreeSet<T>` is the direct Rust equivalent of OCaml's `Set.Make` — both
  are ordered, balanced BST sets with O(log n) operations.
- Rust set operations (`union`, `intersection`, `difference`) return iterators;
  you collect them into a new set, matching OCaml's persistent (immutable) style.
- "Immutable add/remove" in Rust means `clone` + mutate — the original binding
  is unchanged because ownership is separate from the data structure.
- `BTreeSet::iter()` yields elements in ascending order, just like
  `Set.elements` returns a sorted list.

## OCaml Approach

OCaml uses the `Set.Make` functor to produce a typed set module for a given
ordered type. All operations (`union`, `inter`, `diff`, `add`, `remove`,
`filter`, `map`, `fold`) return new sets — the original is never mutated.
The internal representation is a balanced AVL tree shared structurally between
versions (persistent data structure with path copying).

## Rust Approach

Rust's `std::collections::BTreeSet<T>` is a mutable B-tree set. To emulate
OCaml's persistent semantics, wrapper functions `clone` the set before
modifying it. Iterator-based set operations (`union()`, `intersection()`,
`difference()`) take references and return lazy iterators — calling `.collect()`
materialises the new set without touching the originals.

## Key Differences

1. **Persistence model:** OCaml sets share structure (path copying); Rust
   `BTreeSet` is fully cloned — O(n) cost vs OCaml's O(k log n) for add/remove.
2. **Functor vs generics:** `Set.Make(Int)` specialises the module at compile
   time; Rust uses `BTreeSet<T>` with the `Ord` trait bound — same static
   dispatch, different syntax.
3. **Iteration:** OCaml `Set.fold` is right-to-left by default; Rust
   `BTreeSet::iter().fold()` is left-to-right (ascending order).
4. **`map` semantics:** OCaml's `Set.map` guarantees a valid set even when `f`
   is not injective (duplicates are silently merged); Rust `map_set` does the
   same because collecting into `BTreeSet` deduplicates automatically.
