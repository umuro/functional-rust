# Adjacency List Graph — Comparison

## Core Insight
Adjacency lists are the most common graph representation for sparse graphs. Both languages map a node ID to its list of neighbors. The main API difference is Rust's `HashSet::insert()` returning a boolean (replaces the check-then-mark pattern).

## OCaml Approach
- `IntMap.t` mapping node to `int list` of neighbors
- `Hashtbl` for visited set (mutable)
- `Queue` module for BFS
- Recursive function for DFS
- Path reconstruction via parent hashtable

## Rust Approach
- `HashMap<usize, Vec<usize>>` for adjacency list
- `HashSet::insert()` returns `false` if already present — combines check + insert
- `VecDeque` for BFS queue
- Recursive DFS with `&mut HashSet` for visited
- `map_or(&[], ...)` for safe neighbor access

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Adjacency list | `int list IntMap.t` | `HashMap<usize, Vec<usize>>` |
| Visited set | `Hashtbl` + `mem` | `HashSet` + `insert` (returns bool) |
| BFS queue | `Queue` module | `VecDeque` |
| DFS | Recursive + Hashtbl | Recursive + `&mut HashSet` |
| Edge lookup | O(log n) Map | O(1) HashMap |
| Neighbor access | `find_opt` + default | `map_or(&[], ...)` |
