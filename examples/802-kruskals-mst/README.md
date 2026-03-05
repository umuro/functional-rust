# 802. Kruskal's MST with Union-Find

**Difficulty:** 4  **Level:** Advanced

Build a minimum spanning tree by sorting edges and merging components with a Union-Find data structure — O(E log E).

## The Problem This Solves

Kruskal's algorithm is the MST method of choice for sparse graphs. It's also the canonical application of the Union-Find (Disjoint Set Union) data structure — one of the most practically useful data structures in competitive programming and systems engineering. Union-Find appears in dynamic connectivity queries, network partitioning, image segmentation (connected components), and Kruskal's is just its most famous use case.

The algorithm's appeal is conceptual clarity: sort all edges by weight, then greedily add each edge as long as it doesn't create a cycle. "Doesn't create a cycle" means "connects two different components" — which is exactly what Union-Find checks in near-O(1) time.

## The Intuition

Sort all edges by weight. Walk through them cheapest first. For each edge `(u, v)`, check if u and v are already in the same component (`find(u) == find(v)`). If yes, adding this edge would create a cycle — skip it. If no, merge the components (`union(u, v)`) and add the edge to the MST. Stop when the MST has V-1 edges. The Union-Find with path compression and union by rank makes each operation effectively O(α(V)) — nearly constant. OCaml implements Union-Find with mutable `array` references; Rust uses a `struct UnionFind { parent: Vec<usize>, rank: Vec<usize> }`.

## How It Works in Rust

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank:   Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, mut v: usize) -> usize {
        while self.parent[v] != v {
            self.parent[v] = self.parent[self.parent[v]]; // path halving
            v = self.parent[v];
        }
        v
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.find(u), self.find(v));
        if pu == pv { return false; }    // already same component
        // Union by rank: smaller tree hangs under larger
        match self.rank[pu].cmp(&self.rank[pv]) {
            Less    => self.parent[pu] = pv,
            Greater => self.parent[pv] = pu,
            Equal   => { self.parent[pv] = pu; self.rank[pu] += 1; }
        }
        true
    }
}

// O(E log E) dominated by sort; Union-Find ops are O(α(V)) ≈ O(1)
fn kruskal(n: usize, edges: &mut Vec<(i64, usize, usize)>) -> (i64, Vec<(usize, usize, i64)>) {
    edges.sort_unstable_by_key(|&(w, _, _)| w);
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    let mut total = 0i64;
    for &(w, u, v) in edges.iter() {
        if uf.union(u, v) {   // returns true if they were in different components
            total += w;
            mst.push((u, v, w));
        }
    }
    (total, mst)
}
```

Path halving (`parent[v] = parent[parent[v]]`) is a simpler alternative to full path compression with the same amortised complexity. `sort_unstable_by_key` is faster than `sort_by_key` when stability doesn't matter — and for edge weights it doesn't.

## What This Unlocks

- **Dynamic connectivity**: Union-Find supports online "are these two nodes connected?" queries as edges arrive — powering dynamic graph algorithms and streaming network analysis.
- **Image segmentation**: represent each pixel as a node; Union-Find merges similar adjacent pixels into segments. This is the basis of efficient connected-component labelling.
- **Maze generation**: Kruskal's algorithm directly generates random spanning trees — add edges in random order, skip if they'd create a cycle. This produces unbiased random mazes.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Union-Find | Mutable `int array` for parent | `struct UnionFind { parent: Vec<usize>, rank: Vec<usize> }` |
| Path compression | Recursive `find` with `parent.(v) <- root` | Iterative path halving — no recursion, stack-safe |
| Union by rank | `if rank.(pu) < rank.(pv)` | `match rank[pu].cmp(&rank[pv])` — exhaustive |
| Edge sort | `List.sort` by weight | `sort_unstable_by_key` — faster, in-place |
| Cycle check | `find u = find v` | `uf.union(u, v)` returns `false` if same component |
