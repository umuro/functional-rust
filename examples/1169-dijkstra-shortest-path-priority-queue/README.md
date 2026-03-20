# Example 1169: Dijkstra's Shortest Path with a Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs & Algorithms
**OCaml Source:** Classic algorithms — Dijkstra (1959)

## Problem Statement

Find the shortest distances from a start node to all reachable nodes in a
weighted directed graph, using a priority-queue–driven greedy relaxation loop.

## Learning Outcomes

- How OCaml's `Set.Make` ordered set (used as a min-priority-queue) maps to
  Rust's `BinaryHeap<Reverse<…>>` min-heap.
- How OCaml's `Map.Make(String)` immutable map maps to Rust's `HashMap`.
- Why Rust needs the `Reverse` wrapper: `BinaryHeap` is a max-heap by default.
- How OCaml's tail-recursive `rec go pq dist` pattern translates to Rust's
  imperative `while let … = heap.pop()` loop (idiomatic) or a recursive helper
  that passes accumulator state forward (functional style).

## OCaml Approach

OCaml uses two functorised modules: `Set.Make` ordered by `(distance, node)`
acts as a self-sorting priority queue (min element accessible with `min_elt`),
and `Map.Make(String)` stores distances immutably. The algorithm is a
tail-recursive `go` function that shadows `pq` and `dist` on every step,
so the whole loop is expressed as pure data transformation.

## Rust Approach

Rust's `BinaryHeap` is a max-heap; wrapping each entry in `std::cmp::Reverse`
flips the ordering so `pop()` yields the smallest distance first — exactly
what `Set.min_elt` does in OCaml. `HashMap` is mutable and updated in place.
The idiomatic Rust version uses a `while let Some(…) = heap.pop()` loop;
the functional version passes `(heap, dist)` through a recursive helper,
mirroring OCaml's `rec go` directly and using `.iter().fold(…)` for the
neighbour relaxation that OCaml expresses with `List.fold_left`.

## Key Differences

1. **Priority queue:** OCaml uses `Set.Make` (balanced BST, O(log n) insert/delete-min); Rust uses `BinaryHeap` (binary heap, O(log n) push/pop) — same asymptotic cost, different data structure.
2. **Min-heap polarity:** OCaml's `Set.min_elt` naturally returns the smallest element; Rust's `BinaryHeap` is max-first, requiring `Reverse` to achieve the same behaviour.
3. **Stale entries:** OCaml's `Set.remove` ensures each `(distance, node)` pair is unique, preventing re-processing. Rust's heap allows duplicates, so the idiomatic version guards with `if d > dist[u] { continue }`.
4. **Mutation vs immutability:** OCaml's `SMap.add` and `PQ.add` return new values; Rust's `HashMap::insert` and `BinaryHeap::push` mutate in place — the result is the same, but the Rust version avoids allocation per update.

## Exercises

1. Extend the graph to a labeled multigraph where each edge also carries a string label, and return the sequence of edge labels along the shortest path.
2. Implement Johnson's algorithm: reweight negative edges using Bellman–Ford, then run Dijkstra from every node, enabling all-pairs shortest paths on sparse graphs with negative weights.
3. Apply Dijkstra to model a packet-routing simulation: nodes are routers, edges are links with latency, and each router table is computed as the shortest-path tree from that router.
