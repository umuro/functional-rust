# Coin Change — Comparison

## Core Insight
Coin change is the canonical unbounded knapsack problem. Bottom-up DP fills a 1D table; memoized recursion explores the same subproblems top-down. Both languages express the recurrence similarly, but Rust adds a BFS approach treating it as a shortest-path problem.

## OCaml Approach
- Imperative array DP with nested `List.iter` for coins
- Memoized recursion using `Hashtbl` and `List.fold_left` for clean min-finding
- Pattern matching on `find_opt` for cache lookup

## Rust Approach
- `vec!` DP table with nested iteration — straightforward translation
- `HashMap` memoization with inner function taking `&mut` cache
- `VecDeque` BFS approach — coins as graph edges, amount as target node
- `i32::MAX` as sentinel vs OCaml's `max_int`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| DP table | `Array.make (n+1) max_val` | `vec![max_val; n + 1]` |
| Impossible sentinel | `max_int` | `i32::MAX` |
| Coin iteration | `List.iter` | `for &coin in coins` |
| Min finding | `List.fold_left` with `min` | `.iter().fold()` with `.min()` |
| BFS approach | Not shown (less idiomatic) | `VecDeque` — natural in Rust |
| Return convention | `-1` for impossible | `-1` for impossible (LeetCode style) |
