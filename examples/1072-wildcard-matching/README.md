📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1072-wildcard-matching)**

---

# 1072-wildcard-matching — Wildcard Matching

## Problem Statement

Wildcard matching with `?` (any single character) and `*` (any sequence, including empty) is used in file globbing (`*.rs` matches all Rust files), URL routing (`/api/*/users`), and command-line pattern matching. Unlike regex's `.*`, wildcard `*` matches any sequence including empty — semantically simpler but requiring careful DP handling.

The key difference from regex matching (1071): wildcard `*` matches any sequence by itself, while regex `*` requires a preceding character. This makes the DP slightly different.

## Learning Outcomes

- Implement wildcard matching with `?` and `*` using 2D DP
- Understand the difference between wildcard `*` and regex `.*`
- Handle `*` in the DP: `dp[i][j] = dp[i-1][j] || dp[i][j-1]`
- Implement memoized recursion as an alternative
- Connect to Unix shell globbing and URL router matching

## Rust Application

`src/lib.rs` implements `is_match_dp` where `*` matches any sequence. The border initialization sets `dp[0][j] = dp[0][j-1]` when `p[j-1] == '*'` (a leading `*` matches the empty string). The inner `*` case: `dp[i][j] = dp[i-1][j] || dp[i][j-1]` — use `*` to match one more character (`dp[i-1][j]`) or treat `*` as empty (`dp[i][j-1]`).

This DP is the basis of `minimatch` (Node.js), Python's `fnmatch`, and URL router matching in web frameworks.

## OCaml Approach

```ocaml
let is_match s p =
  let s = Array.of_seq (String.to_seq s) in
  let p = Array.of_seq (String.to_seq p) in
  let m, n = Array.length s, Array.length p in
  let dp = Array.make_matrix (m+1) (n+1) false in
  dp.(0).(0) <- true;
  for j = 1 to n do
    if p.(j-1) = '*' then dp.(0).(j) <- dp.(0).(j-1)
  done;
  for i = 1 to m do
    for j = 1 to n do
      dp.(i).(j) <- if p.(j-1) = '*' then dp.(i-1).(j) || dp.(i).(j-1)
        else (p.(j-1) = '?' || p.(j-1) = s.(i-1)) && dp.(i-1).(j-1)
    done
  done;
  dp.(m).(n)
```

Identical recurrence with different `*` semantics than regex matching.

## Key Differences

1. **`*` semantics**: Wildcard `*` matches any sequence standalone; regex `*` requires a preceding character. This changes the border initialization and `*` case.
2. **Simpler DP**: Wildcard DP is slightly simpler — the `*` case is one `||` without the prefix character check needed for regex.
3. **Globbing**: Unix file globbing uses this semantics; `*.rs` = `*` + `.rs` where `*` matches any sequence.
4. **Two-state `*`**: The `dp[i-1][j] || dp[i][j-1]` for `*` encodes "match one more char" vs "treat as empty" — a two-state automaton step.

## Exercises

1. Add support for `[abc]` character classes that match any character in the set.
2. Implement `glob_files(pattern: &str, files: &[&str]) -> Vec<&str>` using wildcard matching to filter a file list.
3. Write a URL router using wildcard matching: `Route { pattern: &str, handler: fn(&str) }` with `match_route(url: &str, routes: &[Route]) -> Option<&Route>`.
