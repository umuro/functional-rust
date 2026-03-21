# Example 1187: List.fold_left — Accumulate a Result
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Reduce a list to a single value by applying a binary function from left to right, threading an accumulator through the list. Classic uses include sum, product, and maximum.

## Learning Outcomes

- How `Iterator::fold` maps directly to OCaml's `List.fold_left`
- That Rust's `std` provides specialized folds (`.sum()`, `.product()`, `.max()`) that are more ergonomic for common cases
- How to write a generic `fold_left` in Rust with higher-kinded-style function parameters
- Pattern-matching on slices (`[head, tail @ ..]`) to express recursive fold without unsafe code

## OCaml Approach

OCaml's `List.fold_left f acc list` threads `acc` through the list calling `f acc elem` at each step. Operator sections like `( + )` and `( * )` let you pass arithmetic operators as first-class functions. `min_int` serves as the identity for max.

## Rust Approach

Rust's `Iterator::fold(init, |acc, x| ...)` is the direct equivalent. For common reductions, `std` provides `.sum()`, `.product()`, and `.max()` which are cleaner and communicate intent better. A generic `fold_left` wrapper can also be written to mirror the OCaml API exactly.

## Key Differences

1. **Operator sections:** OCaml has `( + )` as a value; Rust uses closures `|a, b| a + b` or trait bounds.
2. **Identity for max:** OCaml uses `min_int`; idiomatic Rust uses `.max()` returning `Option<T>` and avoids the sentinel entirely.
3. **Specialised folds:** Rust's `Sum` and `Product` traits let `.sum()` / `.product()` work generically over numeric types.
4. **Recursive fold:** Rust supports slice pattern `[head, tail @ ..]` for explicit recursion, mirroring OCaml's `x :: xs`.
