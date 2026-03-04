# 786. LCS: Classic DP with Backtracking

**Difficulty:** 3  **Level:** Intermediate

Find the longest subsequence common to two sequences — the foundation of diff tools, DNA alignment, and version control.

## The Problem This Solves

Whenever you compare two sequences and need to understand their similarity, you need LCS. `diff` and `git diff` use LCS (or a close variant) to identify which lines were added, removed, or unchanged. Bioinformatics tools use it to align DNA and protein sequences — finding conserved regions across species. Plagiarism detectors, file synchronisation tools, and grammar checkers all rely on some form of sequence alignment.

LCS is also the gateway to understanding a whole family of string DP problems: edit distance, longest common substring, sequence alignment with gaps — all share the same 2D table structure and similar recurrences.

## The Intuition

Two characters either match (extend the common subsequence by one) or don't (take the best from skipping one character in either string). The recurrence captures this cleanly: if `s1[i] == s2[j]`, then `dp[i][j] = dp[i-1][j-1] + 1`; otherwise `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`. Fill the table row by row, then backtrack through it to recover the actual subsequence. In OCaml you'd write the recursive version naturally with pattern matching; Rust uses explicit index arithmetic on a `Vec<Vec<usize>>`.

The diff output variant tags each character as Keep/Insert/Delete — showing how LCS directly drives change detection.

## How It Works in Rust

```rust
// O(n × m) time and space
pub fn lcs(s1: &str, s2: &str) -> String {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (n, m) = (c1.len(), c2.len());

    // Build the DP table
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if c1[i-1] == c2[j-1] {
                dp[i-1][j-1] + 1                    // characters match: extend
            } else {
                dp[i-1][j].max(dp[i][j-1])          // skip one side, take best
            };
        }
    }

    // Backtrack from dp[n][m] to reconstruct the subsequence
    let mut result = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if c1[i-1] == c2[j-1] {
            result.push(c1[i-1]);
            i -= 1; j -= 1;
        } else if dp[i-1][j] > dp[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    result.iter().rev().collect()
}

// Space-optimised: O(m) space, two rows only (no backtrack)
pub fn lcs_length_opt(s1: &[char], s2: &[char]) -> usize {
    let mut prev = vec![0usize; s2.len() + 1];
    let mut curr = vec![0usize; s2.len() + 1];
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            curr[j] = if s1[i-1] == s2[j-1] {
                prev[j-1] + 1
            } else {
                prev[j].max(curr[j-1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
        curr.fill(0);
    }
    prev[s2.len()]
}
```

`std::mem::swap` avoids allocation by reusing the two row buffers. The diff variant extends backtracking with `DiffOp::Keep/Insert/Delete` tags.

## What This Unlocks

- **Text diffing**: `git diff`, `diff -u`, and merge tools all use LCS or Myers' diff (an optimised variant) to produce `+`/`-` hunks.
- **DNA sequence alignment**: finding conserved genes across organisms by locating the longest matching subsequence of nucleotides or codons.
- **Plagiarism detection**: comparing student submissions by computing LCS length as a similarity metric, normalised by sequence length.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| DP table allocation | `Array.make_matrix (n+1) (m+1) 0` | `vec![vec![0usize; m+1]; n+1]` |
| Character extraction | `String.get` or `explode` | `.chars().collect::<Vec<char>>()` |
| Recurrence | Pattern match on equality | `if c1[i-1] == c2[j-1]` branch |
| Backtracking | Recursive function | Iterative `while i > 0 && j > 0` loop |
| Space opt | Swap two `array` references | `std::mem::swap(&mut prev, &mut curr)` |
