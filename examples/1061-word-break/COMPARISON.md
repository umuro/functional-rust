# Word Break — Comparison

## Core Insight
Word break checks if a string can be segmented into valid dictionary words. The DP approach marks reachable positions; BFS treats positions as graph nodes with dictionary words as edges.

## OCaml Approach
- `StringSet` via `Set.Make(String)` functor — ordered set
- `String.sub s j (i-j)` for substring extraction
- `Queue` module for BFS
- Pattern matching on `find_opt` for memoization

## Rust Approach
- `HashSet<&str>` — O(1) average lookup
- String slicing `&s[j..i]` — zero-copy substring (valid for ASCII)
- `VecDeque` for BFS
- Early `break` in inner loops for optimization

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Dictionary type | `Set.Make(String)` (tree) | `HashSet<&str>` (hash) |
| Lookup complexity | O(log n) | O(1) average |
| Substring | `String.sub s pos len` (allocates) | `&s[j..i]` (zero-copy slice) |
| BFS queue | `Queue.t` (mutable) | `VecDeque` |
| Early termination | `ref` flag + check | `break` statement |
