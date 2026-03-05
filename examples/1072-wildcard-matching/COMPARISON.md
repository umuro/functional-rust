# Wildcard Matching — Comparison

## Core Insight
Wildcard `*` matches any sequence (including empty), simpler than regex `*`. The DP recurrence: `*` → `dp[i-1][j] || dp[i][j-1]`. The greedy two-pointer approach remembers the last `*` position and backtracks there on mismatch.

## OCaml Approach
- 2D boolean array DP — same structure as regex matching
- Greedy: `ref` cells for pointers and star position
- `while` loop with `ref` mutation for pointer advancement
- Force-exit trick: `si := m + 1` to break while loop

## Rust Approach
- 2D `vec![vec![false]]` DP
- Greedy: `Option<usize>` for star position — cleaner than `-1` sentinel
- `if let Some(star) = star_idx` for safe star backtrack
- Slice `p[j..].iter().all()` for remaining-all-stars check

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Star sentinel | `ref (-1)` | `Option<usize>` |
| All-stars check | `ref` flag + loop | `.iter().all(\|&c\| c == '*')` |
| Loop exit | `si := m + 1` (hack) | `return false` |
| Pointer advance | `incr si; incr pi` | `si += 1; pi += 1` |
| DP structure | Identical recurrence | Identical recurrence |
