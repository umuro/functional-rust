# 798. Kadane's Algorithm: Maximum Subarray Sum

**Difficulty:** 3  **Level:** Advanced

Find the contiguous subarray with the maximum sum in O(n) — and recover its exact start and end indices.

## The Problem This Solves

The maximum subarray problem appears in financial analysis (find the period of greatest cumulative gain in a price series), image processing (find the brightest rectangular region in a 2D intensity map — Kadane in 2D), signal processing (find the strongest burst in a noisy signal), and genomics (find the region of highest GC-content in a sequence).

Despite looking simple, the problem has subtle edge cases (all-negative arrays, single elements) that naive brute-force O(n²) solutions handle poorly. Kadane's algorithm solves it exactly in a single pass, making it suitable for streaming data where you can't store the full array.

## The Intuition

At each position, you have a choice: extend the current subarray, or start fresh. If `current_sum + x < x`, it means the current prefix is dragging you down — drop it and start a new subarray at `x`. Track the best seen so far. That's it. The entire algorithm fits in one conditional per element. OCaml expresses this elegantly as a fold over the array with an accumulator tuple; Rust uses a `for` loop with `enumerate` to also track indices.

## How It Works in Rust

```rust
// O(n) time, O(1) space — single pass
fn max_subarray(arr: &[i64]) -> (i64, usize, usize) {
    assert!(!arr.is_empty(), "array must be non-empty");
    let mut best_sum   = arr[0];
    let mut best_start = 0;
    let mut best_end   = 0;
    let mut curr_sum   = arr[0];
    let mut curr_start = 0;

    for (i, &x) in arr.iter().enumerate().skip(1) {
        if x > curr_sum + x {
            // Starting fresh is better than extending
            curr_sum   = x;
            curr_start = i;
        } else {
            curr_sum  += x;
        }
        if curr_sum > best_sum {
            best_sum   = curr_sum;
            best_start = curr_start;
            best_end   = i;
        }
    }
    (best_sum, best_start, best_end)
}

// Example: [-2, 1, -3, 4, -1, 2, 1, -5, 4]
// Answer:  sum=6, subarray=[4, -1, 2, 1] at indices 3..6
```

The condition `x > curr_sum + x` is equivalent to `curr_sum < 0` but avoids the separate negative-sum check. For all-negative arrays, this correctly returns the least-negative single element. The index tracking adds four extra variables but no asymptotic cost.

## What This Unlocks

- **Time-series analysis**: find the interval of maximum cumulative return in a sequence of daily price changes — directly applicable to backtesting trading strategies.
- **2D max subarray**: the standard generalisation fixes each pair of rows, runs Kadane on the column sums — O(n²m) for an n×m matrix. Used in image segmentation and 2D pattern detection.
- **Streaming algorithms**: Kadane processes one element at a time with O(1) state, making it ideal for sliding-window variants and online data processing where the array doesn't fit in memory.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iteration | `Array.fold_left` with tuple accumulator | `for (i, &x) in arr.iter().enumerate()` |
| Start fresh condition | `x > curr + x` or `curr < 0` | Same condition, identical semantics |
| Index tracking | Extra `ref` variables in fold | Named mutable variables `curr_start`, `best_end` |
| All-negative case | Returns single element (same logic) | Same: `best_sum = arr[0]` initialisation handles it |
| Return type | Tuple `(sum, start, end)` | Tuple `(i64, usize, usize)` |
