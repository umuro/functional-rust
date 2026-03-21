📖 **[View on hightechmind.io →](https://hightechmind.io/rust/800-floyd-warshall)**

---

# 800-floyd-warshall — Floyd-Warshall Algorithm
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Floyd-Warshall (1962) computes shortest paths between ALL pairs of vertices in a weighted graph in O(V³) time. Unlike Dijkstra (single source) or Bellman-Ford (single source with negative weights), Floyd-Warshall solves the all-pairs problem directly. It is used in network distance matrices, routing table computation, social graph analysis (degree of separation), and computing transitive closures.

## Learning Outcomes

- Initialize the distance matrix from edges and set `dist[i][i] = 0`
- Apply the DP recurrence: `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`
- Understand why the order of loops (k outer, then i, j) is correct
- Detect negative cycles via `dist[i][i] < 0` after the algorithm
- Use Floyd-Warshall as a transitive closure algorithm (boolean variant)

## Rust Application

`floyd_warshall(n, edges)` initializes `dist: Vec<Vec<i32>>` with `i32::MAX/2` (to avoid overflow on addition) and zeros on the diagonal. Edges are loaded directly. The triple loop `k, i, j` updates `dist[i][j] = min(dist[i][j], dist[i][k]+dist[k][j])`. The test verifies that `dist[0][2] == 5` for a 3-node graph. The division-by-2 of MAX prevents overflow when adding two "infinity" values.

## OCaml Approach

OCaml uses `Array.make_matrix n n max_int` with `max_int/2` as the convention. The triple for-loop is identical. OCaml's `min` function and array mutation follow the same structure. For transitive closure, replace `int` with `bool` and `min` with `||`. The `Ocamlgraph` library implements Floyd-Warshall as part of its path algorithms suite.

## Key Differences

1. **All-pairs vs single-source**: Floyd-Warshall is the only practical all-pairs algorithm; Rust programs that need all-pairs often run Dijkstra V times instead for sparse graphs.
2. **Overflow prevention**: Both languages divide the infinity sentinel by 2; Rust's `i32::MAX/2` and OCaml's `max_int/2` serve the same purpose.
3. **Negative cycles**: `dist[v][v] < 0` after the algorithm indicates a negative cycle; both languages detect this the same way.
4. **Transitive closure**: Boolean Floyd-Warshall (with `||` instead of min) computes reachability; OCaml's `Ocamlgraph.Fixpoint` does this automatically for labeled graphs.

## Exercises

1. Implement the boolean transitive closure variant: replace `i32` with `bool` and use `||` and `&&` instead of min/add.
2. Detect and report all negative cycles: after running Floyd-Warshall, find all vertices v where `dist[v][v] < 0` and trace which cycle they belong to.
3. Use Floyd-Warshall to compute "six degrees of separation" in a social network: load a friend graph and find the average shortest path length between all pairs.
