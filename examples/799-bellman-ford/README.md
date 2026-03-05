📖 **[View on hightechmind.io →](https://hightechmind.io/rust/799-bellman-ford)**

---

# 799. Bellman-Ford: Shortest Paths with Negative Edges

**Difficulty:** 4  **Level:** Advanced

Single-source shortest paths that handle negative edge weights — and detect negative-weight cycles — in O(V × E).

## The Problem This Solves

Dijkstra's algorithm breaks on negative edge weights. Bellman-Ford doesn't. In financial networks, negative edges model arbitrage opportunities: a cycle of currency exchanges where you end up with more than you started with. In network routing protocols (like RIP and BGP), Bellman-Ford is the basis for distance-vector routing because it tolerates negative link costs and detects routing loops. In constraint satisfaction, "difference constraints" (xi - xj ≤ w) are solved as shortest-path problems with Bellman-Ford.

The negative cycle detection is as important as the shortest path itself: a negative cycle means "no well-defined shortest path exists" — the path can be made arbitrarily short by looping. Detecting this is essential before trusting any distance output.

## The Intuition

Relax all edges V-1 times. After k rounds of relaxation, you have the correct shortest path for all paths using at most k edges. Since any simple shortest path in a V-node graph uses at most V-1 edges, V-1 rounds suffice. Then run one more round: if any distance still decreases, you've found a negative-weight cycle. OCaml implements this with a recursive loop or `for` loop over an edge list; Rust uses the same imperative structure with a flat `Vec<(usize, usize, i64)>` edge list — more cache-friendly than an adjacency list for Bellman-Ford's edge-scanning pattern.

## How It Works in Rust

```rust
const INF: i64 = i64::MAX / 2;  // half of MAX to avoid overflow on addition

// O(V × E) time, O(V) space
fn bellman_ford(
    n: usize,
    edges: &[(usize, usize, i64)],
    src: usize,
) -> (Vec<i64>, Vec<Option<usize>>, bool) {
    let mut dist = vec![INF; n];
    let mut prev = vec![None; n];
    dist[src] = 0;

    // V-1 relaxation rounds
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] < INF && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                prev[v] = Some(u);
            }
        }
    }

    // Round V: detect negative cycles
    let neg_cycle = edges.iter().any(|&(u, v, w)| {
        dist[u] < INF && dist[u] + w < dist[v]
    });

    (dist, prev, neg_cycle)
}

// Reconstruct path: follow prev[] backwards, then reverse
fn reconstruct(prev: &[Option<usize>], dst: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut v = dst;
    loop {
        path.push(v);
        match prev[v] { Some(p) => v = p, None => break }
    }
    path.reverse();
    path
}
```

Using `INF = i64::MAX / 2` prevents overflow: `INF + w` stays positive for any reasonable edge weight. The `any` closure on round V elegantly expresses the negative-cycle check.

## What This Unlocks

- **Arbitrage detection**: model currency exchange rates as `log(1/rate)` edge weights; a negative-weight cycle in this graph signals a profitable arbitrage loop.
- **Distance-vector routing**: Bellman-Ford is the core of RIP (Routing Information Protocol) and historically of early internet routing. Each router maintains distance estimates and relaxes them through neighbor updates.
- **Difference constraints**: a system of constraints `xj - xi ≤ wij` is feasible iff the corresponding shortest-path graph has no negative cycle — Bellman-Ford solves this directly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Edge list | List of tuples or array | `Vec<(usize, usize, i64)>` — flat, cache-friendly |
| Infinity | `max_int` | `i64::MAX / 2` to avoid overflow |
| Negative cycle detection | Extra loop or `exception` | `.any()` iterator combinator over edges |
| Path reconstruction | Recursive via `prev` array | `loop { match prev[v] }` iterative |
| Predecessor array | `option array` | `Vec<Option<usize>>` |
