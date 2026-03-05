# 800. Floyd-Warshall: All-Pairs Shortest Paths

**Difficulty:** 4  **Level:** Advanced

Compute shortest paths between every pair of nodes in O(V³) — with path reconstruction via a `next` matrix and negative-cycle detection.

## The Problem This Solves

When you need distances between *all* pairs of nodes — not just from one source — Floyd-Warshall is the right tool. Network latency matrices, road distance tables, transitive closure of reachability, and "six degrees of separation" style social network analysis all need all-pairs distances. Floyd-Warshall is simpler to implement than running Dijkstra V times (especially when negative edges are present), and for dense graphs where E ≈ V², it's asymptotically comparable.

The `next` matrix variant makes path reconstruction O(path length) after O(V³) preprocessing — extremely useful when you're building a routing table that will be queried many times.

## The Intuition

Ask: "does routing through node k improve the path from i to j?" Do this for every k from 0 to V-1. After considering all intermediate nodes, `dist[i][j]` holds the shortest path. This is the "intermediate node induction" insight: after the outer loop over k, `dist[i][j]` is the shortest path that only uses nodes 0..=k as intermediates. The diagonal check `dist[i][i] < 0` after the main loop reveals negative cycles. OCaml implements this with three nested loops on a 2D array; Rust uses exactly the same structure with `Vec<Vec<i64>>`.

## How It Works in Rust

```rust
const INF: i64 = i64::MAX / 2;

// O(V³) time, O(V²) space
fn floyd_warshall(
    n: usize,
    edges: &[(usize, usize, i64)],
) -> (Vec<Vec<i64>>, Vec<Vec<Option<usize>>>, bool) {
    let mut dist = vec![vec![INF; n]; n];
    let mut next: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];

    for i in 0..n { dist[i][i] = 0; }
    for &(u, v, w) in edges {
        if w < dist[u][v] {
            dist[u][v] = w;
            next[u][v] = Some(v);
        }
    }

    // Main loop: try routing through each intermediate node k
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] < INF && dist[k][j] < INF {
                    let via = dist[i][k] + dist[k][j];
                    if via < dist[i][j] {
                        dist[i][j] = via;
                        next[i][j] = next[i][k];  // route through k
                    }
                }
            }
        }
    }

    // Negative cycle: any node with negative self-distance
    let neg_cycle = (0..n).any(|i| dist[i][i] < 0);
    (dist, next, neg_cycle)
}

// Path reconstruction: follow next[src][dst] until we arrive
fn reconstruct(next: &Vec<Vec<Option<usize>>>, src: usize, dst: usize) -> Option<Vec<usize>> {
    if next[src][dst].is_none() { return None; }
    let mut path = vec![src];
    let mut v = src;
    while v != dst { v = next[v][dst]?; path.push(v); }
    Some(path)
}
```

The `next[i][j] = next[i][k]` update in the reconstruction matrix is subtle: when you route i→j through k, the *first hop* from i is still `next[i][k]` — because the full path from k to j is already encoded in the `next` matrix.

## What This Unlocks

- **Network latency tables**: precompute latency between all datacenter pairs for SLA routing decisions; rebuild when topology changes.
- **Transitive closure**: set all edge weights to 1, run Floyd-Warshall, then `reachable[i][j] = dist[i][j] < INF`. Simpler than DFS/BFS for static dense graphs.
- **Social network analysis**: "degrees of separation" between all user pairs, or finding the node that minimises average distance to all others (graph centre).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D matrix | `Array.make_matrix n n inf` | `vec![vec![INF; n]; n]` |
| `next` matrix | `int option array array` | `Vec<Vec<Option<usize>>>` |
| Loop order | `for k`, `for i`, `for j` | Identical — order is critical |
| Negative cycle | Check diagonal after loop | `.any(|i| dist[i][i] < 0)` |
| Path reconstruction | Recursive via `next` | Iterative `while v != dst` with `?` propagation |
