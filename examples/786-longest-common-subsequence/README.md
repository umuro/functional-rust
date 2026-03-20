📖 **[View on hightechmind.io →](https://hightechmind.io/rust/786-longest-common-subsequence)**

---

# 786-longest-common-subsequence — Longest Common Subsequence
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The Longest Common Subsequence (LCS) problem finds the longest sequence of characters common to two strings (not necessarily contiguous). It is the foundation of `diff` tools (used in Git, patch, and code review), DNA sequence alignment (BLAST, ClustalW), and plagiarism detection. The classic O(mn) DP solution by Hirschberg (1975) fills an m×n table comparing characters.

## Learning Outcomes

- Fill a 2D DP table `dp[i][j]` = LCS length of `a[:i]` and `b[:j]`
- Apply the recurrence: match increases by 1, mismatch takes max of adjacent cells
- Reconstruct the actual LCS string by backtracking through the table
- Understand the relationship between LCS and edit distance (Levenshtein)
- See how LCS extends to multiple sequences (3-way LCS) at higher complexity

## Rust Application

`lcs(a: &str, b: &str) -> usize` builds the DP table and returns the LCS length. `lcs_string(a, b) -> String` reconstructs the actual subsequence by backtracking from `dp[m][n]`: when `a[i-1] == b[j-1]` add that character; otherwise move toward the larger neighbor. Tests cover identical strings, empty strings, no common characters, and partial overlaps.

## OCaml Approach

OCaml implements LCS with the same 2D array approach. `let dp = Array.make_matrix (m+1) (n+1) 0 in ...`. Backtracking uses a recursive function that pattern-matches on the dp values. OCaml's `List.rev` is often used to accumulate characters in reverse during backtracking. The `Diff` library wraps LCS for line-level diff computation similar to `git diff`.

## Key Differences

1. **Array access**: Rust's `Vec<Vec<usize>>` indexing with `dp[i][j]`; OCaml's `dp.(i).(j)` — syntactically different but equivalent.
2. **String iteration**: Rust collects `chars()` into a `Vec<char>` for indexed access; OCaml uses `Bytes.get` on a `Bytes.of_string` copy.
3. **Backtracking**: Both use iterative backtracking with while loops — identical structure.
4. **Space optimization**: Hirschberg's O(n) space LCS algorithm is expressible in both languages; neither standard library includes it.

## Exercises

1. Implement `lcs_diff(a: &[&str], b: &[&str]) -> Vec<DiffOp>` that returns a diff as a list of `Keep`, `Insert`, and `Delete` operations — the basis for `git diff`.
2. Implement the space-optimized O(m+n) space LCS using Hirschberg's divide-and-conquer algorithm.
3. Extend to three sequences: `lcs3(a, b, c)` using a 3D DP table, and verify it on biological sequences.
