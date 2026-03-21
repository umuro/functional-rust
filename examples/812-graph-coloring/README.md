📖 **[View on hightechmind.io →](https://hightechmind.io/rust/812-graph-coloring)**

---

# 812-graph-coloring — Graph Coloring
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Graph coloring assigns colors to vertices so no two adjacent vertices share a color, using at most k colors. The four-color theorem (every planar map is 4-colorable) is famous; the general k-coloring decision problem is NP-complete for k ≥ 3. Applications: register allocation in compilers (variables with conflicting lifetimes need different registers), exam scheduling (conflicting exams need different time slots), frequency assignment in cellular networks.

## Learning Outcomes

- Implement backtracking k-coloring: assign colors 1..=k to vertices, backtrack when stuck
- Apply the safety check: `is_safe(v, c)` — no adjacent vertex has color c
- Understand why 2-coloring = bipartite check (polynomial) but 3-coloring is NP-complete
- Know the greedy coloring algorithm (not optimal but fast) and its chromatic number approximation
- See the connection to register allocation in LLVM's coloring algorithm

## Rust Application

`graph_coloring(n, edges, m)` builds an adjacency list. `is_safe(v, c)` checks that no neighbor has color `c`. `solve(v)` tries colors 1..=m for vertex v: if safe, assign, recurse on v+1; if successful, return true; else backtrack. Returns `Some(Vec<usize>)` with assignments or `None` if k colors are insufficient. Tests verify 3-coloring of a 4-node graph.

## OCaml Approach

OCaml implements with `colors: int array` initialized to 0 (uncolored). The `is_safe` function uses `List.for_all (fun u -> colors.(u) <> c) adj.(v)`. The recursive backtracking is a natural OCaml pattern. The `exception Found` can terminate early. OCaml's `Graph_coloring` in academic libraries implements both exact and approximation algorithms.

## Key Differences

1. **Backtracking structure**: Identical in both languages — assign, recurse, undo. The algorithm is language-independent.
2. **NP-completeness**: 3-coloring is NP-complete; Rust and OCaml face the same exponential worst case; only small graphs are solvable exactly.
3. **Register allocation**: LLVM and GCC use graph coloring for register allocation (variables as vertices, conflicting lifetimes as edges); Rust's `rustc` uses a similar approach in its codegen.
4. **Chromatic number**: The minimum k for which a graph is k-colorable; computing it exactly is NP-hard; approximations use greedy algorithms.

## Exercises

1. Implement greedy coloring: assign the smallest available color to each vertex in a fixed order. Prove it uses at most `Δ+1` colors where Δ is the maximum degree.
2. Implement Welsh-Powell ordering: sort vertices by degree (descending) before greedy coloring. Verify it often uses fewer colors than arbitrary ordering.
3. Apply graph coloring to exam scheduling: given a set of exams and students enrolled in each, build a conflict graph and find the minimum number of time slots needed.
