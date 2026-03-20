📖 **[View on hightechmind.io →](https://hightechmind.io/rust/377-graph-adjacency-list)**

---

# 377: Graph — Adjacency List
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Graphs model relationships between entities: social networks, road maps, dependency systems, web links. The adjacency list representation stores for each vertex the list of its neighbors, using O(V + E) space — optimal for sparse graphs where E << V². For a social network with 1 billion users each having ~200 friends, adjacency lists use 200 billion entries versus a matrix requiring 10^18 cells.

Graph traversal algorithms (BFS, DFS) built on adjacency lists power shortest-path routing (GPS navigation), network analysis (PageRank), build system dependency resolution, and social graph recommendations.

## Learning Outcomes

- Understand when adjacency lists outperform adjacency matrices (sparse graphs)
- Learn BFS and DFS traversal implementations over adjacency list graphs
- See how `HashMap<usize, Vec<usize>>` provides the core list structure in Rust
- Understand directed vs. undirected graph construction via conditional edge insertion
- Learn how `VecDeque` enables efficient BFS queue operations

## Rust Application

The `Graph` struct in `src/lib.rs` uses `HashMap<usize, Vec<usize>>` as its adjacency list. The `add_edge` method uses `entry().or_default()` to lazily initialize neighbor vectors. For undirected graphs, both directions are inserted. BFS uses `VecDeque` as a queue with `HashSet` for visited tracking. DFS is implemented recursively with an explicit visited set. The `neighbors` method returns a `&[usize]` slice, avoiding allocation.

## OCaml Approach

OCaml represents adjacency lists as `int list array` (array of lists) or `(int, int list) Hashtbl.t`. Graph traversal uses recursive functions naturally — OCaml's stack and tail-call optimization handle DFS elegantly. BFS uses `Queue.t` from the standard library. The functional style produces visited sets using `Set` modules rather than mutable `HashSet`.

## Key Differences

1. **Storage type**: Rust uses `HashMap<usize, Vec<usize>>` for dynamic vertex sets; OCaml uses `array` of lists when vertices are dense integers, or `Hashtbl` for sparse maps.
2. **Traversal style**: Rust's BFS uses iterative `VecDeque` with `&mut HashSet`; OCaml's BFS uses `Queue.t` with a mutable visited array or functional `Set`.
3. **Edge insertion**: Rust uses `entry().or_default()` idiom; OCaml uses pattern matching on `Hashtbl.find_opt` or list prepending.
4. **Iterator protocol**: Rust's `vertices()` returns `impl Iterator`; OCaml produces lists or uses `Hashtbl.iter` with a callback.

## Exercises

1. **Connected components**: Implement a function that returns the number of connected components in an undirected graph using BFS/DFS, and identify which component each vertex belongs to.
2. **Bipartite check**: Write a function that determines whether a graph is bipartite (2-colorable) using BFS coloring, returning `Ok((left, right))` with the two vertex sets or `Err(())` if a cycle of odd length is found.
3. **Shortest path BFS**: Extend BFS to track parent pointers and reconstruct the actual path between two vertices, not just the visited order.
