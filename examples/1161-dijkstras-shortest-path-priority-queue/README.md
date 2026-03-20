# Example 1161: Dijkstra's Shortest Path — Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** https://rosettacode.org/wiki/Dijkstra%27s_algorithm#OCaml

## Problem Statement

Given a weighted directed graph and a start node, compute the shortest distance from
the start to every reachable node using Dijkstra's algorithm with a priority queue.

## Learning Outcomes

- How to turn `BinaryHeap` (max-heap) into a min-heap using `std::cmp::Reverse`
- When and why `String` clones are semantically justified (building owned collections)
- How OCaml's functional `Set`-as-priority-queue translates to Rust's imperative `BinaryHeap`
- Mirroring OCaml's recursive `go pq dist` accumulator pattern in Rust using `.fold()`

## OCaml Approach

OCaml uses a `Set.Make` module as a functional priority queue — elements are `(distance, node)`
pairs ordered lexicographically, so `min_elt` always returns the closest unvisited node.
The immutable `SMap` (string map) holds distances and is passed through a tail-recursive `go`
function alongside the priority queue, yielding a purely functional traversal.

## Rust Approach

Rust uses `BinaryHeap<Reverse<(u32, String)>>` as a min-heap (wrapping in `Reverse` flips
the max-heap's ordering). A `HashMap<String, u32>` tracks best-known distances and is
mutated in place. Stale queue entries — nodes re-inserted with a now-worse distance — are
skipped via a guard check, mirroring the OCaml implicit de-duplication from the functional set.

## Key Differences

1. **Priority queue type:** OCaml uses a persistent `Set` (O(log n) insert/delete, no duplicates); Rust uses `BinaryHeap` (O(log n) push/pop, allows duplicates — stale entries skipped manually).
2. **Mutability:** OCaml builds new maps at each step (immutable); Rust mutates `HashMap` in place for efficiency.
3. **Ordering:** OCaml gets min-heap behavior from `Set.min_elt` on `(int, string)` tuples; Rust wraps entries in `Reverse` to flip `BinaryHeap`'s default max-heap ordering.
4. **String ownership:** OCaml strings are garbage-collected and freely shared; Rust requires explicit `.clone()` when the same `String` must live in both the heap and the distance map simultaneously.

## Exercises

1. Implement `shortest_path_tree` that returns a spanning tree (as a parent map) that encodes the shortest path from the source to every reachable node.
2. Modify the priority queue to use a Fibonacci heap interface and discuss the asymptotic improvement (`O(V log V + E)` vs `O((V+E) log V)`).
3. Apply Dijkstra to a time-dependent graph where edge weights change depending on the current cost accumulated so far, and explain why the greedy argument breaks.
