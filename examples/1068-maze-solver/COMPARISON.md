# Maze Solver — Comparison

## Core Insight
DFS backtracking finds any path; BFS finds the shortest. Both need visited tracking and boundary checking. The parent array in BFS enables path reconstruction by tracing back from the destination.

## OCaml Approach
- `Queue` module for BFS
- `ref` cells for mutable path and found flag
- Tuple arrays for parent tracking `(int * int) array array`
- `Array.iter` over direction tuples

## Rust Approach
- `VecDeque` for BFS
- `const DIRS` for compile-time direction array
- Bounds checking via `i32` arithmetic then cast back to `usize`
- `Option<Vec<...>>` return type — idiomatic for "might not exist"

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Directions | `let directions = [|...|]` | `const DIRS: [(i32,i32); 4]` |
| Bounds check | `r < 0 \|\| r >= rows` | Cast to `i32`, compare, cast back |
| BFS queue | `Queue.t` | `VecDeque` |
| Path return | `option` | `Option<Vec<...>>` |
| Parent tracking | `(int * int) array array` | `Vec<Vec<(usize,usize)>>` |
