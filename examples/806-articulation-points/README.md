📖 **[View on hightechmind.io →](https://hightechmind.io/rust/806-articulation-points)**

---

# 806-articulation-points — Articulation Points

## Problem Statement

An articulation point (cut vertex) is a vertex whose removal disconnects the graph. Finding articulation points in O(V+E) is critical for network reliability analysis: which routers, if removed, would split the internet? Which protein in a protein-protein interaction network is essential? Which road intersection, if closed, disconnects a city? The algorithm also finds bridges (edges whose removal disconnects the graph).

## Learning Outcomes

- Track discovery time and low-link values per vertex during DFS
- Identify articulation points using the root condition (children > 1) and non-root condition (low[child] >= disc[parent])
- Understand the `low[v]` value: minimum discovery time reachable from v's subtree via back edges
- Extend to bridge detection: an edge (u,v) is a bridge if `low[v] > disc[u]`
- Apply to network resilience analysis

## Rust Application

`articulation_points(n, edges)` builds an undirected adjacency list. DFS tracks `disc[v]`, `low[v]`, `parent[v]`, and `ap[v]`. For each child `w`: if unvisited, recurse then update `low[v] = min(low[v], low[w])`; check AP conditions. If `w` already visited and `w != parent[v]`, update `low[v] = min(low[v], disc[w])`. Returns `Vec<usize>` of articulation point indices.

## OCaml Approach

OCaml implements with `ref` cells for `time` and mutable arrays for `disc`, `low`, `parent`, `ap`. Recursive DFS uses `let rec dfs u = ... List.iter (fun v -> ...) adj.(u)`. The `Ocamlgraph.Components.articulation_points` function provides a ready-made implementation. Bridge finding changes the condition from `>=` to `>`.

## Key Differences

1. **Two conditions**: The root condition (children > 1) and non-root condition (low[child] >= disc[u]) handle the two cases identically in both languages.
2. **Parent tracking**: Both use a `parent` array to avoid treating the tree edge back to parent as a back edge; the `-1` / `None` sentinel serves the same purpose.
3. **Undirected graphs**: Both add edges in both directions for undirected graphs; the parent check prevents incorrectly counting the parent as a back edge.
4. **Network analysis**: ISP engineers use articulation point algorithms to identify single points of failure; tools like `traceroute` data combined with this algorithm maps internet topology.

## Exercises

1. Implement `find_bridges(n, edges) -> Vec<(usize, usize)>` using the same DFS with the modified condition `low[v] > disc[u]`.
2. Compute the 2-vertex-connected components: groups of vertices that remain connected after removing any single vertex.
3. Apply to a social network: find "bridge users" whose profiles, if deleted, would disconnect communities. Visualize the result.
