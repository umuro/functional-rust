# 789. Longest Increasing Subsequence

**Difficulty:** 4  **Level:** Advanced

Find the longest strictly increasing subsequence in O(n log n) using patience sorting — with full reconstruction via predecessor tracking.

## The Problem This Solves

LIS appears wherever you need to find the longest "compatible chain" in a sequence with order constraints. Stack scheduling (find the maximum number of non-overlapping intervals when sorted by end time), box stacking problems (stack boxes where each dimension is strictly smaller), and activity selection with precedence constraints all reduce to LIS. In bioinformatics, finding the longest chain of non-crossing gene matches between two genomes is an LIS problem after sorting.

The naive O(n²) DP approach works for small inputs, but the patience-sorting based O(n log n) algorithm is what you'd deploy in production on large datasets — and it's the one worth understanding deeply.

## The Intuition

Imagine dealing cards into piles: each new card goes on the leftmost pile whose top card is ≥ the new card, or starts a new pile if none exists. The number of piles at the end equals the LIS length. This is patience sorting. The key insight: maintain a `tails` array where `tails[i]` holds the smallest possible tail value of any increasing subsequence of length `i+1`. Binary search finds the right insertion point in O(log n). OCaml would write a recursive descent with a `ref array`; Rust uses `Vec::partition_point` — the idiomatic binary search that returns the first index where the predicate fails.

## How It Works in Rust

```rust
// O(n log n) — patience sorting with partition_point binary search
fn lis_length(arr: &[i64]) -> usize {
    let mut tails: Vec<i64> = Vec::new();
    for &x in arr {
        // Find first index where tails[i] >= x (lower bound)
        let pos = tails.partition_point(|&t| t < x);
        if pos == tails.len() {
            tails.push(x);      // x extends the longest sequence
        } else {
            tails[pos] = x;     // x replaces: keeps tail value as small as possible
        }
    }
    tails.len()
}

// Reconstruction: track predecessor indices
fn lis_reconstruct(arr: &[i64]) -> Vec<i64> {
    let mut tails: Vec<i64> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();        // tails[k] came from arr[idx[k]]
    let mut pred: Vec<Option<usize>> = vec![None; n]; // predecessor of each element

    for (i, &x) in arr.iter().enumerate() {
        let pos = tails.partition_point(|&t| t < x);
        // Update tails and idx...
        pred[i] = if pos > 0 { Some(idx[pos - 1]) } else { None };
    }

    // Walk predecessor chain backwards, then reverse
    let mut k = idx[tails.len() - 1];
    loop {
        result.push(arr[k]);
        match pred[k] { Some(p) => k = p, None => break }
    }
    result.reverse();
    result
}
```

`partition_point` is Rust's cleaner spelling of `lower_bound`. The `tails` array is *not* the LIS itself — it's a running optimistic summary. The reconstruction via `pred` (predecessor) array recovers the actual sequence.

## What This Unlocks

- **Sequence scheduling**: find the maximum number of non-overlapping tasks, sorted jobs, or compatible intervals — all reduce to LIS after preprocessing.
- **Box/container stacking**: stack N-dimensional boxes maximising stack height; each dimension constraint becomes a sort key, turning it into LIS.
- **Patience sort connection**: the full patience sorting algorithm (which finds the LIS) also provides an O(n log n) sort strategy, linking combinatorics to practical sorting.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Binary search | `Array.blit` + manual bisect | `Vec::partition_point(predicate)` |
| Tails array | Mutable `array ref` | `Vec<i64>` grown with `push` |
| Predecessor tracking | `array` of `option int` | `Vec<Option<usize>>` |
| Reconstruction | Recursive walk | Iterative `loop { match pred[k] }` |
| Strict vs non-strict | `<` vs `<=` in comparison | Change `t < x` to `t <= x` in `partition_point` |
