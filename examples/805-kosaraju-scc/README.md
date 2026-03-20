📖 **[View on hightechmind.io →](https://hightechmind.io/rust/805-kosaraju-scc)**

---

# 805-kosaraju-scc — Kosaraju's Strongly Connected Components
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Kosaraju's algorithm (1978, independently by Sharir 1981) finds SCCs using two DFS passes: first on the original graph to compute finish-time order, then on the reversed graph in reverse finish order. Each DFS tree in the second pass is exactly one SCC. While Tarjan's uses one pass, Kosaraju's is conceptually simpler and easier to implement correctly. Both run in O(V+E) time.

## Learning Outcomes

- Understand the two-pass approach: finish order DFS + reversed graph DFS
- Build the reversed graph `radj` by swapping edge directions
- Use a `finish_order: Vec<usize>` stack populated during the first DFS
- Each `dfs2` call on an unvisited vertex in reverse finish order yields exactly one SCC
- Compare with Tarjan's: same asymptotic complexity, different implementation approach

## Rust Application

`kosaraju(n, edges)` builds `adj` and `radj` simultaneously. First DFS (`dfs1`) populates `order` with vertices in finish order. After reversing `visited`, the second DFS (`dfs2`) processes vertices from `order.rev()`, collecting reachable vertices in the reversed graph. Each such collection is an SCC. Nested `fn dfs1` and `fn dfs2` are defined inside the main function.

## OCaml Approach

OCaml's two-function pattern mirrors the two-pass algorithm naturally. `let rec dfs1 v = ... order := v :: !order` and `let rec dfs2 v = comp := v :: !comp`. The reversed graph uses `Hashtbl` for edge storage. OCaml's `List.iter (fun v -> if not vis.(v) then ...) (List.rev !order)` drives the second pass. The `Ocamlgraph` library offers Kosaraju's as an alternative to Tarjan's.

## Key Differences

1. **Code simplicity**: Kosaraju's two-pass DFS is more straightforward to reason about than Tarjan's single-pass with low-link values.
2. **Memory**: Kosaraju's requires storing the reversed graph; Tarjan's uses only the original graph with additional O(V) arrays.
3. **Cache behavior**: Kosaraju's second DFS on the reversed graph has worse cache behavior than Tarjan's single pass; in practice the difference is small.
4. **Correctness proof**: Kosaraju's correctness is easier to prove: finish order captures the "reachability hierarchy" of SCCs.

## Exercises

1. Implement an iterative version of both DFS passes to avoid stack overflow on graphs with millions of vertices.
2. Time Kosaraju's vs Tarjan's on a graph with 100,000 nodes and 500,000 edges — do they produce the same SCCs (verify by comparing sorted SCC sets)?
3. Use Kosaraju's to identify "sink SCCs" in a dependency graph: SCCs with no outgoing edges to other SCCs are safe to process first in a topological order.
