📖 **[View on hightechmind.io →](https://hightechmind.io/rust/252-insertion-sort)**

---

# Example 252: Insertion Sort

**Difficulty:** ⭐
**Category:** Sorting Algorithms
**OCaml Source:** https://rosettacode.org/wiki/Sorting_algorithms/Insertion_sort#OCaml

## Problem Statement

Build a sorted list by inserting each element of an unsorted list into the correct position of an already-sorted accumulator, repeating until all elements are placed.

## Learning Outcomes

- How `fold_left` replaces explicit loops: the OCaml pattern `List.fold_left (fun acc x -> insert x acc) []` maps directly to Rust's `.fold(Vec::new(), |acc, x| ...)`
- In-place mutation vs. allocation: Rust's `data.swap()` approach is zero-allocation; the functional approach allocates a new `Vec` per insertion
- `slice::partition_point` as a binary-search replacement for OCaml's linear scan through a sorted list
- Pattern matching on slice heads (`[h, rest @ ..]`) as a direct translation of OCaml's `h :: t` patterns

## OCaml Approach

OCaml defines a recursive `insert` that pattern-matches on the list head: if `x <= h` the element is prepended before `h`; otherwise `h` is kept and the recursion continues into the tail. `insertion_sort` folds this inserter over an empty accumulator, building the result purely through list construction without mutation.

## Rust Approach

Three implementations show the spectrum. The idiomatic in-place version uses two nested index loops with `slice::swap` — zero allocation, cache-friendly, matching Rust's strengths. The functional version uses `.fold` with `Vec::insert` after a `partition_point` binary search, preserving the OCaml fold shape. The recursive version translates `insert` almost word-for-word using slice pattern matching.

## Key Differences

1. **Mutation model:** OCaml constructs new lists at every step; Rust's idiomatic version mutates a single `Vec` in place.
2. **List head access:** OCaml's `h :: t` destructuring becomes Rust's `[h, rest @ ..]` slice pattern — same idea, different syntax.
3. **Search strategy:** OCaml's `insert` scans linearly; Rust's functional version uses `partition_point` (binary search) on the sorted accumulator for O(log n) comparisons.
4. **Stability:** Both OCaml's `x <= h` guard and Rust's `partition_point(|h| h < &x)` place equal elements in original order — both sorts are stable.
