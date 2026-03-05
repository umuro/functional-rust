# 0/1 Knapsack — Comparison

## Core Insight
The 0/1 knapsack builds a 2D table where `dp[i][w]` = max value using first `i` items with capacity `w`. The 1D optimization exploits the fact that each row only depends on the previous row, with reverse iteration ensuring items aren't reused.

## OCaml Approach
- `Array.init` for 2D arrays — creates array of arrays
- `for w = capacity downto weight` for reverse iteration in 1D version
- `Hashtbl` with tuple keys `(i, w)` for memoization
- Pattern matching on `find_opt` for cache access

## Rust Approach
- `vec![vec![0; cap+1]; n+1]` for 2D table
- `(weight..=cap).rev()` for reverse range — idiomatic Rust
- `HashMap<(usize, usize), i64>` for memoization with tuple keys
- Nested function with explicit parameter passing (no closure capture of `&mut`)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| 2D table | `Array.init (n+1) (fun _ -> Array.make ...)` | `vec![vec![0; cap+1]; n+1]` |
| Reverse iteration | `for w = cap downto wt` | `(wt..=cap).rev()` |
| Tuple hash key | `(i, w)` — works directly | `(usize, usize)` — `Hash` auto-derived |
| Space optimization | 1D array with `downto` | 1D vec with `.rev()` range |
| Max function | `max` (built-in) | `.max()` method on `i64` |
