# 809. Max Flow: Ford-Fulkerson with BFS (Edmonds-Karp)

**Difficulty:** 5  **Level:** Master

Find the maximum flow through a network from source to sink using BFS augmenting paths — O(V × E²) with guaranteed termination and integer-optimal solutions.

## The Problem This Solves

Maximum flow models any problem where you're maximising throughput through a constrained network: bandwidth allocation across internet links, pipeline capacity for oil or water, maximum number of edge-disjoint paths (for redundancy), and bipartite matching (which reduces to max-flow). The max-flow min-cut theorem — that maximum flow equals minimum cut capacity — is one of the most elegant results in combinatorial optimisation, with direct applications in computer vision (image segmentation as a graph cut problem).

Edmonds-Karp specifies Fulkerson's algorithm with BFS to find the *shortest* augmenting path (by hop count). This eliminates the pathological cases of pure Ford-Fulkerson that can loop forever on irrational capacities, and gives the O(VE²) bound.

## The Intuition

Maintain a residual capacity matrix: `cap[u][v]` is how much more flow can be sent from u to v. Initially this equals the original capacity. Find any path from source to sink with positive residual capacity (augmenting path). Send as much flow as possible along this path (bottleneck = minimum edge capacity). Update residuals: decrease forward edges, increase backward edges. Repeat until no augmenting path exists. The backward edge `cap[v][u] += flow` is the key insight — it allows "undoing" flow on a path by sending flow backwards. BFS finds shortest augmenting paths, giving the Edmonds-Karp variant.

## How It Works in Rust

```rust
use std::collections::VecDeque;

// O(V × E²) — BFS finds shortest augmenting paths each iteration
// cap: V×V residual capacity matrix (modified in-place)
fn max_flow(cap: &mut Vec<Vec<i64>>, src: usize, snk: usize) -> i64 {
    let n = cap.len();
    let mut total = 0i64;

    loop {
        // BFS to find an augmenting path
        let mut parent = vec![usize::MAX; n];
        parent[src] = src;
        let mut deque = VecDeque::from([src]);

        'bfs: while let Some(u) = deque.pop_front() {
            for v in 0..n {
                if parent[v] == usize::MAX && cap[u][v] > 0 {
                    parent[v] = u;
                    if v == snk { break 'bfs; }
                    deque.push_back(v);
                }
            }
        }

        if parent[snk] == usize::MAX { break; }  // no augmenting path

        // Find bottleneck: min capacity along the path
        let mut flow = i64::MAX;
        let mut v = snk;
        while v != src { let u = parent[v]; flow = flow.min(cap[u][v]); v = u; }

        // Update residual capacities
        v = snk;
        while v != src {
            let u = parent[v];
            cap[u][v] -= flow;   // forward edge: consume capacity
            cap[v][u] += flow;   // backward edge: allow cancellation
            v = u;
        }
        total += flow;
    }
    total
}
```

The dense adjacency matrix `Vec<Vec<i64>>` is used here (O(V²) space) because max-flow algorithms naturally need to look up `cap[u][v]` and `cap[v][u]` in O(1). For sparse graphs, an edge-list representation with forward/backward edge pairs would be more memory-efficient.

## What This Unlocks

- **Bipartite matching**: model as a flow network (source → left nodes → right nodes → sink, all capacity 1); max-flow gives maximum matching. Hungarian algorithm is an alternative but max-flow generalises to weighted matching.
- **Image segmentation (graph cuts)**: model pixels as nodes, similarity as edge capacities; min-cut separates foreground from background. The min-cut equals max-flow by the max-flow min-cut theorem.
- **Network reliability**: maximum number of edge-disjoint paths equals max-flow with unit capacities — measuring how many link failures a network can survive.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Residual matrix | `int array array` | `Vec<Vec<i64>>` modified in-place |
| BFS queue | `Queue.t` | `VecDeque<usize>` |
| Path tracking | `parent` array or `Hashtbl` | `Vec<usize>` with `usize::MAX` as sentinel |
| Bottleneck scan | Recursive path walk | Iterative `while v != src` loop |
| Break from inner BFS | `raise Exit` exception | `break 'bfs` labelled break |
