# Example 1085: Dijkstra's Shortest Path
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Find the shortest distances from a source node to all reachable nodes in a weighted directed graph using Dijkstra's algorithm. The graph is represented as an adjacency list, and edge weights are non-negative integers. The algorithm must efficiently select the next lowest-cost node at each step, making priority queue design a central concern.

## Learning Outcomes

- How to turn Rust's max-heap `BinaryHeap` into a min-heap by reversing `Ord` on a wrapper `State` type
- How to build an adjacency list functionally using `fold` over a flat list of `(from, to, weight)` edge tuples
- Why stale heap entries can replace a separate "visited" set: skip any popped state whose recorded cost is already lower
- How `shortest_path` uses iterator chaining (`flat_map`, `filter`, `map`) to backtrack through the distance map without mutable bookkeeping
- How Rust's ownership model makes accidental aliasing in graph algorithms a compile-time error rather than a runtime bug

## OCaml Approach

OCaml builds the graph with `List.fold_left` over a list of tuples, storing neighbors in an `IntMap`. The search driver `loop` is a tail-recursive function that pattern-matches on a sorted association list used as a priority queue. Each recursive call passes updated distance map `dist'` and heap `heap'` as accumulators, making the entire control flow purely functional — no mutable cells, no early returns, no imperative break.

The OCaml implementation calls `List.sort compare` on the priority list after each expansion, giving O(n log n) per step. This is less efficient than a binary heap but makes the algorithm's logic transparent in a few lines of code.

## Rust Application

`Graph` is a `HashMap<usize, Vec<Edge>>` constructed by `build_graph` using `fold` over a slice of edge tuples. `dijkstra` drives the search with a `BinaryHeap<State>` where `State` implements reversed `Ord` to achieve min-heap semantics. Distance updates use `HashMap` insertions rather than in-place mutation, keeping each update explicit. The `shortest_path` function reconstructs the path by chaining `flat_map` and `filter` over the adjacency list to find predecessor nodes from the distance map.

## Key Differences

1. **Priority queue:** Rust reverses `Ord` on a `State` newtype to coerce the standard `BinaryHeap` (max-heap) into min-heap behavior; OCaml uses a sorted list re-sorted after each expansion — simpler but O(n²) overall
2. **Distance map:** Rust uses a `HashMap` with explicit `insert` calls; OCaml uses a persistent `IntMap` where each `add` returns a new map, making immutability structural rather than stylistic
3. **Visited tracking:** Rust avoids a visited set by comparing the popped cost against the stored distance and skipping stale entries; OCaml threads the same check through the recursive accumulator pattern
4. **Path reconstruction:** Rust chains iterator combinators (`flat_map`, `filter`, `map`) for backtracking; OCaml would use a separate recursive pass over the distance map with pattern matching on `IntMap` entries

## Exercises

1. Extend `dijkstra` to return the full shortest path (sequence of node IDs) in addition to the distance, without calling the separate `shortest_path` function — thread a predecessor map through the loop instead
2. Replace the distance `HashMap` with a purely functional approach using a recursive helper that passes state as a parameter, eliminating all interior mutation
3. Adapt the implementation to handle undirected graphs by inserting both directions in `build_graph`, and verify correctness on a graph with cycles
4. Benchmark the sorted-list OCaml approach against the BinaryHeap Rust approach on a dense graph with 1000 nodes — measure how queue operations dominate runtime
