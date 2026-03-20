📖 **[View on hightechmind.io →](https://hightechmind.io/rust/365-disjoint-set)**

---

# 365: Disjoint Set (Union-Find)

## Problem Statement

Many problems require tracking which elements belong to the same group and efficiently merging groups: Kruskal's minimum spanning tree algorithm, connected components in a graph, network percolation, image segmentation, and equivalence class tracking in type inference. The Union-Find (Disjoint Set Union) data structure, developed by Bernard Galler and Michael Fischer (1964) with the key optimizations by Tarjan (1975), provides near-O(1) amortized operations. The two optimizations — path compression and union by rank — together achieve O(α(n)) per operation where α is the inverse Ackermann function, effectively constant for all practical inputs.

## Learning Outcomes

- Implement Union-Find with `parent: Vec<usize>` where `parent[x] == x` marks root nodes
- Apply path compression in `find`: flatten the tree by setting `parent[x] = root` directly
- Apply union by rank: always attach the shorter tree under the taller one
- Track component count and component sizes
- Implement `connected(x, y) -> bool` as `find(x) == find(y)`
- Recognize Union-Find in Kruskal's MST, cycle detection, and percolation

## Rust Application

```rust
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // each node is its own root
            rank: vec![0; n],
            size: vec![1; n],
            components: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry { return false; } // already in same set
        // union by rank: attach smaller tree to larger
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
            self.rank[rx] += 1;
        }
        self.components -= 1;
        true
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
```

Path compression is the crucial optimization: `parent[x] = find(parent[x])` replaces the parent with the ultimate root, flattening future lookups. Without it, trees degenerate to O(n) chains. With both optimizations, even 10^9 operations on 10^9 elements finish in seconds.

## OCaml Approach

```ocaml
type uf = {
  parent: int array;
  rank: int array;
  mutable components: int;
}

let make n =
  { parent = Array.init n Fun.id; rank = Array.make n 0; components = n }

let rec find uf x =
  if uf.parent.(x) = x then x
  else begin
    let root = find uf uf.parent.(x) in
    uf.parent.(x) <- root;  (* path compression *)
    root
  end

let union uf x y =
  let rx = find uf x and ry = find uf y in
  if rx = ry then false
  else begin
    if uf.rank.(rx) < uf.rank.(ry) then uf.parent.(rx) <- ry
    else if uf.rank.(rx) > uf.rank.(ry) then uf.parent.(ry) <- rx
    else (uf.parent.(ry) <- rx; uf.rank.(rx) <- uf.rank.(rx) + 1);
    uf.components <- uf.components - 1;
    true
  end
```

The algorithm is identical in both languages — Union-Find is inherently imperative (mutating parent pointers is the key insight). Both use array-based storage for O(1) index access.

## Key Differences

| Aspect | Rust `UnionFind` | OCaml `uf` |
|--------|-----------------|------------|
| Storage | `Vec<usize>` | `int array` |
| Path compression | Recursive with assignment | Recursive with assignment |
| Borrowing | `&mut self` for path compression | Mutable array cells |
| Size tracking | `Vec<usize>` per root | Not shown (add `size` array) |
| Complexity | O(α(n)) amortized | O(α(n)) amortized |

## Exercises

1. **Kruskal's MST**: Sort edges by weight, then apply Union-Find to select edges that connect different components; stop when you have `n-1` edges for `n` nodes — this is Kruskal's O(E log E) MST algorithm.
2. **Connected components**: Given an undirected graph as edge list, use Union-Find to count connected components; report the size of each component using the `size` array.
3. **Percolation**: Create an N×N grid where each cell is open with probability p; use Union-Find to determine if the top row connects to the bottom row; find the percolation threshold via Monte Carlo simulation.
