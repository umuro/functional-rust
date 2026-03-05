📖 **[View on hightechmind.io →](https://hightechmind.io/rust/787-edit-distance-levenshtein)**

---

# 787. Edit Distance (Levenshtein) DP

**Difficulty:** 3  **Level:** Intermediate

Compute the minimum number of insertions, deletions, and substitutions to transform one string into another — the backbone of spell checking, fuzzy search, and DNA analysis.

## The Problem This Solves

Edit distance quantifies how "different" two strings are, measured by the cheapest sequence of single-character edits needed to turn one into the other. Spell checkers use it to suggest corrections — "recieve" → "receive" has edit distance 2. Search engines use fuzzy matching based on edit distance to handle typos. DNA analysis tools measure mutation distance between gene sequences. Database deduplication pipelines use it to identify near-duplicate records.

Unlike LCS which only identifies commonality, edit distance explicitly models *transformation cost*, making it suitable for any domain where the edit operations themselves have real-world meaning (mutations, keyboard typos, OCR errors).

## The Intuition

At each position `(i, j)`, you're asking: what's the cheapest way to convert `s1[0..i]` to `s2[0..j]`? If the characters match, you pay nothing and inherit `dp[i-1][j-1]`. If they differ, you take the cheapest of three choices: substitute (diagonal), delete from s1 (up), or insert from s2 (left) — each costing 1. The table fills in O(n×m), and you can backtrack through it to recover the exact sequence of edits. The space-optimised two-row version is identical in logic but only needs O(m) space. OCaml's version is naturally recursive with memoisation; Rust prefers the iterative tabulation.

## How It Works in Rust

```rust
// O(n × m) time, O(n × m) space
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (n, m) = (c1.len(), c2.len());

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..=n { dp[i][0] = i; }   // delete all of s1
    for j in 0..=m { dp[0][j] = j; }   // insert all of s2

    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if c1[i-1] == c2[j-1] {
                dp[i-1][j-1]                               // match, free
            } else {
                1 + dp[i-1][j]          // delete from s1
                    .min(dp[i][j-1])    // insert from s2
                    .min(dp[i-1][j-1])  // substitute
            };
        }
    }
    dp[n][m]
}

// Space-optimised: O(m) — two rows only
pub fn edit_distance_opt(s1: &str, s2: &str) -> usize {
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr = vec![0usize; m + 1];
    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            curr[j] = if c1[i-1] == c2[j-1] { prev[j-1] }
                      else { 1 + prev[j].min(curr[j-1]).min(prev[j-1]) };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

// Traceback produces typed edit operations:
// Match(c), Substitute(from, to), Delete(c), Insert(c)
```

The traceback walks from `dp[n][m]` back to `dp[0][0]`, choosing the operation that produced each cell's value.

## What This Unlocks

- **Spell checking**: compute edit distance to all dictionary words; suggest those within distance 1–2. Bk-trees organise dictionary entries by edit distance for fast lookup.
- **Fuzzy string matching**: `grep -P`, `fzf`, and database LIKE operators use edit distance under the hood for approximate matching.
- **Sequence alignment**: Needleman-Wunsch (global alignment) and Smith-Waterman (local alignment) are generalisations of Levenshtein with affine gap penalties — the same DP table, different cost functions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Three-way min | `min (min del ins) sub` | `.min(dp[i][j-1]).min(dp[i-1][j-1])` |
| Table init | `Array.init` with base cases | Separate loops setting `dp[i][0]` and `dp[0][j]` |
| Space opt | Swap two arrays | `std::mem::swap(&mut prev, &mut curr)` |
| Edit ops enum | Variant type | `enum EditOp { Match, Substitute, Insert, Delete }` |
| Chars iteration | `String.get i` or `Array.of_seq` | `.chars().collect::<Vec<char>>()` |
