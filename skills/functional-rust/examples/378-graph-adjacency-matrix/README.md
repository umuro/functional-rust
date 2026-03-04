# 378: Graph as Adjacency Matrix

**Difficulty:** 3  **Level:** Advanced

Represent a graph as a 2D boolean matrix — optimal for dense graphs and constant-time edge queries.

## The Problem This Solves

You have a graph with N nodes and need to answer "is there an edge between node A and node B?" frequently. An adjacency list (`Vec<Vec<usize>>`) answers this in `O(degree(A))` — you scan A's neighbor list. For a dense graph where most nodes connect to most others, that's expensive. An adjacency matrix answers it in `O(1)`: just check `matrix[a][b]`.

The tradeoff is space: `O(N²)` bits regardless of edge count. For a 1000-node graph, that's 1 million bits (~125KB) — fine. For a 100,000-node sparse graph, that's 10 billion bits (~1.25GB) — not fine. The decision rule: dense graphs (E ≈ N²) → matrix. Sparse graphs (E ≈ N) → adjacency list.

Adjacency matrices shine in graph algorithms that need constant-time edge queries: Floyd-Warshall all-pairs shortest paths, transitive closure, dense graph clique detection.

## The Intuition

An adjacency matrix for N nodes is an `N × N` array where `matrix[i][j] = true` means "there is an edge from node i to node j." For undirected graphs, the matrix is symmetric: `matrix[i][j] == matrix[j][i]`. For weighted graphs, store weights instead of booleans: `matrix[i][j] = Some(weight)` or `f64::INFINITY` for no edge.

For large N, use a bit matrix (`Vec<u64>` where each u64 holds 64 bits) to reduce memory by 64× compared to `Vec<Vec<bool>>`.

## How It Works in Rust

```rust
struct AdjMatrix {
    edges: Vec<Vec<bool>>,
    n: usize,
}

impl AdjMatrix {
    fn new(n: usize) -> Self {
        Self { edges: vec![vec![false; n]; n], n }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from][to] = true;
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.edges[from][to]
    }

    fn neighbors(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.n).filter(move |&j| self.edges[node][j])
    }
}

let mut g = AdjMatrix::new(4);
g.add_edge(0, 1);
g.add_edge(0, 2);
g.add_edge(2, 3);

assert!(g.has_edge(0, 1));
assert!(!g.has_edge(1, 3));
```

For production use, the `petgraph` crate provides both matrix and adjacency-list representations with graph algorithm implementations.

## What This Unlocks

- **All-pairs shortest paths** — Floyd-Warshall runs in O(N³) with O(1) edge access, no adjacency scanning.
- **Transitive closure** — multiply boolean matrices to find reachability.
- **Dense graph algorithms** — clique detection, graph coloring are simpler with matrix representation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D array | `Array2D` or `array.(i).(j)` | `Vec<Vec<T>>` or `[T; N*N]` flat with index math |
| Graph library | `ocamlgraph` | `petgraph` |
| Edge query | `O(degree)` in adjacency list | `O(1)` with matrix |
| Memory | Not typed for graphs | Explicit: `N²` bools or `N²` floats |
