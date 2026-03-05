# Stone Game — Comparison

## Core Insight
Stone game is minimax DP on intervals. The key trick: store the *score difference* rather than absolute scores. Taking pile `i` gives `piles[i] - dp[i+1][j]` because after taking, your opponent is the "current player" for the remaining interval. The mathematical solution (first player always wins with even piles) bypasses DP entirely.

## OCaml Approach
- `Array.init n (fun i -> Array.init n ...)` for 2D table with diagonal init
- `max` of two choices (take left vs take right)
- `Hashtbl` for memoization
- Score reconstruction: `(total + diff) / 2`

## Rust Approach
- `vec![vec![0; n]; n]` with separate diagonal init loop
- `.max()` method chaining
- `HashMap` for memoization
- Same score reconstruction formula

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| 2D init | `Array.init n (fun i -> Array.init n (fun j -> ...))` | Init loop + `vec!` |
| Minimax | `max (take_left) (take_right)` | `.max()` |
| Score difference | Subtraction: `piles.(i) - dp.(i+1).(j)` | Same: `piles[i] - dp[i+1][j]` |
| Math proof | `true` (same insight) | `true` (same insight) |
| Score split | `(total + diff) / 2` | `(total + diff) / 2` |
