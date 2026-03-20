# Example 1170: Dijkstra's Shortest Path — Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** https://rosettacode.org/wiki/Dijkstra%27s_algorithm#OCaml

## Problem Statement

Find the shortest distance from a start node to every reachable node in a
weighted directed graph. Uses Dijkstra's greedy algorithm driven by a
priority queue that always processes the cheapest unvisited node first.

## Learning Outcomes

- How `BinaryHeap<(Reverse<u32>, String)>` replaces OCaml's ordered `Set.Make` as a min-heap
- How `BTreeMap` replaces OCaml's `Map.Make(String)` for an ordered, functional distance map
- Pattern for "lazy deletion" of stale heap entries instead of a decrease-key operation
- How OCaml's recursive `go pq dist` translates to either an iterative loop or a recursive Rust function

## OCaml Approach

OCaml builds a sorted set (`Set.Make`) keyed on `(dist, node)` pairs to act as a
priority queue — `min_elt` extracts the cheapest entry in O(log n), and the set is
immutable so each step returns a new version. A recursive inner function `go`
threads both the queue and the distance map as pure values.

## Rust Approach

The idiomatic solution wraps `BinaryHeap` (a max-heap) with `std::cmp::Reverse`
to invert ordering, giving an O(log n) min-heap. Because Rust's heap has no
decrease-key, stale entries are left in place and skipped when popped (the
"lazy deletion" pattern). A `BTreeMap` stores distances, matching OCaml's
ordered string map.

## Key Differences

1. **Priority queue type:** OCaml uses an immutable `Set.Make` (balanced BST);
   Rust uses a mutable `BinaryHeap` with `Reverse` for min semantics.
2. **Decrease-key:** OCaml's set supports cheap removal of any element;
   Rust's heap does not, so stale entries accumulate and are filtered on pop.
3. **Immutability:** OCaml's `go` receives new map/set values each call;
   Rust's idiomatic version mutates `dist` and `heap` in a `while let` loop.
4. **Recursive style:** The recursive Rust version uses `BTreeMap<(u32,String),()>`
   as a functional ordered set, closely mirroring the OCaml structure.
