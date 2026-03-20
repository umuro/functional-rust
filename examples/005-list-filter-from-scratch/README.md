# Example 005: List Filter From Scratch

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions | Lists | Predicates
**OCaml Source:** Functional programming classic; derives `List.filter`

## Problem Statement

Implement a filter function that removes elements from a list that don't satisfy a given predicate (boolean test function). This is the fundamental operation for selecting a subset of elements while preserving order.

## Learning Outcomes

- How to work with predicate functions (`fn(&T) -> bool`) in Rust
- Three approaches to filtering: idiomatic iterators, recursive pattern matching, and fold
- The role of `.clone()` when working with owned collections vs borrowed slices
- How Rust handles higher-order functions (functions that take functions as parameters)
- The trade-off between idiomatic Rust (iterators) and functional style (recursion)

## OCaml Approach

OCaml's `filter` is recursive: it pattern-matches on the head and tail of the list, recursively filters the tail, and then conditionally prepends the head. This naturally preserves list order because the predicate is tested as each element is deconstructed.

## Rust Approach

Rust provides three idiomatic paths:

1. **Iterator chain** (most idiomatic): `items.iter().filter(...).cloned().collect()` — leverages Rust's lazy iterators and standard library
2. **Recursive pattern matching** (closest to OCaml): matches on `[h, rest @ ..]` and recursively calls itself
3. **Fold/accumulate** (functional alternative): uses `fold` to build the result bottom-up

All three preserve order and handle empty lists correctly. The iterator version is preferred in production Rust because it avoids allocations during traversal.

## Key Differences

1. **List representation:** OCaml uses cons lists (`h :: t`); Rust uses slices (`&[T]`)
2. **Recursion safety:** OCaml's immutable recursion is safe by default; Rust requires `fn(&T)` to avoid mutable predicates
3. **Cloning:** Rust must `.clone()` elements when moving them into the result vector; OCaml lists share structure via references
4. **Laziness:** Rust iterators are lazy (elements processed on-demand); fold is eager (processes all elements immediately)

## Exercises

1. Implement `reject` — the inverse of `filter`: keep only elements for which the predicate returns `false`.
2. Write `filter_map` from scratch: apply a function `f: T -> Option<U>` to each element and collect only the `Some` results into a new `Vec<U>`.
3. Implement `partition` from scratch that splits a list into a pair `(Vec<T>, Vec<T>)` — elements satisfying the predicate and those that do not — in a single pass.
