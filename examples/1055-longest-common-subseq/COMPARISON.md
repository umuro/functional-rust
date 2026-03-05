# Longest Common Subsequence — Comparison

## Core Insight
LCS uses a 2D DP table where `dp[i][j]` = length of LCS of first `i` chars of s1 and first `j` chars of s2. Backtracking from the bottom-right corner reconstructs the actual subsequence.

## OCaml Approach
- `Array.init` for 2D table, `Buffer` for string reconstruction
- Character access via `s.[i]` (O(1) for ASCII strings)
- Backtracking uses `ref` cells for mutable indices
- `String.init` with index reversal to flip the backtracked result

## Rust Approach
- Strings converted to `Vec<char>` first (UTF-8 aware)
- `vec![vec![0; n+1]; m+1]` for 2D table
- Backtracking pushes to `Vec<char>` then `reverse()`
- `into_iter().collect()` to convert back to `String`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| String indexing | `s.[i]` — O(1) byte access | Must convert to `Vec<char>` first |
| 2D table | `Array.init` + `Array.make` | `vec![vec![]; ...]` |
| String building | `Buffer.add_char` + `Buffer.contents` | `Vec<char>` + `.collect()` |
| Reversal | `String.init` with reversed index | `.reverse()` in-place |
| UTF-8 handling | Byte-level (ASCII only) | Full Unicode via `chars()` |
