📖 **[View on hightechmind.io →](https://hightechmind.io/rust/801-prims-mst)**

---

# 801-prims-mst — Prim's Minimum Spanning Tree
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A Minimum Spanning Tree (MST) of a weighted undirected graph connects all vertices with minimum total edge weight. Prim's algorithm (1957) grows the MST greedily from a starting vertex, always adding the cheapest edge connecting the MST to a new vertex. MSTs are used in network design (laying cables or pipes with minimum cost), cluster analysis, and approximation algorithms for traveling salesman.

## Learning Outcomes

- Use a min-heap (`BinaryHeap<Reverse<(weight, vertex)>>`) to always select the cheapest edge
- Mark vertices as visited to avoid cycles
- Build the adjacency list representation from edge list
- Understand the O((V+E)logV) time complexity with a binary heap
- Compare with Kruskal's: Prim's is better for dense graphs, Kruskal's for sparse

## Rust Application

`prims_mst(n, edges)` builds an adjacency list. A `BinaryHeap<Reverse<(i32, usize)>>` starts with `Reverse((0, 0))`. The main loop pops the cheapest edge, skips visited vertices, marks the vertex, adds edge weight to total, and pushes all unvisited neighbors. Returns the total MST weight. Tests use a simple 8-node graph.

## OCaml Approach

OCaml uses `Map` or `Hashtbl` for adjacency lists and `Set` as a priority queue (OCaml's set is a balanced BST, giving O(log n) min extraction). Alternatively, `module PQueue = Set.Make(...)` serves as a min-heap. The `Ocamlgraph` library provides `Kruskal` and `Prim` implementations. OCaml's `Array.iteri` can build adjacency lists from edge arrays efficiently.

## Key Differences

1. **Priority queue**: Rust's `BinaryHeap<Reverse<...>>` is a max-heap turned into a min-heap via `Reverse`; OCaml uses `Set` as a BST-based priority queue.
2. **Immutable traversal**: OCaml can express Prim's as a fold over priority queue states; Rust's imperative while-loop is more straightforward.
3. **Dense vs sparse**: For dense graphs (E ≈ V²), Prim's with an array-based priority queue achieves O(V²) — faster than the heap variant; both languages support this.
4. **Network design**: Real cable-laying optimization uses MST as a lower bound, then adds fault tolerance — a topic in telecommunications engineering.

## Exercises

1. Modify the algorithm to also return the MST edges (not just the total weight), storing `(u, v, weight)` triples.
2. Implement Prim's with an indexed priority queue (decrease-key operation) to achieve O((V+E)logV) without duplicate heap entries.
3. Compare Prim's and Kruskal's (example 802) on the same graph and verify they produce the same total weight (MSTs are not unique but have equal weight).
