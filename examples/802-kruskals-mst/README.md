📖 **[View on hightechmind.io →](https://hightechmind.io/rust/802-kruskals-mst)**

---

# 802-kruskals-mst — Kruskal's Minimum Spanning Tree

## Problem Statement

Kruskal's algorithm (1956) finds the MST by sorting all edges by weight and adding each edge if it doesn't create a cycle, using a Union-Find (Disjoint Set Union) data structure to detect cycles in near-O(1). While Prim's grows from a vertex, Kruskal's grows from edges. It is more efficient for sparse graphs (E ≈ V) and is the basis for Borůvka's parallel MST algorithm used in distributed computing.

## Learning Outcomes

- Implement Union-Find with path compression and union by rank
- Sort edges by weight and process them greedily
- Skip edges whose endpoints are in the same component (would create a cycle)
- Understand O(E log E) time complexity (dominated by sorting)
- See how Union-Find achieves near-O(1) amortized per operation with path compression

## Rust Application

`UnionFind` has `parent: Vec<usize>` and `rank: Vec<usize>`. `find` implements path compression (directly updating `parent[x]` to the root). `union` merges by rank to keep trees balanced. `kruskals_mst(n, edges)` sorts edges by weight, then iterates: for each edge, if `union(u, v)` succeeds (they were in different components), add the weight. Returns total MST weight.

## OCaml Approach

OCaml's Union-Find uses `Array.make n (Array.init n Fun.id)` for the parent array and imperative updates. The path compression is expressed as a recursive function with a `ref` for the root. OCaml's `Array.sort` sorts edges in-place. The `Ocamlgraph.Kruskal` module provides a clean functional implementation using persistent data structures.

## Key Differences

1. **Union-Find ergonomics**: Rust's `struct UnionFind` with methods is idiomatic; OCaml uses a module with mutable arrays, similar in structure.
2. **Path compression**: Both languages implement the two-pass path compression the same way; Rust's recursive implementation matches OCaml's.
3. **Sorting**: Rust's `Vec::sort_by_key` and OCaml's `Array.sort` are both O(n log n); the edge comparison is identical.
4. **Parallel MST**: Borůvka's algorithm (more parallelizable than Kruskal's) is used in distributed graph processing (Apache Spark GraphX); Kruskal's is the sequential baseline.

## Exercises

1. Modify to return the actual MST edges `Vec<(usize, usize, i32)>`, not just the total weight.
2. Implement `max_spanning_tree` by reversing the edge sort order — identical to Kruskal's but taking maximum-weight edges.
3. Implement an online Kruskal's that accepts edges one at a time and reports the current spanning forest weight after each insertion.
