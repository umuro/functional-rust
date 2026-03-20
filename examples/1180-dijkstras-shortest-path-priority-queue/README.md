# Example 1180: Dijkstra's Shortest Path — Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** Functional algorithms using Set.Make + Map.Make as ordered priority queue and distance map

## Problem Statement

Find the shortest path distances from a start node to all reachable nodes in a weighted directed graph using Dijkstra's algorithm.

## Learning Outcomes

- How `BinaryHeap<Reverse<T>>` implements a min-heap — the idiomatic Rust priority queue
- How `BTreeSet<(u32, String)>` mirrors OCaml's `Set.Make` as a sorted priority queue
- How stale-entry lazy deletion replaces decrease-key in a heap-based approach
- How `.fold()` threads mutable state through a functional loop, mirroring `List.fold_left`

## OCaml Approach

OCaml uses `Set.Make` with a custom comparator on `(int * string)` as a purely functional priority queue ordered by `(distance, node)`. `Map.Make(String)` stores distances immutably. The tail-recursive `go pq dist` function is the workhorse, processing the minimum element via `PQ.min_elt` and building new versions of `pq` and `dist` on each step.

## Rust Approach

The idiomatic version uses `BinaryHeap<Reverse<(u32, String)>>` — Rust's max-heap wrapped with `Reverse` to get min-heap semantics. Stale entries are left in the heap and skipped when popped, avoiding the need for a decrease-key operation. The functional version uses `BTreeSet<(u32, String)>` (naturally sorted ascending) to directly mirror the OCaml `Set.Make` approach with a tail-recursive `go` function and `.fold()` for neighbor relaxation.

## Key Differences

1. **Priority queue:** OCaml `Set.Make` is a balanced BST (purely functional); Rust idiomatic uses `BinaryHeap` (mutable, O(log n) push/pop). Both order by `(distance, node)`.
2. **Lazy deletion:** OCaml removes the minimum and never re-inserts duplicates (purely functional — no stale entries possible). Rust's heap allows multiple entries per node; stale ones are skipped with a guard check.
3. **Distance map:** OCaml uses `Map.Make(String)` (immutable, persistent). Rust uses `HashMap` (mutable) or `BTreeMap` (ordered, closer to OCaml).
4. **Ownership vs. persistence:** OCaml creates new maps/sets on every update. Rust takes ownership and mutates in place — semantically equivalent but physically different.
