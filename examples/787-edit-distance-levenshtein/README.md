📖 **[View on hightechmind.io →](https://hightechmind.io/rust/787-edit-distance-levenshtein)**

---

# 787-edit-distance-levenshtein — Edit Distance (Levenshtein)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Edit distance (Levenshtein distance, 1966) measures how many single-character insertions, deletions, or substitutions transform one string into another. It is the backbone of spell checkers (aspell, hunspell), fuzzy string search (Elasticsearch, Redis), autocorrect (mobile keyboards), DNA sequence alignment, and natural language processing. The DP algorithm runs in O(mn) time and is one of the most practically important algorithms in computing.

## Learning Outcomes

- Build the Levenshtein DP table with proper initialization of boundary conditions
- Apply the recurrence: min of delete (`dp[i-1][j]+1`), insert (`dp[i][j-1]+1`), and substitute (`dp[i-1][j-1]+cost`)
- Understand the relationship between edit distance and LCS
- Implement Damerau-Levenshtein (adds transpositions) and Jaro-Winkler distance
- Optimize from O(mn) space to O(min(m,n)) using two rolling rows

## Rust Application

`edit_distance(a: &str, b: &str) -> usize` initializes `dp[i][0] = i` (delete all of a) and `dp[0][j] = j` (insert all of b). The main loop computes the three-way min. The classic example: `edit_distance("kitten", "sitting") == 3` (substitute k→s, substitute e→i, insert g). Tests cover empty strings, identical strings, single characters, and the kitten/sitting benchmark.

## OCaml Approach

OCaml implements the same algorithm with `Array.make_matrix`. The `ukkonen` library provides a fast O(nd) edit distance for closely-related strings. `diff` libraries in OCaml use edit distance internally for line-level comparisons. The functional style can express this as a memoized recursion, but the imperative nested loop is more efficient and idiomatic for DP.

## Key Differences

1. **Algorithm**: The DP algorithm is language-independent; Rust and OCaml implementations are structurally identical.
2. **Rolling optimization**: Replacing the 2D table with two rows reduces memory from O(mn) to O(n); both languages implement this identically.
3. **Unicode**: Rust's `chars()` handles Unicode properly; OCaml's `Bytes.get` operates on bytes, requiring explicit UTF-8 handling for Unicode strings.
4. **Production use**: The `strsim` Rust crate provides Levenshtein, Damerau-Levenshtein, and Jaro-Winkler; OCaml's `stringdist` provides similar functionality.

## Exercises

1. Implement `bounded_edit_distance(a, b, max_dist)` that returns `None` if the edit distance exceeds `max_dist`, short-circuiting computation for obviously different strings.
2. Implement Damerau-Levenshtein distance that also counts adjacent character transpositions (`"ab" → "ba"` costs 1 instead of 2).
3. Write `fuzzy_search(query: &str, candidates: &[&str], threshold: usize) -> Vec<(&str, usize)>` that returns all candidates within `threshold` edit distance, sorted by distance.
