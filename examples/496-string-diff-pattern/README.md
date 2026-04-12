📖 **[View on hightechmind.io →](https://hightechmind.io/rust/496-string-diff-pattern)**

---

# String Diff Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



Levenshtein edit distance measures the minimum number of single-character insertions, deletions, and substitutions to transform one string into another — the foundation of spell checkers, fuzzy search, and diff algorithms.

## Problem Statement

How similar are two strings? This question drives: autocorrect (find the dictionary word closest to a typo), DNA sequence alignment (edit distance is Smith-Waterman without gaps), database fuzzy search (`pg_trgm`), file diff tools, and version control. **Levenshtein distance** solves this with dynamic programming: `dp[i][j]` is the edit distance between the first `i` chars of `s` and the first `j` chars of `t`. The recurrence is O(M×N) time and space, with well-known space-optimisation to O(min(M,N)).

## Learning Outcomes

- Implement Wagner-Fischer algorithm (Levenshtein DP) on character vectors
- Build the DP table iteratively from base cases to full solution
- Apply `.min()` chaining for the three-way minimum
- Implement `closest` using `Iterator::min_by_key` with Levenshtein as the key
- Understand the connection to LCS (longest common subsequence) and diff

## Rust Application

The DP fills an `(m+1) × (n+1)` table:

```rust
let sv: Vec<char> = s.chars().collect();
let tv: Vec<char> = t.chars().collect();
let mut dp = vec![vec![0usize; n + 1]; m + 1];
for i in 0..=m { dp[i][0] = i; }
for j in 0..=n { dp[0][j] = j; }
for i in 1..=m { for j in 1..=n {
    dp[i][j] = if sv[i-1] == tv[j-1] {
        dp[i-1][j-1]
    } else {
        1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1])
    };
}}
```

`closest` uses `min_by_key` for clean idiomatic selection:

```rust
fn closest<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates.iter().min_by_key(|&&c| levenshtein(query, c)).copied()
}
```

## OCaml Approach

```ocaml
let levenshtein s t =
  let sv = Array.of_seq (String.to_seq s) in
  let tv = Array.of_seq (String.to_seq t) in
  let m, n = Array.length sv, Array.length tv in
  let dp = Array.make_matrix (m+1) (n+1) 0 in
  for i = 0 to m do dp.(i).(0) <- i done;
  for j = 0 to n do dp.(0).(j) <- j done;
  for i = 1 to m do for j = 1 to n do
    dp.(i).(j) <- if sv.(i-1) = tv.(j-1) then dp.(i-1).(j-1)
    else 1 + min dp.(i-1).(j) (min dp.(i).(j-1) dp.(i-1).(j-1))
  done done;
  dp.(m).(n)
```

OCaml's `min` is a 2-argument function requiring chaining; Rust's `.min()` is chained on the value.

## Key Differences

1. **`Vec<char>` vs. `String`**: Rust converts to `Vec<char>` for O(1) indexed character access; OCaml uses `Array.of_seq (String.to_seq s)` for the same purpose.
2. **`min_by_key` ergonomics**: Rust's `Iterator::min_by_key` reads as prose; OCaml's `List.fold_left` with explicit comparison is more verbose.
3. **Space optimisation**: The current implementation uses O(M×N) space; Rust's iterator and slice API makes the space-optimised 1D rolling array straightforward.
4. **Unicode correctness**: Converting to `Vec<char>` ensures Unicode scalar values are the unit of comparison; byte-level comparison would corrupt multi-byte characters.

## Exercises

1. **Space-optimised version**: Rewrite `levenshtein` using two `Vec<usize>` rows instead of the full matrix, reducing memory from O(M×N) to O(min(M,N)).
2. **Edit path reconstruction**: Extend the DP to backtrack through the table and return the sequence of edits (insert/delete/substitute) needed to transform `s` into `t`.
3. **Jaro-Winkler distance**: Implement the Jaro-Winkler similarity (used in record linkage/deduplication) and compare its fuzzy-match quality against Levenshtein for common name misspellings.
