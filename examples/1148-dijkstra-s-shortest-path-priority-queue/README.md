# Example 1148: Dijkstra's Shortest Path — Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** Classic algorithm — functional priority-queue style with `Set.Make` and `Map.Make`

## Problem Statement

Find the shortest distances from a start node to every reachable node in a weighted directed graph, using Dijkstra's algorithm with a priority queue.

## Learning Outcomes

- How `BinaryHeap<Reverse<...>>` implements a min-heap for O(log n) priority-queue operations
- How `BTreeSet<(usize, String)>` mirrors OCaml's ordered `Set.Make` as an alternative priority queue
- How `Iterator::fold` mirrors OCaml's `List.fold_left` for accumulating state over neighbors
- Why stale-entry checking replaces OCaml's purely functional distance map immutability

## OCaml Approach

OCaml uses `Set.Make` as a functional priority queue — the minimum element is retrieved via `min_elt` and removed via `remove`, producing a new set each iteration. `Map.Make(String)` stores distances immutably, and `List.fold_left` threads both structures through each neighbor update in a single pass.

## Rust Approach

Idiomatic Rust uses `BinaryHeap<Reverse<(usize, String)>>` as a mutable min-heap with O(log n) push/pop. Since entries are never removed mid-heap, stale entries (superseded by a shorter path) are skipped on pop. A `HashMap<String, usize>` holds current best distances with O(1) lookup.

## Key Differences

1. **Priority queue:** OCaml's `Set.Make` is a balanced BST (immutable, structural sharing); Rust's `BinaryHeap` is a mutable binary heap — faster in practice but requires stale-entry filtering.
2. **Distance map:** OCaml's `Map.Make` is immutable (returns new map on `add`); Rust's `HashMap`/`BTreeMap` is mutated in place.
3. **Fold vs loop:** OCaml's `List.fold_left` is the idiomatic iteration primitive; Rust's `Iterator::fold` is equivalent and used in `dijkstra_functional`.
4. **Sorted output:** OCaml's `Map.Make` iterates in key order by default; Rust's `HashMap` is unordered — use `BTreeMap` when sorted iteration matters.

## Exercises

1. Track the number of edge relaxations performed and compare it between the priority-queue variant and a brute-force `O(V²)` implementation for small dense graphs.
2. Implement bidirectional Dijkstra that searches from both source and target simultaneously and stops when the two frontiers meet, potentially halving the search space.
3. Apply the shortest-path algorithm to a word-ladder puzzle: nodes are words and edges connect words differing by one letter; find the shortest transformation sequence.
