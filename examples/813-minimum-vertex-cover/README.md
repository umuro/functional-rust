📖 **[View on hightechmind.io →](https://hightechmind.io/rust/813-minimum-vertex-cover)**

---

# 813-minimum-vertex-cover — Minimum Vertex Cover (Trees)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A vertex cover is a set of vertices such that every edge has at least one endpoint in the set. The minimum vertex cover finds the smallest such set. For general graphs this is NP-hard (equivalent to maximum independent set), but for trees it is solvable in O(n) by DP on the tree structure. Applications: network security (placing sensors to monitor all links), database index optimization, and approximation algorithms for general graphs.

## Learning Outcomes

- Implement tree DP with two states per vertex: `dp[v][0]` (v not in cover) and `dp[v][1]` (v in cover)
- Apply the recurrence: if v not in cover, all children must be covered
- If v in cover, children may or may not be covered (take minimum)
- Run DFS from the root, computing dp values bottom-up
- Understand König's theorem: in bipartite graphs, max matching = min vertex cover

## Rust Application

`min_vertex_cover_tree(n, edges)` builds an adjacency list. DFS computes `dp[v][0]` (v excluded: all children must be included) and `dp[v][1]` (v included: each child takes min of included/excluded). Returns `min(dp[0][0], dp[0][1])`. Tests: a star graph with center node (cover = {center}, size 1) and a path.

## OCaml Approach

OCaml implements tree DP with `Array.make n [|0; 0|]` and recursive DFS. `let rec dfs v parent = ... List.iter (fun u -> if u <> parent then (dfs u v; ...)) adj.(v)`. The DP update is: `dp.(v).(0) <- dp.(v).(0) + dp.(u).(1)` and `dp.(v).(1) <- dp.(v).(1) + min dp.(u).(0) dp.(u).(1)`. OCaml's pattern matching makes the two cases readable.

## Key Differences

1. **Tree DP structure**: Both languages implement the same O(n) tree DP; the recursion pattern is nearly identical.
2. **Parent tracking**: Both need to avoid revisiting the parent edge; Rust uses `vis` array, OCaml passes `parent` explicitly.
3. **General graphs**: For general graphs, minimum vertex cover is NP-hard; the 2-approximation (take both endpoints of each maximal matching edge) works in both languages.
4. **König's theorem**: In bipartite graphs, the minimum vertex cover equals maximum matching size — a deep connection enabling polynomial solutions for bipartite instances.

## Exercises

1. Implement `min_vertex_cover_reconstruct(n, edges) -> Vec<usize>` that returns the actual vertices in the cover, not just the count.
2. Implement the 2-approximation for general graphs: find a maximal matching and include both endpoints of each matched edge. Verify the result is a valid cover.
3. Implement maximum independent set for trees: since `MIS = n - MVC`, use the tree DP to compute both. Verify `MVC + MIS = n`.
