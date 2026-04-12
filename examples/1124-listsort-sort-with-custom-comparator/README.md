# Example 1124: List.sort — Sort with Custom Comparator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Sort a list of strings in three different ways: lexicographically, by string length (with ties broken alphabetically), and in descending order. The core challenge is implementing a generic `sort_with` function that accepts a custom comparator, directly mirroring the signature and behavior of OCaml's `List.sort`. This example demonstrates how first-class comparison functions enable flexible, reusable sorting without duplicating traversal logic.

## Learning Outcomes

- How OCaml's `List.sort cmp xs` maps to Rust's `slice::sort_by(|a, b| cmp(a, b))` and why the comparator return type differs
- How to pass comparison closures as arguments in both languages, and the trait bounds (`Fn(&T, &T) -> Ordering`) Rust requires
- How to compose and chain comparators in Rust using `.then()` on `Ordering` — equivalent to the OCaml pattern `compare (key a) (key b)`
- The distinction between in-place mutation (Rust slices) and persistent return values (OCaml lists), and how a clone-first wrapper reconciles the two models
- Why both `List.sort` and Rust's `sort_by` guarantee stability, and what that means for tie-breaking

## OCaml Approach

OCaml's `List.sort` takes a comparison function returning a negative integer, zero, or a positive integer — the same convention as C's `qsort`. For lexicographic order, `String.compare` is passed directly as a first-class function value. For length-based ordering, an anonymous function `fun a b -> compare (String.length a) (String.length b)` extracts the key before delegating to the polymorphic `compare`. Descending order simply reverses the argument order: `fun a b -> String.compare b a`. OCaml lists are immutable, so `List.sort` always returns a new list; no mutation is observable at the call site.

## Rust Application

Rust's `sort_by` mutates the slice in place, so the generic `sort_with<T: Clone, F>` wrapper clones the input first to produce a new `Vec`, matching OCaml's value-returning semantics. The comparator closure must return `std::cmp::Ordering` (an enum with `Less`, `Equal`, `Greater`) rather than a raw integer. Chaining multiple sort criteria uses the `.then()` combinator on `Ordering`: `a.len().cmp(&b.len()).then(a.cmp(b))` sorts by length first and falls back to lexicographic order for ties — expressed in a single expression without branching. Rust's `sort_by` is stable, matching OCaml's guarantee.

## Key Differences

1. **Comparator return type:** OCaml comparators return `int` (negative/zero/positive), following the C convention; Rust uses the `std::cmp::Ordering` enum — same three-way semantics expressed as a typed value rather than a numeric convention.
2. **In-place vs. persistent:** OCaml's `List.sort` returns a new list because lists are immutable; Rust's `sort_by` sorts the slice in place, requiring an explicit `.to_vec()` clone to achieve value-returning behavior.
3. **Comparator chaining:** OCaml typically chains via `if compare_first = 0 then compare_second else compare_first`; Rust provides `.then()` and `.then_with()` on `Ordering`, making multi-key comparisons concise and branch-free.
4. **Trait bounds vs. structural typing:** OCaml accepts any function of the right type automatically; Rust requires the closure to implement `Fn(&T, &T) -> std::cmp::Ordering`, which the compiler enforces at the call site.

## Exercises

1. Implement `sort_by_key` that takes a key extraction function `f: Fn(&T) -> K` where `K: Ord`, mirroring OCaml's common idiom `List.sort (fun a b -> compare (f a) (f b)) xs`. Verify it produces the same output as `sort_with` with an equivalent comparator.
2. Sort a list of `(String, i32)` pairs: first by the integer descending, then by string ascending for ties. Express the comparator using `.then()` chaining rather than an `if` expression.
3. Implement `is_sorted_by` that takes a slice and a comparator and returns `bool` — true if every adjacent pair satisfies `cmp(a, b) != Ordering::Greater`. Write tests covering empty, single-element, ascending, and descending inputs.
