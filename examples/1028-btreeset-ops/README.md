📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1028-btreeset-ops)**

---

# 1028-btreeset-ops — BTreeSet Operations
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Set operations — union, intersection, difference, and symmetric difference — are mathematical primitives used throughout computing: dependency resolution (which packages are in both A and B?), access control (union of permissions), deduplication, and set-based query optimization in databases.

Rust's `BTreeSet` provides sorted sets with efficient O(n) set operations on sorted sequences and O(log n) membership tests. The sorted ordering guarantees deterministic iteration order, making it suitable for reproducible output.

## Learning Outcomes

- Perform union, intersection, difference, and symmetric difference on `BTreeSet`
- Understand that `BTreeSet` iterates in sorted order
- Check subset, superset, and disjoint relationships
- Use set operations as iterators for lazy evaluation
- Choose `BTreeSet` vs `HashSet` based on ordering requirements

## Rust Application

`src/lib.rs` demonstrates `union`, `intersection`, `difference`, and `symmetric_difference` — all returning iterators over borrowed elements. The results are collected into `Vec<i32>` to show the sorted order. `subset_checks` shows `is_subset`, `is_superset`, and `is_disjoint`. The iterator-based API means you can chain set operations without materializing intermediate collections.

Set operations in the standard library run in O(n) time by merging two sorted iterators — more efficient than building a `HashMap` for each side.

## OCaml Approach

OCaml's `Set.Make(Ord)` provides the same operations:

```ocaml
module IntSet = Set.Make(Int)

let union a b = IntSet.union a b
let inter a b = IntSet.inter a b
let diff a b = IntSet.diff a b
```

OCaml's sets are persistent — operations return new sets using structural sharing. Rust's `BTreeSet` is mutable, so operations produce new sets by cloning.

## Key Differences

1. **Persistence**: OCaml's `Set` is persistent via structural sharing; Rust's `BTreeSet` requires `.clone()` to preserve the original.
2. **Iterator API**: Rust's set operations return iterators, enabling lazy chaining; OCaml's operations return new sets immediately.
3. **No `HashSet` in OCaml stdlib**: OCaml's standard library only has sorted sets; `Hashtbl`-based sets exist in `Base` and `Core`.
4. **Ownership of elements**: Rust's set operation iterators yield references; collecting copies requires explicit `.copied()` or `.cloned()`.

## Exercises

1. Write a `multi_union(sets: Vec<BTreeSet<i32>>) -> BTreeSet<i32>` function that unions any number of sets.
2. Implement a simple dependency resolver: given a map of package -> direct dependencies, compute the full transitive closure using repeated set unions.
3. Use `symmetric_difference` to find words that appear in one text but not the other, implementing a simple diff tool for word sets.
