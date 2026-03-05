# Egg Drop — Comparison

## Core Insight
The egg drop problem asks for worst-case minimum trials. The key insight for the optimal solution: instead of asking "given k eggs and n floors, what's the minimum trials?", ask "given t trials and k eggs, what's the maximum floors we can check?" This gives the recurrence `f(t,k) = 1 + f(t-1,k-1) + f(t-1,k)`.

## OCaml Approach
- 2D array DP with nested loops
- `ref` cells for binary search pointers
- `max_int` as initial sentinel for minimization
- `while` loop with `ref` counter for optimal approach

## Rust Approach
- 2D `vec!` DP table
- Binary search with tuple destructuring `let (mut lo, mut hi)`
- `usize::MAX` as sentinel
- Early return from nested loop for optimal approach
- Method chaining `.max()` and `.min()`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Sentinel | `max_int` | `usize::MAX` |
| Binary search | `ref` pointers + `while` | `let (mut lo, mut hi)` + `while` |
| Min/max | `min`/`max` functions | `.min()`/`.max()` methods |
| Optimal loop | `while dp.(!t).(eggs) < floors` | `for t in 1..=floors` + early return |
| Early exit | Increment `ref` counter | `return t` from inner loop |
