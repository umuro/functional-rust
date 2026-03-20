# List.sort — Sort with Custom Comparator

## Problem Statement
Sort a list of strings in three ways: lexicographically, by length, and in descending order. Implement a generic `sort_with` that accepts a custom comparator, mirroring OCaml's `List.sort`.

## Learning Outcomes
- How OCaml's `List.sort cmp xs` maps to Rust's `slice::sort_by(|a, b| cmp(a, b))`
- Pass comparison functions as closures in both languages
- Chain comparators: sort by length first, then alphabetically to break ties

## Rust Application
`sort_by` takes a closure returning `std::cmp::Ordering`. Chaining comparisons uses `.then()`: `a.len().cmp(&b.len()).then(a.cmp(b))` sorts by length first, then alphabetically. The generic `sort_with<T: Clone, F>` clones the input slice to return a new sorted `Vec`.

## OCaml Approach
`List.sort String.compare words` uses the built-in string comparison. `List.sort (fun a b -> compare (String.length a) (String.length b)) words` uses an anonymous function. OCaml's sort is guaranteed stable; Rust's `sort_by` is also stable.

## Key Differences
1. **In-place vs. persistent:** OCaml's `List.sort` returns a new sorted list (lists are immutable); Rust `sort_by` sorts in place — the `sort_with` wrapper clones first to match OCaml semantics
2. **Comparator type:** OCaml comparators return `int` (negative/zero/positive); Rust uses `std::cmp::Ordering` enum — same semantics, different representation
3. **Stability:** Both `List.sort` and Rust's `sort_by` are stable (preserve relative order of equal elements)

## Exercises
1. Implement `sort_by_key` that takes a key extraction function `f: Fn(&T) -> K` where `K: Ord`, mirroring OCaml's `List.sort_uniq`
2. Sort a list of `(String, i32)` pairs: first by the integer descending, then by string ascending
3. Implement `is_sorted` that checks whether a slice is already sorted with respect to a given comparator
