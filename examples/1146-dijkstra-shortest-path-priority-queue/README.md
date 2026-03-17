# Example 1146: Dijkstra's Shortest Path with Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** Classic algorithm using Set.Make as a functional priority queue

## Problem Statement

Given a weighted directed graph and a start node, compute the shortest distance from the start to every reachable node using Dijkstra's algorithm.

## Learning Outcomes

- How OCaml's `Set.Make` doubles as a priority queue via lexicographic ordering on `(distance, node)` tuples
- How Rust's `BinaryHeap<Reverse<...>>` provides the equivalent min-heap with zero extra dependencies
- Stale-entry detection: Dijkstra with a lazy deletion heap vs OCaml's purely functional priority queue
- How the borrow checker shapes the functional Rust version: collect relaxed edges before mutating dist/heap

## OCaml Approach

OCaml uses `Set.Make` with a custom comparator on `(int * string)` as an ordered priority queue — `min_elt` extracts the closest node in O(log n). The distance map is a persistent `Map.Make(String)`. The algorithm is a tail-recursive loop (`go pq dist`) using `List.fold_left` over neighbors, producing new immutable `dist` and `pq` values at each step.

## Rust Approach

Rust uses `std::collections::BinaryHeap<Reverse<(usize, String)>>` as a min-heap. The `Reverse` wrapper flips the max-heap into a min-heap — identical semantics to OCaml's ordered set. Distance tracking uses a `HashMap<String, usize>`. Stale entries are skipped at pop time rather than removed eagerly, which is idiomatic for Rust's mutable heap.

## Key Differences

1. **Priority Queue:** OCaml `Set.Make` is a persistent balanced BST; Rust `BinaryHeap` is a mutable heap — both give O(log n) insert and O(log n) min extraction.
2. **Immutability:** OCaml's `dist` and `pq` are immutable values threaded through recursion; Rust uses `mut` bindings updated in a `while let` loop.
3. **Stale entries:** OCaml's `Set.remove` prunes the old entry immediately; Rust's lazy approach pushes duplicate entries and skips them at pop, trading memory for simpler code.
4. **Borrow checker:** The functional Rust version must `.collect()` relaxed edges into a `Vec` before updating `dist` and `heap` — the compiler enforces the separation that OCaml gets "for free" from immutability.
