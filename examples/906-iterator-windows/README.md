📖 **[View on hightechmind.io →](https://hightechmind.io/rust/906-iterator-windows)**

---

# 906-iterator-windows — Iterator Windows
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Signal processing, financial time-series analysis, and machine learning feature engineering all use sliding window operations: compute a statistic over each overlapping sub-window of a sequence. A window of size k over n elements produces n-k+1 windows. Naive implementation requires O(k) work per window; efficient windowed algorithms precompute partial results. Rust's `.windows(n)` on slices yields zero-copy overlapping sub-slices in O(1) per window, enabling cache-friendly windowed computation. OCaml lacks a built-in equivalent and requires manual implementation.

## Learning Outcomes

- Use `.windows(k)` to produce overlapping zero-copy sub-slice references
- Compute moving averages using `windows(k).map(|w| sum/k)`
- Detect monotonic subsequences using `windows(2).all(|w| w[0] < w[1])`
- Find local maxima using `windows(3).filter(|w| w[1] > w[0] && w[1] > w[2])`
- Extract bigrams (consecutive pairs) using `windows(2).map(|w| (&w[0], &w[1]))`

## Rust Application

`moving_average` maps each window of size `k` to its average using `w.iter().sum::<i32>() as f64 / k as f64`. `is_strictly_increasing` uses `windows(2).all(|w| w[0] < w[1])` — elegant one-liner. `local_maxima` uses `windows(3).enumerate().filter(|(_, w)| w[1] > w[0] && w[1] > w[2]).map(|(i, _)| i + 1)` — returning indices into the original slice (+1 offset for the window center). `bigrams` extracts consecutive pairs as references. All operations are zero-copy — windows borrow from the original slice.

## OCaml Approach

OCaml arrays support `Array.sub arr i k` for windowed access: `Array.init (n - k + 1) (fun i -> f (Array.sub arr i k))`. For lists, recursion is required: `let rec windows k = function | lst when List.length lst < k -> [] | lst -> List.filteri (fun i _ -> i < k) lst :: windows k (List.tl lst)`. This is O(n²) for lists. The `Bigarray` module provides stride-based views for numerical code.

## Key Differences

1. **Zero-copy**: Rust `.windows(n)` yields references into the original slice — zero allocation per window; OCaml `Array.sub` allocates a new array per window.
2. **Bounds guarantee**: Rust windows always have exactly `n` elements; OCaml `Array.sub` can raise `Invalid_argument` on incorrect bounds.
3. **List vs slice**: OCaml lists cannot provide O(1) windows; Rust slice windows are O(1) due to contiguous memory layout.
4. **Enumerate + index**: Rust can combine windows with enumerate to get the window's position; OCaml's functional approach loses positional information.

## Exercises

1. Implement `max_subarray_sum(data: &[i32], k: usize) -> i32` that finds the maximum sum over all windows of size k.
2. Write `trend_labels(prices: &[f64]) -> Vec<&str>` using `windows(2)` that labels each transition as "up", "down", or "flat".
3. Implement `autocorrelation(data: &[f64], lag: usize) -> f64` using two windows offset by `lag` to compute the correlation.
