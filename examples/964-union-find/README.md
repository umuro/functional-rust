**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[union-find on hightechmind.io](https://hightechmind.io/posts/functional-rust/union-find)

---

## Problem Statement

Implement Union-Find (Disjoint Set Union) with path compression and union by rank. Path compression flattens the parent chain so that future `find` operations are O(1) amortized. Union by rank ensures the shorter tree is attached under the taller one, bounding tree height to O(log n). Track the number of connected components.

## Learning Outcomes

- Implement `find` with two-pass path compression: first locate root, then point all nodes directly to root
- Implement `union` by rank: attach lower-rank root under higher-rank root; increment rank only on ties
- Track `components` counter, decrementing on successful union (different components merged)
- Understand why the inverse-Ackermann function α(n) bounds the amortized per-operation cost
- Implement iterative `find` (not recursive) to avoid stack overflow on degenerate inputs

## Rust Application

```rust
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),  // each node is its own parent
            rank: vec![0; n],
            components: n,
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        // Pass 1: find root
        let mut root = i;
        while self.parent[root] != root { root = self.parent[root]; }
        // Pass 2: path compression — point all to root
        while self.parent[i] != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb { return false; }  // already connected
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less    => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal   => { self.parent[rb] = ra; self.rank[ra] += 1; }
        }
        self.components -= 1;
        true
    }
}
```

`parent[i] = i` means node `i` is a root. The two-pass compression in `find` makes every node on the path from `i` to root point directly to root after the call. Subsequent `find(i)` calls reach root in O(1).

`union` returns `bool` to indicate whether a merge occurred (false = already in the same component). This is useful for detecting cycle formation in graph algorithms.

## OCaml Approach

```ocaml
type t = {
  parent: int array;
  rank: int array;
  mutable components: int;
}

let create n = {
  parent = Array.init n (fun i -> i);
  rank = Array.make n 0;
  components = n;
}

let rec find uf i =
  if uf.parent.(i) = i then i
  else begin
    let root = find uf uf.parent.(i) in
    uf.parent.(i) <- root;  (* path compression *)
    root
  end

let union uf a b =
  let ra = find uf a and rb = find uf b in
  if ra = rb then false
  else begin
    (if uf.rank.(ra) < uf.rank.(rb)
     then uf.parent.(ra) <- rb
     else if uf.rank.(ra) > uf.rank.(rb)
     then uf.parent.(rb) <- ra
     else begin uf.parent.(rb) <- ra; uf.rank.(ra) <- uf.rank.(ra) + 1 end);
    uf.components <- uf.components - 1;
    true
  end
```

OCaml's recursive `find` with in-place path compression is natural but risks stack overflow on very deep chains before the first compression. Rust's iterative two-pass version avoids this.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Path compression | Iterative two-pass | Recursive single-pass |
| Array access | `self.parent[i]` | `uf.parent.(i)` |
| Stack safety | Iterative — no overflow risk | Recursive — risk before first compression |
| Mutable field | `components` in struct | `mutable components` in record |

Union-Find is the foundation of Kruskal's minimum spanning tree algorithm, network connectivity, and cycle detection. The nearly-O(1) amortized cost makes it practical for millions of operations.

## Exercises

1. Implement `connected(a, b) -> bool` as a convenience wrapper over `find(a) == find(b)`.
2. Use Union-Find to implement Kruskal's MST algorithm: sort edges by weight, union each if not already connected.
3. Implement `component_sizes() -> HashMap<usize, usize>` mapping root → component size.
4. Extend to support an arbitrary label per component: `set_label(root, label)`, `get_label(node) -> label`.
5. Implement a version where `find` is `&self` (not `&mut self`) using `UnsafeCell` for interior mutability — analyze the tradeoffs.
