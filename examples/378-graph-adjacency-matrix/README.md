📖 **[View on hightechmind.io →](https://hightechmind.io/rust/378-graph-adjacency-matrix)**

---

# 378: Graph — Adjacency Matrix
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Some graph algorithms require O(1) edge existence queries. For dense graphs where most pairs of vertices are connected — circuit boards, protein interaction networks, game adjacency grids — storing edges as a 2D boolean matrix provides constant-time lookup at the cost of O(V²) space. Transitive closure algorithms (Floyd-Warshall), adjacency matrix multiplication for path counting, and spectral graph analysis all operate naturally on matrix form.

Adjacency matrices appear in network routing tables, game AI pathfinding on grid maps, social influence modeling, and scientific computing (graph Laplacian for mesh simulation).

## Learning Outcomes

- Understand when adjacency matrices outperform adjacency lists (dense graphs, O(1) edge queries)
- Learn how to represent a 2D adjacency structure using `Vec<Vec<bool>>` in Rust
- Understand degree computation, edge counting, and neighbor enumeration over matrix form
- See the trade-off: O(1) `has_edge` vs. O(V) neighbor enumeration
- Learn to implement both directed and undirected edges in matrix form

## Rust Application

The `Graph` struct in `src/lib.rs` stores the graph as `Vec<Vec<bool>>` initialized to all-false. `add_edge` sets `matrix[u][v] = true` and symmetrically `matrix[v][u]` for undirected graphs. `has_edge` is a direct index operation — O(1). `neighbors` uses an iterator with `filter` over the row. Edge counting iterates the upper triangle to avoid double-counting undirected edges.

## OCaml Approach

OCaml uses `bool array array` for the adjacency matrix, initialized with `Array.make_matrix`. Edge checking is `matrix.(u).(v)`. Neighbor enumeration uses `Array.to_seq` with `Seq.filter_mapi`. The matrix representation integrates naturally with OCaml's array performance, which matches Rust's since both compile to contiguous memory.

## Key Differences

1. **Index syntax**: Rust uses `matrix[u][v]` with bounds-checked indexing (panics on OOB); OCaml uses `matrix.(u).(v)` which also raises `Invalid_argument` on OOB.
2. **Initialization**: Rust uses `vec![vec![false; n]; n]`; OCaml uses `Array.make_matrix n n false` — equivalent performance, different ergonomics.
3. **Neighbor iteration**: Rust uses `(0..n).filter(|&v| self.matrix[u][v]).collect()`; OCaml would use `Array.to_seqi` or a recursive fold.
4. **Mutability**: Rust requires `&mut self` for edge insertion enforced by the borrow checker; OCaml arrays are mutable by default with no compile-time tracking.

## Exercises

1. **Transitive closure**: Implement Floyd-Warshall's algorithm to compute the transitive closure matrix — `closure[u][v] = true` if there is any path from u to v.
2. **Matrix multiplication for path counting**: Implement matrix boolean multiplication and use it to count whether paths of exactly length k exist between any two vertices.
3. **Graph complement**: Write a function that returns the complement graph (edges where original has none, no edges where original has some), and verify it using a known complete graph.
