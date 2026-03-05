# Burst Balloons — Comparison

## Core Insight
Burst balloons is interval DP: think about which balloon to burst *last* in each subinterval. Adding virtual boundary balloons (value 1) simplifies edge cases. `dp[i][j]` = max coins obtainable from all balloons strictly between i and j.

## OCaml Approach
- `Array.init (n+2)` for padded balloon array
- Triple-nested loops for gap/start/split
- `max` for tracking best split
- `Hashtbl` with tuple keys for memoization

## Rust Approach
- `vec![1i32; n+2]` for padded array
- Same triple-nested loop structure
- `.max()` chained method
- Two memoization styles: `HashMap` and 2D `Vec<Vec<i32>>` with `-1` sentinel

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Padded array | `Array.init (n+2) (fun i -> ...)` | `vec![1; n+2]` + fill loop |
| Memo sentinel | `Hashtbl` (no sentinel needed) | `vec![vec![-1; len]; len]` or `HashMap` |
| Gap iteration | `for gap = 2 to len-1` | `for gap in 2..len` |
| Overflow risk | OCaml ints (63-bit) | `i32` — safe for typical inputs |
