📖 **[View on hightechmind.io →](https://hightechmind.io/rust/808-topological-sort-dfs)**

---

# 808-topological-sort-dfs — Topological Sort (DFS)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Topological sort orders the vertices of a Directed Acyclic Graph (DAG) so that every edge goes from earlier to later in the order. It is fundamental to build systems (Makefile, Cargo, Bazel — dependency ordering), task schedulers, course prerequisite ordering, and package managers. The DFS-based algorithm by Tarjan (1976) runs in O(V+E) and also detects cycles (where topological sort is undefined).

## Learning Outcomes

- Implement DFS-based topological sort using a three-color marking (0=unvisited, 1=in-progress, 2=done)
- Detect cycles: a back edge to an in-progress vertex (color=1) indicates a cycle
- Return `None` for cyclic graphs and `Some(Vec<usize>)` for DAGs
- Reverse the DFS finish order to get topological order
- Compare with Kahn's BFS algorithm (alternative approach using in-degree counts)

## Rust Application

`topological_sort(n, edges)` uses `visited: Vec<u8>` with states 0 (unvisited), 1 (gray/in-progress), 2 (black/done). `dfs` returns `false` if a cycle is found (back edge to gray node). Vertices are pushed to `result` on finish (post-order). After all DFS calls, `result.reverse()` gives topological order. Returns `None` if any DFS detects a cycle.

## OCaml Approach

OCaml implements with `color: int array` and `let rec dfs v = ...`. The `exception Cycle` can short-circuit the entire DFS cleanly. OCaml's `List.rev` reverses the finish order. `Ocamlgraph.Topological.fold` provides a functional topological traversal. Kahn's algorithm (in-degree + queue) is simpler to implement in OCaml due to its BFS nature.

## Key Differences

1. **Three-color DFS**: Both languages use 0/1/2 coloring to detect cycles; Rust's `u8` array and OCaml's `int array` are equivalent.
2. **Cycle detection**: Rust returns `false` / `None`; OCaml can raise an exception (`exception Cycle`) for a more functional early-exit.
3. **Kahn's alternative**: Kahn's algorithm (BFS + in-degree) is simpler to reason about; DFS-based is more common in compiler implementations.
4. **Build systems**: Cargo uses topological sort for compilation order; Bazel and Gradle use variants for distributed build graphs.

## Exercises

1. Implement Kahn's BFS-based topological sort as an alternative and verify it produces a valid ordering (not necessarily the same one as DFS).
2. Find all valid topological orderings of a small DAG and count them — this is #P-hard in general but feasible for small graphs.
3. Implement a parallel topological execution scheduler: process all vertices with in-degree 0 in parallel, then decrement in-degrees and add newly freed vertices.
