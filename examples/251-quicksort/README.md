# Example 251: Quicksort

**Difficulty:** ⭐⭐
**Category:** Sorting algorithms
**OCaml Source:** https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#OCaml

## Problem Statement

Sort a list of comparable elements using the quicksort algorithm: select a pivot, partition remaining elements into those less than and those greater-or-equal, recurse on each partition, then concatenate.

## Learning Outcomes

- How Rust's slice pattern matching (`[pivot, rest @ ..]`) mirrors OCaml's list destructuring
- The `Clone` bound: when returning new allocations from borrowed input, values must be cloned
- Two valid quicksort styles — functional (allocating, like OCaml) vs in-place (Lomuto partition)
- Why idiomatic Rust prefers `data.sort()` over hand-rolled sorts for production code

## OCaml Approach

OCaml uses recursive list destructuring with a single pivot and `List.partition` to split the tail. The result is built by concatenating sorted sub-lists with `@`. Lists are persistent and immutable, so every call allocates new lists.

## Rust Approach

The functional translation uses slice patterns (`[pivot, rest @ ..]`) and `.partition()` on an iterator, producing `Vec<T>` values at each level. For production use, Rust's `slice::sort` (an introsort variant — hybrid quicksort/heapsort/insertion sort) is preferred. The in-place Lomuto scheme shows how Rust's mutable borrow splitting (`&mut data[..k]`) enables safe recursive in-place algorithms.

## Key Differences

1. **Persistence vs mutation:** OCaml lists are immutable; functional Rust clones into new `Vec`s; idiomatic Rust sorts in place with `&mut [T]`.
2. **Pattern syntax:** OCaml `pivot :: rest` becomes Rust `[pivot, rest @ ..]` on slices.
3. **`Clone` requirement:** Rust must explicitly annotate `T: Clone` when cloning elements out of a borrowed slice; OCaml's GC handles this transparently.
4. **Production sort:** `slice::sort` is O(n log n) worst-case (introsort); the naïve functional quicksort is O(n²) on sorted input.
