# Example 1099: Dijkstra's Shortest Path with Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Priority Queues
**OCaml Source:** Classic graph algorithms — Dijkstra's algorithm with ordered set as priority queue

## Problem Statement

Implement Dijkstra's single-source shortest path algorithm using a priority queue. Given a weighted directed graph and a start node, compute the minimum distance from the start to every reachable node.

## Learning Outcomes

- How Rust's `BinaryHeap` with `Reverse` serves as a min-priority queue (vs OCaml's `Set.Make`)
- Threading mutable state through a loop vs OCaml's recursive accumulation of immutable maps
- Using `HashMap` for O(1) distance lookups compared to OCaml's `Map` (balanced tree, O(log n))
- How `BTreeSet` in Rust can mirror OCaml's ordered-set-as-priority-queue pattern directly

## OCaml Approach

OCaml uses `Set.Make` with a custom comparator on `(int * string)` tuples to create an ordered set that doubles as a priority queue — `min_elt` always returns the closest unvisited node. Distances are tracked in an immutable `Map.Make(String)`, and the algorithm recurses with `go pq dist`, threading both structures as arguments. `List.fold_left` relaxes neighbors functionally, producing updated `(dist, pq)` pairs without mutation.

## Rust Approach

The idiomatic Rust solution uses `BinaryHeap<Reverse<(u64, &str)>>` as a min-heap priority queue with `HashMap` for O(1) distance lookups. A "stale entry" check skips nodes whose distance was already improved. The functional variant mirrors OCaml closely using `BTreeSet` (ordered, supports `iter().next()` for min extraction) and `fold` for neighbor relaxation. A third variant uses an explicit `HashSet` for visited-node tracking.

## Key Differences

1. **Priority queue:** OCaml uses `Set.Make` (balanced BST, O(log n) insert/remove/min); Rust idiomatically uses `BinaryHeap` (binary heap, O(log n) push/pop) with `Reverse` for min-ordering
2. **Distance map:** OCaml's `Map` is a persistent balanced tree; Rust's `HashMap` provides O(1) amortized lookups with in-place mutation
3. **State threading:** OCaml recurses with `go pq dist` passing immutable structures; Rust mutates `dist` and `heap` in a `while let` loop
4. **Missing keys:** OCaml catches `Not_found` exceptions; Rust uses `.get().unwrap_or(&u64::MAX)` — no exceptions needed
