# Example 100: Dijkstra's Shortest Path

**Difficulty:** ⭐⭐⭐
**Category:** [Graph Algorithms | Priority Queues | Functional Data Structures]
**OCaml Source:** Classic graph algorithm with functional priority queue

## Problem Statement

Given a weighted directed graph and a source node, find the shortest distance from the source to every reachable node. This is Dijkstra's algorithm — one of the most important algorithms in computer science — implemented with functional programming patterns in both OCaml and Rust.

## Learning Outcomes

- Implement **graph algorithms** using functional patterns (folds, immutable maps)
- Understand how **ownership** in Rust prevents aliasing bugs common in mutable graph code
- Compare **functional priority queues** (OCaml sorted list) vs **Rust's BinaryHeap** (reversed for min-heap)
- Practice **pattern matching** on complex data structures
- Build **path reconstruction** through backtracking on distance maps

## OCaml Approach

OCaml uses a purely functional priority queue (sorted association list) and immutable `IntMap` for distances. The algorithm is expressed as a recursive loop with pattern matching:

```ocaml
let rec loop pq dist =
  match PQ.pop pq with
  | None -> dist
  | Some ((d, u), pq') ->
    (* fold over neighbors, accumulate new distances *)
```

Key features:
- **No mutation** — distance map is rebuilt at each step via `IntMap.add`
- **Functional PQ** — insertion maintains sorted order, pop returns head
- **Pattern matching** — clean None/Some dispatch replaces while loops

## Rust Approach

Rust uses `BinaryHeap` (reversed via custom `Ord` for min-heap behavior) and `HashMap` for distances. While the maps are technically mutable, the algorithm follows a functional accumulation pattern:

```rust
while let Some(State { cost, node }) = heap.pop() {
    if cost > *dist.get(&node).unwrap_or(&u64::MAX) {
        continue; // skip stale — functional "already visited"
    }
    // fold over neighbors...
}
```

Key features:
- **Ownership prevents aliasing** — no accidental shared references to graph nodes
- **Custom Ord** — Rust's max-heap becomes min-heap via reversed comparison
- **Zero-cost abstractions** — iterator chains compile to the same code as manual loops
- **Path reconstruction** — functional backtracking through the distance map

## Key Differences

1. **Mutability:** OCaml's `IntMap` is truly immutable (new map per update). Rust's `HashMap` is mutated in place but owned — Rust's type system prevents the bugs that mutation usually causes.

2. **Priority Queue:** OCaml builds a functional sorted list (O(n) insert). Rust uses `BinaryHeap` (O(log n) insert/pop) but needs a wrapper type with reversed `Ord` since Rust only provides max-heap.

3. **Type Safety:** Both languages prevent type errors, but Rust additionally prevents iterator invalidation and use-after-free — critical in graph algorithms where nodes reference other nodes.

4. **Performance:** Rust's version is closer to C speed (no GC, no boxing of integers). OCaml's purely functional version allocates more but is elegant and correct by construction.

## Exercises

1. Add bidirectional edges and verify shortest paths update correctly
2. Implement A* search by adding a heuristic function parameter
3. Build a functional version in Rust using `im::HashMap` (persistent data structure crate)
4. Add negative edge detection and return an error if found (Dijkstra doesn't handle negative weights)
