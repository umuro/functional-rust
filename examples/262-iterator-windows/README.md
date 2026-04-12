📖 **[View on hightechmind.io →](https://hightechmind.io/rust/262-iterator-windows)**

---

# 262: Sliding Windows over Slices
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Signal processing, time-series analysis, and pattern recognition algorithms frequently examine overlapping subsequences of fixed length. A moving average over stock prices, an n-gram language model, or detecting whether an array is sorted — all require looking at consecutive groups of elements simultaneously. The `windows(n)` method provides zero-copy, overlapping sub-slices of a fixed length, making these algorithms expressible as simple iterator pipelines.

## Learning Outcomes

- Understand how `windows(n)` yields overlapping sub-slices of length `n`
- Distinguish `windows()` from `chunks()`: overlapping vs non-overlapping
- Compute sliding averages, detect sorted order, and find patterns using `windows()`
- Recognize the zero-copy nature: each window is a borrowed sub-slice, not a new allocation

## Rust Application

`slice::windows(n)` returns a `Windows<'_, T>` iterator yielding `&[T]` sub-slices. Each successive window advances by one position. For a slice of length `m`, there are `m - n + 1` windows:

```rust
let data = [1i32, 2, 3, 4, 5];
// Moving average with window size 2
let avgs: Vec<f64> = data.windows(2)
    .map(|w| w.iter().sum::<i32>() as f64 / 2.0)
    .collect();
// [1.5, 2.5, 3.5, 4.5]

// Detect sorted order: every adjacent pair must be non-decreasing
let is_sorted = data.windows(2).all(|w| w[0] <= w[1]); // true
```

## OCaml Approach

OCaml lacks a standard `windows` function on lists. The idiomatic functional approach uses `List.init` and `List.sub` on arrays, or defines a recursive function that takes the head as a sub-list:

```ocaml
let windows n lst =
  let arr = Array.of_list lst in
  Array.init (Array.length arr - n + 1) (fun i ->
    Array.to_list (Array.sub arr i n))
```

This allocates new sub-arrays; Rust's `windows()` avoids this via slice references.

## Key Differences

1. **Zero-copy**: Rust's `windows()` yields borrowed `&[T]` slices with no allocation; OCaml's equivalent creates new sub-lists.
2. **Overlap**: `windows(n)` is overlapping; `chunks(n)` is non-overlapping — two complementary methods in Rust.
3. **Standard library**: `windows()` is a stable part of Rust's slice API; OCaml requires user-defined functions.
4. **Use cases**: Signal processing (DSP), n-gram text analysis, financial moving averages, sorted-order detection.

## Exercises

1. Use `windows(3)` to compute a 3-element moving average over a `Vec<f64>` of temperature readings.
2. Detect local maxima in a sequence: an element is a local maximum if it is strictly greater than its neighbors — use `windows(3)`.
3. Find the starting index of a pattern (sub-slice) within a larger slice using `windows()` and `position()`.
