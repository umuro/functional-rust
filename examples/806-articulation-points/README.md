📖 **[View on hightechmind.io →](https://hightechmind.io/rust/806-articulation-points)**

---

# 806: Articulation Points and Bridges

**Difficulty:** 5  **Level:** Master

Find every vertex and edge whose removal disconnects a graph — in a single O(V+E) DFS pass.

## The Problem This Solves

An articulation point (cut vertex) is a vertex whose removal splits a connected graph into two or more disconnected components. A bridge is an edge whose removal does the same. Together they identify the fragile spots in any network: routers that, if they fail, partition the network; roads that, if closed, isolate a region; proteins that, if mutated, break a pathway.

Use this algorithm when you need to find single points of failure in undirected graphs. Network reliability analysis, road infrastructure planning, and circuit board design all depend on locating articulation points and bridges efficiently. The naive approach (remove each vertex/edge and re-run BFS/DFS) costs O(V·(V+E)); Tarjan's DFS-based method finds them all in one pass.

The algorithm produces two outputs: the set of articulation point indices and the set of bridge edges `(u, v)`. You can then use these to identify 2-connected components (biconnected components) — subgraphs with no articulation points.

## The Intuition

During DFS, every non-tree edge goes to an ancestor (back edge). A vertex `v` is an articulation point if any of its DFS children `c` cannot reach an ancestor of `v` through back edges — meaning removing `v` would cut `c` off.

Track two values per vertex:
- `disc[v]`: DFS discovery time (when we first visit `v`)
- `low[v]`: the minimum discovery time reachable from the subtree rooted at `v` via back edges

Then: `v` is an articulation point if it has a child `c` where `low[c] >= disc[v]`. An edge `(v, c)` is a bridge if `low[c] > disc[v]` (strict — the child can't reach even `v` itself).

Special case: the DFS root is an articulation point iff it has more than one DFS child.

O(V+E) time. In OCaml you'd use mutable refs or a State monad. In Rust, iterative DFS with a `returning` phase cleanly handles the post-order update of `low` values.

## How It Works in Rust

```rust
fn find_aps_bridges(adj: &[Vec<usize>]) -> (Vec<usize>, Vec<(usize, usize)>) {
    let n = adj.len();
    let mut disc = vec![u32::MAX; n]; // u32::MAX = unvisited
    let mut low  = vec![0u32; n];
    let mut parent = vec![usize::MAX; n];
    let mut timer  = 0u32;
    let mut aps    = vec![];
    let mut bridges = vec![];

    for root in 0..n {
        if disc[root] != u32::MAX { continue; }
        // Stack entries: (vertex, edge_index, is_returning)
        let mut stack: Vec<(usize, usize, bool)> = vec![(root, 0, false)];
        let mut child_count = vec![0usize; n]; // DFS children of root

        while let Some((v, ei, returning)) = stack.last_mut() {
            let v = *v;
            if !returning && disc[v] == u32::MAX {
                // Pre-visit
                disc[v] = timer; low[v] = timer; timer += 1;
            }
            let adj_v = &adj[v];
            if *ei < adj_v.len() {
                let w = adj_v[*ei]; *ei += 1;
                if disc[w] == u32::MAX {
                    // Tree edge
                    parent[w] = v;
                    if v == root { child_count[root] += 1; }
                    stack.push((w, 0, false));
                } else if w != parent[v] {
                    // Back edge — update low
                    low[v] = low[v].min(disc[w]);
                }
            } else {
                // Post-visit: propagate low upward
                stack.pop();
                if let Some(&(p, _, _)) = stack.last() {
                    low[p] = low[p].min(low[v]);
                    // Check articulation point (non-root)
                    if p != root && low[v] >= disc[p] { aps.push(p); }
                    // Check bridge
                    if low[v] > disc[p] { bridges.push((p, v)); }
                }
            }
        }
        // Root is AP iff it has ≥2 DFS children
        if child_count[root] >= 2 { aps.push(root); }
    }
    aps.sort_unstable(); aps.dedup();
    (aps, bridges)
}
```

The `ei` (edge index) in the stack frame replaces the loop state that would normally be implicit in a recursive call — iterating neighbors without restarting from zero each time.

## What This Unlocks

- **Network resilience analysis**: identify single points of failure in communication or power grids.
- **Biconnected component decomposition**: split a graph into 2-edge-connected components for robust path planning.
- **Compiler control-flow analysis**: find critical nodes in control-flow graphs where program flow must pass through.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable DFS state | `ref` cells or `Array.make` | `vec![]` arrays indexed by vertex |
| Iterative DFS loop state | Rare — usually recursive | `(vertex, edge_index, returning)` tuple in stack |
| Low-link update | Post-order in recursive call | Propagated when stack entry is popped |
| Deduplication | `List.sort_uniq` | `sort_unstable` + `dedup` |
| Root child count | Extra counter threaded through recursion | Separate `child_count` array |
