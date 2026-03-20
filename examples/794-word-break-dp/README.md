📖 **[View on hightechmind.io →](https://hightechmind.io/rust/794-word-break-dp)**

---

# 794-word-break-dp — Word Break

## Problem Statement

Given a string and a dictionary of words, determine if the string can be segmented into a sequence of dictionary words. This is used in natural language processing (Chinese word segmentation — Chinese text has no spaces), search engine query parsing, and code tokenization. The DP approach avoids the exponential backtracking of a naive recursive search by caching whether each prefix is breakable.

## Learning Outcomes

- Model word break as `dp[i]` = whether `s[:i]` can be formed from dictionary words
- Apply the recurrence: `dp[i] = any j: dp[j] && s[j..i] in dict`
- Use a `HashSet` for O(1) word lookups vs. O(k) linear scan
- Understand the O(n²) time and O(n) space complexity
- Extend to count all possible segmentations (exponential output)

## Rust Application

`word_break(s, dict)` builds `HashSet` from `dict`. `dp[0] = true` (empty prefix). For each i from 1 to n, scans all j from 0 to i: if `dp[j]` and `s[j..i]` is in the dictionary, set `dp[i] = true`. The example `word_break("leetcode", ["leet","code"])` returns true; `word_break("catsandog", [...])` returns false. Tests cover both cases.

## OCaml Approach

OCaml implements with `Array.make (n+1) false` and uses `Hashtbl` for the dictionary. The inner loop uses `try` with `Hashtbl.find` or `Hashtbl.mem`. Functional style uses `Array.init` + `List.exists` over the dictionary. The `words` list can be sorted by length to prune impossible substrings early. OCaml's `Re` library provides regex-based word segmentation as an alternative approach.

## Key Differences

1. **HashSet vs Hashtbl**: Rust uses `HashSet<&str>` for O(1) lookup; OCaml's `Hashtbl.mem` provides the same.
2. **Slice comparison**: Rust's `s[j..i]` string slice is a `&str`; OCaml's `String.sub` creates a new string allocation on each check — a minor performance difference.
3. **Early termination**: Rust's `break` exits the inner loop once `dp[i]` is true; OCaml uses a boolean flag or exception-based early exit.
4. **Segmentation**: The all-segmentations variant uses DFS with memoization; both languages implement it similarly.

## Exercises

1. Implement `word_break_all(s, dict) -> Vec<String>` that returns all valid space-separated segmentations (e.g., `["cat sand dog", "cats and dog"]`).
2. Add word length bounds to the inner loop: only check substrings of length between `min_word_len` and `max_word_len` in the dictionary, reducing the inner loop iterations.
3. Implement a streaming version that breaks a text input into words on the fly, maintaining the DP state as new characters arrive.
