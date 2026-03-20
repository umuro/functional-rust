📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1038-adjacency-matrix)**

---

# 1038-adjacency-matrix — Adjacency Matrix Graph

## Problem Statement

The adjacency matrix represents a graph as a V×V boolean (or weighted) matrix where `matrix[i][j] = true` means there is an edge from node i to node j. Edge lookup is O(1) — a direct array access — making it ideal for dense graphs where the number of edges approaches V². It is the standard representation in network routing tables, transition matrices in Markov chains, and Floyd-Warshall all-pairs shortest path.

The trade-off is O(V²) space even for sparse graphs, which makes it unsuitable for social networks or road graphs but perfect for complete or nearly-complete graphs.

## Learning Outcomes

- Represent a graph as `Vec<Vec<bool>>` with O(1) edge lookup
- Add directed and undirected edges
- Compute out-degree, in-degree, and find neighbors
- Implement matrix multiplication for reachability (transitive closure)
- Understand the space vs time trade-offs versus adjacency list

## Rust Application

`src/lib.rs` implements `MatrixGraph` with `matrix: Vec<Vec<bool>>` and size `n`. `add_edge(from, to)` sets `matrix[from][to] = true` — O(1). `has_edge(from, to)` is a direct array lookup — O(1). `neighbors(node)` scans row `node` for `true` values — O(V). `out_degree` counts `true` values in a row; `in_degree` scans a column.

The adjacency matrix shines for `has_edge` queries: while an adjacency list requires searching the neighbor list (O(degree)), the matrix answers in O(1).

## OCaml Approach

OCaml uses a 2D array:

```ocaml
type graph = { matrix: bool array array; size: int }

let make n = { matrix = Array.make_matrix n n false; size = n }
let add_edge g i j = g.matrix.(i).(j) <- true
let has_edge g i j = g.matrix.(i).(j)
```

OCaml arrays are mutable by default, making the update syntax cleaner. The semantics are identical to Rust's `Vec<Vec<bool>>`.

## Key Differences

1. **Array access syntax**: OCaml uses `arr.(i).(j)` for 2D array access; Rust uses `matrix[i][j]`.
2. **Initialization**: OCaml's `Array.make_matrix n n false` is a one-liner; Rust uses `vec![vec![false; n]; n]`.
3. **Flat vs nested**: Rust often uses a flat `Vec<bool>` with manual index calculation (`i * n + j`) for better cache locality; OCaml's nested arrays are standard.
4. **Weighted graphs**: Changing `bool` to `Option<f64>` or `f64` with `f64::INFINITY` for no-edge handles weighted adjacency matrices in both languages.

## Exercises

1. Implement Floyd-Warshall all-pairs shortest path on a weighted `Vec<Vec<f64>>` matrix with `f64::INFINITY` for no-edge.
2. Write `transpose(g: &MatrixGraph) -> MatrixGraph` that reverses all edge directions.
3. Compute the transitive closure by matrix squaring: keep multiplying the boolean matrix by itself (using OR instead of AND) until it stabilizes.
