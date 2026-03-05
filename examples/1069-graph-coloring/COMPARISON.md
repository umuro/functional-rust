# Graph Coloring — Comparison

## Core Insight
Graph coloring assigns colors to vertices so no edge connects same-colored vertices. Backtracking tries each color, backtracks on conflict. Greedy coloring is fast but may use more colors than optimal.

## OCaml Approach
- `Array` for colors with `0` as uncolored sentinel
- `List.for_all` for adjacency list safety check
- `ref` flag for early exit from color loop

## Rust Approach
- `Vec<usize>` for colors, inner `fn` for recursion
- `.iter().all()` for adjacency list safety — idiomatic
- `HashSet` in greedy approach to find first unused color
- `(1..).find()` — infinite iterator to find smallest available color

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Safety check (list) | `List.for_all` | `.iter().all()` |
| Early exit | `ref` flag + `not !found` | `return true/false` |
| Greedy first-fit | Would use `List.find` | `(1..).find(\|c\| !used.contains(c))` |
| Graph representation | `int array array` or `int list array` | `Vec<Vec<i32>>` or `Vec<Vec<usize>>` |
