📖 **[View on hightechmind.io →](https://hightechmind.io/rust/380-weighted-graph)**

---

# 380: Weighted Graph and Dijkstra's Algorithm
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Edge weights model real costs: road distances, network latency, flight prices, pipeline capacities. Edsger Dijkstra introduced his shortest-path algorithm in 1956 to find minimum-cost routes in graphs with non-negative weights. It runs in O((V + E) log V) with a binary heap and is the foundation of GPS routing, internet packet routing (OSPF protocol), game AI pathfinding, and transportation optimization.

Dijkstra's algorithm is implemented in every major programming language's standard library ecosystem and powers Google Maps, Cisco routers, and real-time strategy game unit movement.

## Learning Outcomes

- Understand why Dijkstra's algorithm requires non-negative edge weights (negative edges require Bellman-Ford)
- Learn how a min-heap (priority queue) drives the greedy selection of the next closest vertex
- Understand the `Reverse` wrapper for converting Rust's max-heap `BinaryHeap` into a min-heap
- Learn path reconstruction via parent pointer arrays alongside distance arrays
- See how `u64::MAX` as infinity enables clean relaxation logic with `saturating_add`

## Rust Application

The `dijkstra` function in `src/lib.rs` uses `BinaryHeap<Reverse<(u64, usize)>>` — wrapping with `Reverse` turns the standard max-heap into a min-heap. The relaxation loop uses `saturating_add` to prevent overflow when summing distances. The stale-entry check (`if d > dist[u] { continue }`) is the standard lazy-deletion optimization that avoids a decrease-key operation. `dijkstra_with_path` adds a `prev` array for path reconstruction by walking backwards from destination to source.

## OCaml Approach

OCaml implements Dijkstra using a priority queue module (from the standard library or a third-party `psq` package). The functional approach tracks distances in a `Map` rather than a mutable array, creating a new map at each relaxation step. An imperative version uses `Bigarray` or `Array` for distances. OCaml's `compare` functions work with tuples directly for heap ordering.

## Key Differences

1. **Min-heap**: Rust wraps with `Reverse` to invert `BinaryHeap` ordering; OCaml priority queues natively support custom comparators or use `(dist, vertex)` tuple ordering directly.
2. **Infinity representation**: Rust uses `u64::MAX` with `saturating_add`; OCaml uses `max_int` or `infinity` (float) or a custom `type distance = Inf | Finite of int`.
3. **Path reconstruction**: Rust uses a `Vec<usize>` parent array indexed by vertex; OCaml uses a `Hashtbl` or `Map` mapping vertex to previous vertex.
4. **Mutability**: Rust's distance array is `mut Vec<u64>` updated in place; OCaml's functional approach would use a persistent map, which is slower but produces an immutable audit trail.

## Exercises

1. **Bellman-Ford**: Implement Bellman-Ford shortest paths which handles negative edge weights, and add cycle detection that identifies negative-weight cycles.
2. **All-pairs shortest paths**: Implement Floyd-Warshall for dense graphs, returning a 2D distance matrix, and compare runtime with running Dijkstra from every vertex.
3. **A* search**: Extend Dijkstra with a heuristic function `h(v) -> u64` that estimates remaining distance to goal, turning it into A* for grid-based pathfinding. Use Manhattan distance as the heuristic.
