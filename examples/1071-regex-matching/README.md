📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1071-regex-matching)**

---

# 1071-regex-matching — Regex Matching with . and *
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implementing basic regular expression matching with `.` (any character) and `*` (zero or more of the preceding character) is a classic DP/recursion problem. Real regex engines use NFA/DFA compilation for efficiency, but understanding the DP solution illuminates how `.*` can match arbitrarily many characters and why regex backtracking can be exponential.

This is a fundamental problem in parsing, pattern matching, and understanding the semantics of regular expressions.

## Learning Outcomes

- Implement regex matching with `.` and `*` using 2D DP
- Understand the `p[j-1] == '*'` case: zero occurrences (skip p[j-2]) or one-or-more
- Implement the memoized recursive variant for clarity
- Understand why `.*` can match the empty string (zero occurrences)
- Connect to Thompson NFA compilation for linear-time regex matching

## Rust Application

`src/lib.rs` implements `is_match_dp(s: &str, p: &str) -> bool` where `dp[i][j]` = does `s[0..i]` match `p[0..j]`? The border initialization handles patterns like `a*b*` that can match the empty string. The `*` case requires two sub-cases: zero occurrences (`dp[i][j] = dp[i][j-2]`) or one-or-more (`dp[i][j] |= dp[i-1][j]` when characters match).

The DP correctly handles the tricky `.*` pattern that matches any sequence. A recursive implementation with memoization is provided for comparison.

## OCaml Approach

```ocaml
let is_match s p =
  let s = Array.of_seq (String.to_seq s) in
  let p = Array.of_seq (String.to_seq p) in
  let m, n = Array.length s, Array.length p in
  let dp = Array.make_matrix (m+1) (n+1) false in
  dp.(0).(0) <- true;
  for j = 2 to n do
    if p.(j-1) = '*' then dp.(0).(j) <- dp.(0).(j-2)
  done;
  for i = 1 to m do
    for j = 1 to n do
      dp.(i).(j) <- if p.(j-1) = '*' then
        dp.(i).(j-2) || (p.(j-2) = '.' || p.(j-2) = s.(i-1)) && dp.(i-1).(j)
      else (p.(j-1) = '.' || p.(j-1) = s.(i-1)) && dp.(i-1).(j-1)
    done
  done;
  dp.(m).(n)
```

Identical structure. The DP recurrence is purely mathematical and language-independent.

## Key Differences

1. **Pattern vs `regex` crate**: This implements a subset for teaching; production Rust uses the `regex` crate (NFA-based, no exponential backtracking).
2. **Character access**: Rust collects `chars()` into `Vec<char>`; OCaml converts to `Array.of_seq`.
3. **Two sub-cases for `*`**: Both encode `dp[i][j-2] || (char_matches && dp[i-1][j])` — the OR logic is identical.
4. **Edge case**: `dp[0][j]` handles patterns matching the empty string — `a*`, `.*`, `a*b*` all match `""`. Both initialize this border correctly.

## Exercises

1. Extend the pattern language to support `+` (one or more) and `?` (zero or one).
2. Implement `regex_find_all(s: &str, p: &str) -> Vec<(usize, usize)>` that returns all (start, end) positions where the pattern matches.
3. Write a function that compiles a simple regex into an NFA and uses Thompson's algorithm for linear-time matching.
