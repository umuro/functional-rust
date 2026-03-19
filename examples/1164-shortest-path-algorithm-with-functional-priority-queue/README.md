# Example 1164: Shortest Path Algorithm with Functional Priority Queue

**Difficulty:** ⭐⭐⭐
**Category:** Graphs
**OCaml Source:** https://rosettacode.org/wiki/Dijkstra%27s_algorithm#OCaml

## Problem Statement

Implement Dijkstra's shortest-path algorithm using a functional priority queue — an ordered set that provides O(log n) insert and min-element extraction, mirroring how OCaml uses `Set.Make` as a persistent priority queue.

## Learning Outcomes

- How Rust's `BinaryHeap<Reverse<...>>` replaces OCaml's `Set.Make`-based priority queue
- Why lazy deletion (skipping stale heap entries) is idiomatic in both languages
- How `BTreeSet` can mirror OCaml's ordered-set priority queue in functional style
- The ownership pattern of cloning to release borrows before mutating collections

## OCaml Approach

OCaml uses `Set.Make` with a `(int * string)` comparison to create a sorted set that acts as a functional priority queue — always ordered by `(distance, node)`, with the minimum element retrievable in O(log n). The algorithm is tail-recursive via a local `go` function, threading the priority queue and distance map as immutable values that are replaced on each step.

## Rust Approach

The idiomatic Rust version uses `BinaryHeap<Reverse<(u32, String)>>` — a max-heap wrapped with `Reverse` to invert the ordering into a min-heap. Stale entries (where a shorter path was found later) are skipped lazily when popped. The functional version uses `BTreeSet<(u32, String)>` which directly mirrors OCaml's `Set.Make`: a sorted set where the smallest `(distance, node)` pair is always first in iteration order.

## Key Differences

1. **Priority queue representation:** OCaml uses `Set.Make` (persistent ordered set); Rust uses `BinaryHeap<Reverse<...>>` (imperative) or `BTreeSet` (structural analog to Set.Make)
2. **Mutability:** OCaml threads immutable maps through recursion; Rust mutates `HashMap`/`BTreeMap` and the heap in place
3. **Stale entry handling:** Both use lazy deletion — OCaml checks `if d > dist_u then go pq' dist`, Rust checks `if d > dist[u] { continue }`
4. **Borrow constraint:** Rust must `.cloned()` the BTreeSet's first element to release the immutable borrow before calling `.remove()` — OCaml's persistent data structures have no such constraint
