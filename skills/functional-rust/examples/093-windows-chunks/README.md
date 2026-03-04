# 093: Windows and Chunks

**Difficulty:** 3  **Level:** Intermediate

`.windows(n)` gives overlapping slices of size n; `.chunks(n)` gives non-overlapping. Both are zero-copy.

## The Problem This Solves

Many algorithms need to look at multiple adjacent elements together: moving averages, pattern matching, batch processing, signal analysis. Writing these manually with index arithmetic is tedious and easy to get wrong at the boundaries.

Rust bakes both patterns directly into slices. `.windows(n)` slides a view of size `n` across the data one step at a time — perfect for comparing neighbors. `.chunks(n)` cuts the data into non-overlapping blocks — perfect for batch work.

Both yield `&[T]` references, so there's no copying. OCaml has neither built-in; you'd implement them with `Array.sub` in O(n²) or write a recursive list version.

## The Intuition

Imagine laying a ruler of length `n` across your data:
- **windows**: slide it one position at a time → consecutive overlapping views.
- **chunks**: jump by `n` each time → non-overlapping blocks.

For a list `[1,2,3,4,5]` with `n=3`:
- `windows(3)` → `[1,2,3]`, `[2,3,4]`, `[3,4,5]` (3 windows)
- `chunks(3)` → `[1,2,3]`, `[4,5]` (last chunk may be short)
- `chunks_exact(3)` → `[1,2,3]` only (remainder accessible via `.remainder()`)

## How It Works in Rust

```rust
// Moving average: each window of size n → one average
fn moving_average(data: &[f64], n: usize) -> Vec<f64> {
    data.windows(n)
        .map(|w| w.iter().sum::<f64>() / n as f64)
        .collect()
}

// Consecutive difference: compare each element to its neighbor
fn pairwise_diff(data: &[i32]) -> Vec<i32> {
    data.windows(2).map(|w| w[1] - w[0]).collect()
}

// Local maxima: middle element is larger than both neighbors
fn local_maxima(data: &[i32]) -> Vec<i32> {
    data.windows(3)
        .filter(|w| w[1] > w[0] && w[1] > w[2])
        .map(|w| w[1])
        .collect()
}

// Batch processing: sum each non-overlapping chunk
fn chunk_sums(data: &[i32], size: usize) -> Vec<i32> {
    data.chunks(size).map(|c| c.iter().sum()).collect()
}

// chunks_exact guarantees full-size chunks; .remainder() holds leftovers
fn process_full_batches(data: &[i32], size: usize) {
    let chunks = data.chunks_exact(size);
    let leftover = chunks.remainder(); // &[T] of remaining elements
    for chunk in chunks { /* always exactly `size` elements */ }
}
```

## What This Unlocks

- **Signal processing**: moving average, moving standard deviation, peak detection.
- **Pattern search**: `data.windows(pattern.len()).any(|w| w == pattern)`.
- **Batch APIs**: split a large payload into chunks and send each independently.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sliding window | Manual `Array.sub` | `.windows(n)` built-in |
| Non-overlapping | Manual recursion | `.chunks(n)` built-in |
| Guaranteed size | Filter by length | `.chunks_exact(n)` + `.remainder()` |
| Element type | List of lists (copies) | Iterator of `&[T]` (zero-copy) |
| Complexity | O(n²) with copies | O(n) with slice references |
