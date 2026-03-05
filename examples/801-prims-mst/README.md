# 801. Prim's Algorithm: Minimum Spanning Tree

**Difficulty:** 4  **Level:** Advanced

Grow a minimum spanning tree greedily by always adding the cheapest edge crossing the cut — O(E log V) with a binary heap.

## The Problem This Solves

A minimum spanning tree connects all nodes in a weighted graph with the minimum total edge cost while forming no cycles. Network infrastructure design uses MSTs constantly: laying cables between cities, designing circuit board connections, clustering data points by minimum connection cost. Prim's algorithm is the go-to MST method for dense graphs (many edges per node), while Kruskal's (example 802) is preferred for sparse graphs.

Beyond pure infrastructure, MST structure appears in approximation algorithms: the 2-approximation for the Travelling Salesman Problem builds an MST then doubles its edges; hierarchical clustering uses single-linkage dendrogram construction which is equivalent to MST.

## The Intuition

Maintain a "frontier" of nodes in the MST. At each step, pick the cheapest edge from the MST to any not-yet-included node, add that node, and update edge costs for its neighbours. The `key[v]` array tracks the minimum edge weight connecting v to the current MST. A min-heap on `(key[v], v)` efficiently finds the cheapest next addition. Rust's `BinaryHeap` is a max-heap by default, so we wrap entries in `Reverse<(weight, node)>` to simulate a min-heap — the standard Rust idiom for Dijkstra and Prim. OCaml might use a priority queue module; Rust uses the standard library heap with `Reverse`.

## How It Works in Rust

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// O(E log V) time, O(V + E) space
// adj: Vec<Vec<(usize, i64)>> — adjacency list of (neighbour, weight)
fn prim(adj: &[Vec<(usize, i64)>]) -> (i64, Vec<(usize, usize, i64)>) {
    let n = adj.len();
    let mut key    = vec![i64::MAX; n];  // cheapest edge weight to MST
    let mut parent = vec![usize::MAX; n]; // which MST node connects to this
    let mut in_mst = vec![false; n];
    let mut heap   = BinaryHeap::new();

    key[0] = 0;
    heap.push(Reverse((0i64, 0usize)));  // (weight, node) — Reverse for min-heap

    let mut total = 0i64;
    let mut mst   = Vec::new();

    while let Some(Reverse((w, u))) = heap.pop() {
        if in_mst[u] { continue; }     // stale entry: skip
        in_mst[u] = true;
        if parent[u] != usize::MAX {
            total += w;
            mst.push((parent[u], u, w));
        }
        for &(v, wv) in &adj[u] {
            if !in_mst[v] && wv < key[v] {
                key[v]    = wv;
                parent[v] = u;
                heap.push(Reverse((wv, v)));  // may push stale entries
            }
        }
    }
    (total, mst)
}
```

The heap may contain stale entries — old `(key[v], v)` pairs pushed before a better edge was found. The `if in_mst[u] { continue }` guard skips them. This lazy-deletion approach is simpler than a decrease-key heap and works well in practice.

## What This Unlocks

- **Network cable layout**: minimise total cable length connecting office buildings, data centre racks, or circuit components.
- **Cluster analysis**: single-linkage hierarchical clustering produces a dendrogram equivalent to the MST — cut it at a distance threshold to get k clusters.
- **TSP approximation**: construct the MST, perform a DFS preorder traversal, and you have a 2-approximation of the optimal Travelling Salesman tour.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Min-heap | Priority queue module or sorted list | `BinaryHeap<Reverse<(i64, usize)>>` |
| Stale entry handling | Separate `visited` check | `if in_mst[u] { continue }` |
| Adjacency list | `list array` or `Hashtbl` | `Vec<Vec<(usize, i64)>>` |
| Infinity key | `max_int` | `i64::MAX` |
| MST edges | Accumulate in list | `Vec<(usize, usize, i64)>` grown with `push` |
