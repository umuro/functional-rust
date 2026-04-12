# Example 1193: List.sort — Sort with Custom Comparator

**Difficulty:** ⭐⭐
**Category:** Lists & HOF
**OCaml Source:** OCaml standard library `List.sort`

## Problem Statement

Sort a collection using a custom comparison function, enabling flexible ordering without duplicating traversal logic. This example sorts strings three ways — lexicographically, by length, and in descending order — and implements both the standard library wrapper and a from-scratch merge sort to show the algorithm's recursive structure.

## Learning Outcomes

- How OCaml's `List.sort cmp xs` maps to Rust's `slice::sort_by(|a, b| cmp(a, b))` and why the comparator return type differs (`int` vs. `Ordering`)
- How to chain sorting criteria in Rust using `.then()` on `Ordering` values — equivalent to OCaml's `compare (key a) (key b)` pattern
- Why Rust's `sort_by` is in-place while OCaml's `List.sort` returns a new list, and how a clone-first wrapper reconciles the two models
- How merge sort's divide-and-conquer structure mirrors a purely recursive OCaml implementation

## OCaml Approach

OCaml's `List.sort : ('a -> 'a -> int) -> 'a list -> 'a list` accepts a comparison function returning a negative integer, zero, or positive integer (the C `qsort` convention). `String.compare` and the polymorphic `compare` are passed directly as first-class function values. Because OCaml lists are immutable, `List.sort` always returns a new sorted list without modifying the original. The sort is guaranteed stable.

## Rust Approach

`slice::sort_by` mutates in place, so the generic `sort_with` wrapper clones the input slice first to match OCaml's value-returning semantics. The comparator must return `std::cmp::Ordering` (an enum with `Less`, `Equal`, `Greater`) rather than a raw integer. Chaining criteria uses the `.then()` combinator: `a.len().cmp(&b.len()).then(a.cmp(b))` sorts by length first and falls back to lexicographic order for ties. `sort_by` is stable, matching OCaml's guarantee.

## Key Differences

1. **Comparator return type:** OCaml comparators return `int` (negative/zero/positive) following the C convention; Rust uses the `Ordering` enum — the same three-way semantics expressed as a type-safe value.
2. **In-place vs. persistent:** OCaml's `List.sort` returns a new list because lists are immutable; Rust's `sort_by` sorts the slice in place, requiring an explicit `.to_vec()` clone for value-returning behavior.
3. **Composing comparators:** OCaml composes comparators by nesting: `if c = 0 then compare ...`; Rust uses `.then()` on `Ordering` for a fluent, expression-oriented chain with no if-expression needed.
4. **Stability:** Both `List.sort` and Rust's `sort_by` are guaranteed stable; equal elements preserve their original relative order.
