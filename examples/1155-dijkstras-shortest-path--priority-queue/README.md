# Example 1155: Dijkstra's Shortest Path — Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** Classic graph algorithm using functional ordered sets as a priority queue

## Problem Statement

Find the shortest path distances from a start node to all reachable nodes in a weighted directed graph, using Dijkstra's algorithm with a priority queue.

## Learning Outcomes

- How `BinaryHeap<Reverse<T>>` provides min-heap semantics in Rust (vs OCaml's `Set.Make`)
- Handling stale priority-queue entries with a distance check (lazy deletion)
- Lifetime annotations on graphs that borrow string keys from the caller
- Why `collect()` is sometimes needed to separate immutable and mutable borrows of the same map

## OCaml Approach

OCaml uses a functional `Set` (a balanced BST ordered by `(distance, name)`) as its priority queue — it is immutable, so each step produces a new `pq` and `dist`. The `fold_left` over neighbors threads `(dist, pq)` as an accumulator, mirroring the purely functional style.

## Rust Approach

Rust uses `BinaryHeap<Reverse<(u32, &str)>>` — a standard max-heap wrapped with `Reverse` to get min-heap behaviour. Distances are stored in a mutable `HashMap`. Stale entries (pushed before a shorter path was found) are skipped with a single guard check, avoiding a separate "visited" set.

## Key Differences

1. **Priority queue:** OCaml uses an immutable ordered `Set`; Rust uses a mutable `BinaryHeap` with `Reverse` for min ordering.
2. **Distance map:** OCaml's `SMap` is an immutable functional map returned from each recursion; Rust mutates a `HashMap` in place.
3. **Stale entries:** OCaml removes the exact entry before processing; Rust uses lazy deletion — stale entries stay in the heap and are skipped when popped.
4. **Borrow checker:** Rust cannot hold a shared borrow of `dist` (for `get`) and a mutable borrow (for `insert`) simultaneously in one iterator chain — collecting updates first resolves this.

## Exercises

1. Extend the implementation to support weighted undirected graphs (edges listed once but traversable in both directions) and verify shortest paths are symmetric.
2. Add a `reachable_from` function that returns the set of all nodes reachable from a given source using the Dijkstra search frontier.
3. Use the shortest-path implementation to solve a grid-based pathfinding problem: represent a 2D grid as a graph where blocked cells have infinite-weight edges and find the shortest route from top-left to bottom-right.
