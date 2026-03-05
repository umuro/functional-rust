# Sudoku Solver — Comparison

## Core Insight
Sudoku is backtracking with three overlapping constraints (row, column, 3×3 box). Maintaining constraint arrays reduces validation to O(1) per candidate. Both languages use in-place mutation with undo on backtrack.

## OCaml Approach
- 2D arrays with `ref` for found-cell tracking
- Nested `for` loops with `ref` flags for early exit simulation
- `Array.init 9 (fun _ -> Array.make 10 false)` for constraint arrays
- Box index: `(r/3)*3 + c/3`

## Rust Approach
- Fixed-size arrays `[[u8; 9]; 9]` — stack-allocated, cache-friendly
- Nested `for` loops with early `return false` — cleaner control flow
- `[[bool; 10]; 9]` constraint arrays — also stack-allocated
- Inner `fn` with explicit mutable references

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Board type | `int array array` (heap) | `[[u8; 9]; 9]` (stack) |
| Constraint arrays | `bool array array` | `[[bool; 10]; 9]` (fixed) |
| Empty cell search | `ref None` + iteration | Nested `for` with early `return` |
| Backtrack | `board.(r).(c) <- 0` | `board[r][c] = 0` |
| Early exit | `ref` flag + `not !solved` check | `return false` / `return true` |
