📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1055-longest-common-subseq)**

---

# 1055-longest-common-subseq — Longest Common Subsequence
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The longest common subsequence (LCS) finds the longest sequence of characters (not necessarily contiguous) that appears in the same order in two strings. It is the foundation of `diff` tools (git diff, Unix diff), DNA sequence alignment in bioinformatics, and file comparison in version control systems.

The DP solution uses a 2D table where `dp[i][j]` is the LCS length of the first `i` characters of `s1` and the first `j` characters of `s2`, giving O(m×n) time and space.

## Learning Outcomes

- Implement LCS length using a 2D DP table
- Backtrack the DP table to reconstruct the actual subsequence
- Understand the recurrence: equal characters extend, unequal take the max
- Optimize space using a rolling two-row approach
- Connect LCS to the `diff` algorithm and DNA sequence alignment

## Rust Application

`src/lib.rs` implements `lcs_length` with the standard 2D table. `lcs_string` fills the same table and then backtracks from `dp[m][n]` to reconstruct the actual subsequence string by following the choices that led to each cell. `lcs_space_optimized` uses only two rows instead of the full table, halving memory usage.

The LCS is the basis of the Myers diff algorithm used in git. The similarity metric between two documents is often expressed as `2 * lcs_length / (len_a + len_b)`.

## OCaml Approach

```ocaml
let lcs_length s1 s2 =
  let a = Array.of_seq (String.to_seq s1) in
  let b = Array.of_seq (String.to_seq s2) in
  let m, n = Array.length a, Array.length b in
  let dp = Array.make_matrix (m+1) (n+1) 0 in
  for i = 1 to m do
    for j = 1 to n do
      dp.(i).(j) <- if a.(i-1) = b.(j-1)
        then dp.(i-1).(j-1) + 1
        else max dp.(i-1).(j) dp.(i).(j-1)
    done
  done;
  dp.(m).(n)
```

The structure is identical; the syntax difference is OCaml's `.(i)` vs Rust's `[i]`.

## Key Differences

1. **Character iteration**: Rust collects `chars()` into `Vec<char>` for indexed access; OCaml converts to `Array.of_seq`.
2. **Matrix initialization**: OCaml's `Array.make_matrix` creates a 2D array in one call; Rust's `vec![vec![0; n+1]; m+1]` uses nested Vecs.
3. **Backtracking**: Both reconstruct the LCS by following `dp` cell comparisons from `(m, n)` to `(0, 0)` — the algorithm is identical.
4. **Space optimization**: The rolling two-row optimization is straightforward in both; Rust's `std::mem::swap` makes the row rotation explicit.

## Exercises

1. Implement `edit_distance_from_lcs(s1: &str, s2: &str) -> usize` using the formula `|s1| + |s2| - 2 * lcs_length`.
2. Write `lcs_all(s1: &str, s2: &str) -> Vec<String>` that returns all LCS strings (there may be multiple).
3. Implement the Myers diff algorithm that produces `+/-` lines like `git diff` using LCS as the core.
