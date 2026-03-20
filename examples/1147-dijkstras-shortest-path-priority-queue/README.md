# Example 1147: Dijkstra's Shortest Path — Functional Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** 99 OCaml Problems / classic graph algorithms

## Problem Statement

Find the shortest path from a start node to all reachable nodes in a weighted directed
graph. This example emphasises the functional priority-queue idiom: OCaml uses
`Set.Make` as an ordered-set priority queue; Rust mirrors this with `BTreeSet`.

## Learning Outcomes

- How OCaml's `Set.Make` doubles as a min-heap via ordered-set semantics
- How Rust's `BTreeSet<(usize, String)>` is the structural equivalent
- How `BTreeMap` mirrors OCaml's `Map.Make(String)` — deterministic sorted output
- How OCaml's `let rec go pq dist` tail-recursive loop translates to a Rust `fn go(…)` recursive helper with `fold` over neighbours

## OCaml Approach

OCaml uses `Set.Make` with a custom comparator on `(int * string)` tuples as a
functional priority queue. `PQ.min_elt` extracts the minimum cheaply; `PQ.remove`
shadows the binding to produce a new set. `Map.Make(String)` holds distances
immutably. The inner `let rec go pq dist` is a tail-recursive loop that the
OCaml compiler optimises into iteration.

## Rust Approach

`BTreeSet<(usize, String)>` is Rust's structural analogue: tuples compare
lexicographically, so the minimum-distance entry is always first. `iter().next()`
is `PQ.min_elt`; `remove` is `PQ.remove`. `BTreeMap<String, usize>` mirrors
`Map.Make(String)` and produces sorted output without an explicit sort step.
A second implementation uses a recursive `go` helper with `Iterator::fold` over
neighbours, matching the OCaml `List.fold_left` pattern directly.

## Key Differences

1. **Priority queue**: OCaml uses `Set.Make` (functional BST); Rust uses `BTreeSet` (mutable B-tree). Both give O(log n) min-extraction and O(log n) insertion.
2. **Distance map**: OCaml's `Map.Make(String)` is persistent/immutable; Rust's `BTreeMap` is mutable but produces the same sorted-key iteration order.
3. **Tail recursion**: OCaml's TCO turns `let rec go` into a loop automatically; Rust's `fn go` recurses on the stack — safe for small graphs, not for production-scale graphs.
4. **Stale entries**: Both approaches allow duplicate `(dist, node)` entries in the priority queue; the `alt < current` guard ensures only improvements are enqueued.

## Exercises

1. Generalize the graph representation to use a generic edge-weight type `W: Ord + Add + Zero` so the same Dijkstra function works on both integer and floating-point graphs.
2. Implement `eccentricity` — the maximum shortest-path distance from a given node to any reachable node — and use it to compute the graph's diameter and radius.
3. Modify Dijkstra to return a `Stream`-like iterator that yields `(node, distance)` pairs in discovery order, enabling early termination once a target node is reached.
