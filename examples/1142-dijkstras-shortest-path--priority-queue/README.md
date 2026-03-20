# Example 1142: Dijkstra's Shortest Path — Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** Classic algorithm, functional implementation with `Set` as priority queue

## Problem Statement

Find the shortest distance from a start node to every reachable node in a
weighted directed graph using Dijkstra's algorithm.

## Learning Outcomes

- How Rust's `BinaryHeap` with `std::cmp::Reverse` replaces OCaml's ordered `Set` as a min-priority queue
- Using `HashMap` for mutable distance tracking versus OCaml's persistent `Map`
- Mirroring OCaml's `List.fold_left` with Rust's `.fold()` over neighbour slices
- Lifetime annotations on graph references when nodes are string slices

## OCaml Approach

OCaml uses a functionally-ordered `Set` module (sorted by `(dist, node)`) as the
priority queue. The algorithm is a tail-recursive `go` helper that passes `pq`
and `dist` as immutable values through each iteration, updating them via
`PQ.add`/`SMap.add` which return new persistent structures.

## Rust Approach

Rust uses `BinaryHeap<(Reverse<u32>, &str)>` — a max-heap made into a min-heap
with `Reverse` — which gives O(log n) push/pop matching OCaml's `Set` insert/min_elt.
The imperative version mutates `dist: HashMap` in place; the functional version
threads ownership through a recursive helper with `.fold()` mirroring OCaml exactly.

## Key Differences

1. **Priority queue type:** OCaml uses a persistent ordered `Set`; Rust uses a mutable `BinaryHeap` with `Reverse` for min-heap semantics
2. **Distance map:** OCaml's `Map` is persistent (each update creates a new map); Rust's `HashMap` is mutable and updated in place
3. **Stale entry handling:** OCaml's `Set` guarantees uniqueness so stale entries don't arise; Rust's heap may hold duplicates — filtered with a `d > best` guard
4. **Ownership:** Rust node names are `&'a str` slices borrowing from the graph; OCaml strings are heap-allocated and shared by the GC

## Exercises

1. Modify the implementation to return the full shortest path as a `Vec<NodeId>` in addition to the distance, by tracking predecessors during relaxation.
2. Add support for finding shortest paths from a single source to all reachable nodes, returning a `HashMap<NodeId, u64>` of distances.
3. Extend the graph representation to support negative-weight detection: run a Bellman–Ford pass after Dijkstra and verify results agree on non-negative graphs.
