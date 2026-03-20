📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1056-edit-distance)**

---

# 1056-edit-distance — Edit Distance (Levenshtein)

## Problem Statement

Edit distance (Levenshtein distance) measures how many single-character edits — insertions, deletions, and substitutions — are needed to transform one string into another. It is the core metric in spell checkers, fuzzy search, DNA sequence alignment, and natural language processing for measuring string similarity.

Vladimir Levenshtein introduced the metric in 1966. The DP solution fills a 2D table in O(m×n) time, making it tractable for strings up to a few thousand characters.

## Learning Outcomes

- Implement edit distance using a 2D DP table
- Optimize to O(min(m, n)) space using two rolling rows
- Understand the recurrence: match, substitute, insert, delete
- Use edit distance for fuzzy string matching
- Connect to the `strsim` crate for production use

## Rust Application

`src/lib.rs` implements `edit_distance` with the full 2D table where `dp[i][j]` is the edit distance between the first `i` characters of `s1` and the first `j` characters of `s2`. The recurrence: if `s1[i-1] == s2[j-1]`, cost is 0; otherwise, take `1 + min(substitute, insert, delete)`. `edit_distance_opt` uses two rolling rows for O(n) space.

The `strsim` crate provides `levenshtein`, `jaro_winkler`, and other string similarity metrics for production use.

## OCaml Approach

```ocaml
let edit_distance s1 s2 =
  let a = Array.of_seq (String.to_seq s1) in
  let b = Array.of_seq (String.to_seq s2) in
  let m, n = Array.length a, Array.length b in
  let dp = Array.init (m+1) (fun i ->
    Array.init (n+1) (fun j -> if i = 0 then j else if j = 0 then i else 0)
  ) in
  for i = 1 to m do
    for j = 1 to n do
      dp.(i).(j) <- if a.(i-1) = b.(j-1) then dp.(i-1).(j-1)
        else 1 + min (min dp.(i-1).(j-1) dp.(i-1).(j)) dp.(i).(j-1)
    done
  done;
  dp.(m).(n)
```

The algorithms are structurally identical — edit distance is a mathematical operation with one correct DP formulation.

## Key Differences

1. **Initialization**: Rust initializes the border with separate loops; OCaml can use `Array.init` with a conditional expression.
2. **`min` three-way**: Rust uses `a.min(b).min(c)`; OCaml uses `min (min a b) c`. Both are equivalent.
3. **Space optimization**: The two-row rolling optimization is identical in both languages — the mathematical insight is language-independent.
4. **Unicode**: Both use char-level edit distance here. Byte-level edit distance is faster but incorrect for multibyte characters.

## Exercises

1. Implement `edit_distance_k(s1: &str, s2: &str, k: usize) -> bool` that returns whether edit distance is ≤ k without computing the full table (using the diagonal band optimization).
2. Write `closest_word(target: &str, dictionary: &[&str]) -> &str` that returns the word with minimum edit distance from the dictionary.
3. Implement the Damerau-Levenshtein distance that also counts transpositions (swapping adjacent characters) as a single edit.
