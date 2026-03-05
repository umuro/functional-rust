# All Subsets (Power Set) — Comparison

## Core Insight
Three approaches: backtracking (include/skip decisions), bitmasking (enumerate 0..2^n), and iterative doubling (fold over elements, cloning all subsets with new element added). Each has different elegance-performance tradeoffs.

## OCaml Approach
- Backtracking with `ref` list and prepend/tail
- Bitmask: `List.init total` + `lsl`/`land` bit ops
- Fold: `Array.fold_left` with list append `@` — elegant but allocates
- `List.rev` needed for ordering in backtrack/bitmask

## Rust Approach
- Backtracking: `push`/`pop` + `clone()`
- Bitmask: nested iterator chain `.map` + `.filter` + `.collect()` — very idiomatic
- Fold: `iter().fold(vec![vec![]], ...)` with clone and push
- `1 << n` for power of 2

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Bitmask idiom | `mask land (1 lsl i)` | `mask & (1 << i)` |
| Iterator chain | `List.init` + loop | `(0..total).map(...).collect()` |
| Fold doubling | `Array.fold_left` + `@` | `.fold(vec![vec![]], ...)` + clone |
| Subset count | `1 lsl n` | `1 << n` |
| Cloning | `List.rev` (structural sharing) | `.clone()` (deep copy) |
