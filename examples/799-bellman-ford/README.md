📖 **[View on hightechmind.io →](https://hightechmind.io/rust/799-bellman-ford)**

---

# 799-bellman-ford — Bellman-Ford Algorithm
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Bellman-Ford (1958/1965) finds shortest paths from a source to all vertices in a weighted graph, handling negative edge weights that Dijkstra cannot handle. It also detects negative-weight cycles. Used in network routing (RIP — Routing Information Protocol uses Bellman-Ford), currency arbitrage detection, and financial risk analysis. The O(VE) time complexity is slower than Dijkstra's O((V+E)logV) but handles a broader class of problems.

## Learning Outcomes

- Implement Bellman-Ford with V-1 relaxation rounds over all edges
- Detect negative-weight cycles by checking if any edge can still be relaxed after V-1 rounds
- Understand why V-1 rounds suffice: any shortest path has at most V-1 edges
- Return `None` for graphs with negative cycles (shortest path is undefined)
- Compare with Dijkstra: Bellman-Ford is slower but handles negative weights

## Rust Application

`bellman_ford(n, edges, src)` initializes `dist[src] = 0`, all others to `i32::MAX`. Performs n-1 relaxation rounds over all edges. After n-1 rounds, checks for remaining relaxations — if any exist, returns `None` (negative cycle). Tests include the classic 5-node example with mixed positive and negative weights, and the negative-cycle detection case.

## OCaml Approach

OCaml implements with `Array.make n max_int` and nested `for` loops. The negative cycle check adds one more relaxation attempt. Functional style using `List.iter` over edges is idiomatic. OCaml's `Hashtbl` can represent the adjacency list. The `Int.max_int / 2` sentinel avoids overflow when adding edges to "infinity" values.

## Key Differences

1. **Overflow guard**: Rust uses `i32::MAX` as infinity and guards with `!= i32::MAX` before adding; OCaml uses `max_int/2` to avoid overflow — both serve the same purpose.
2. **Negative cycle**: Both languages return an error/option on negative cycle detection, propagating the undefined result.
3. **SPFA optimization**: The "Shortest Path Faster Algorithm" uses a queue to skip relaxations that won't improve; this optimization applies equally to both languages.
4. **Routing protocols**: RIP uses Bellman-Ford with hop count as weight; BGP uses a path-vector variant; both are implemented in production network equipment software.

## Exercises

1. Implement SPFA (Bellman-Ford with a queue): only relax edges from nodes whose distance was recently updated. Benchmark against naive Bellman-Ford on sparse graphs.
2. Add `bellman_ford_path(n, edges, src, dst) -> Option<Vec<usize>>` that reconstructs the shortest path from src to dst using a predecessor array.
3. Use Bellman-Ford to detect currency arbitrage: given exchange rates as a complete graph with log-weight edges, a negative cycle indicates an arbitrage opportunity.
