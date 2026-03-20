📖 **[View on hightechmind.io →](https://hightechmind.io/rust/789-longest-increasing-subsequence)**

---

# 789-longest-increasing-subsequence — Longest Increasing Subsequence

## Problem Statement

The Longest Increasing Subsequence (LIS) problem finds the longest subsequence of a sequence in which all elements are in sorted increasing order. It appears in patience sorting (used in the solitaire card game and by Timsort), scheduling theory, and bioinformatics. The naive O(n²) DP is classic; the O(n log n) solution using binary search and "patience sorting" is a key algorithmic insight showing that binary search applies to DP optimization.

## Learning Outcomes

- Implement the O(n²) DP solution: `dp[i]` = LIS length ending at index i
- Implement the O(n log n) binary search solution using a `tails` array
- Understand the `tails` invariant: `tails[i]` is the smallest tail element of any increasing subsequence of length `i+1`
- Apply `binary_search` to maintain the `tails` array
- Reconstruct the actual LIS by storing predecessor indices

## Rust Application

`lis(arr)` fills `dp[i] = 1 + max(dp[j] for j < i if arr[j] < arr[i])`. `lis_binary_search` maintains `tails: Vec<i32>`. For each element, binary search finds the leftmost position where it can replace a tail; if beyond the end, extend. The `tails.len()` is the LIS length. Tests cover sorted input (LIS = n), reverse sorted (LIS = 1), and the classic `[10,9,2,5,3,7,101,18]` example (LIS = 4).

## OCaml Approach

OCaml implements the O(n²) version functionally with `Array.init n (fun i -> ...)` and the O(n log n) version with a mutable `tails` array and binary search via `Array.blit`. OCaml's standard library includes `Array.blit` for efficient element shifts. The patience sorting metaphor maps naturally: each pile is a `tails` slot, and placing a card uses binary search to find the leftmost pile that accepts it.

## Key Differences

1. **Binary search**: Rust's `Vec::binary_search` returns `Ok(pos)` for exact match and `Err(pos)` for insertion point — exactly what LIS needs; OCaml uses `Array.binary_search` from third-party libraries.
2. **Algorithm clarity**: The `tails` invariant is subtle; both languages benefit from a comment explaining why this works despite `tails` not being the actual LIS.
3. **Reconstruction**: Both languages use an O(n²) predecessor array for reconstruction; it cannot be inferred from `tails` alone.
4. **Timsort connection**: Python's `timsort` uses the patience sorting / LIS insight to identify already-sorted runs; Rust's std sort uses a similar technique.

## Exercises

1. Implement `lis_reconstruct(arr: &[i32]) -> Vec<i32>` that returns the actual LIS (not just its length) by storing predecessor indices during the O(n²) DP.
2. Modify `lis_binary_search` to handle non-strictly-increasing (allowing equal elements) by changing `<` to `<=` in the binary search.
3. Use LIS to count the minimum number of strictly decreasing subsequences needed to partition a sequence (Dilworth's theorem: equals LIS length).
