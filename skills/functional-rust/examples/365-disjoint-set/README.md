# 365: Union-Find — Disjoint Set Forest

**Difficulty:** 3  **Level:** Advanced

Track which elements belong to the same group and merge groups in O(α(n)) amortized — effectively O(1) in practice.

## The Problem This Solves

You're given a list of edges in a network and need to answer: "are nodes A and B connected?" After processing all edges, you also need to know how many connected components exist. A BFS/DFS from scratch answers each query in O(V+E) and doesn't update incrementally.

Union-Find is built exactly for this: process edges one by one, merging sets as you go, and answer "same group?" queries in near-constant time at any point. It's the workhorse behind Kruskal's minimum spanning tree algorithm — add the cheapest edge that doesn't connect two already-connected nodes.

The second class of problems is cycle detection in undirected graphs: if you try to union two nodes that already share a root, they're in the same component — you've found a cycle. This is exactly the check Kruskal's needs, and it's O(α(n)) per edge.

## The Intuition

Imagine you have a collection of groups, each with a representative ("root"). To check if two people are in the same group, ask them both: "who is your group's root?" If the roots match, same group. To merge two groups, just point one root to the other.

The clever part is **path compression**: after finding a root, update every node along the path to point directly at the root. Next time you ask the same node, the answer is immediate. **Union by rank** ensures the tree stays flat by always attaching the smaller tree under the larger. Together, these make the amortized cost per operation O(α(n)) — the inverse Ackermann function, which is ≤ 4 for any n you'll encounter in practice.

## How It Works in Rust

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    count: usize, // number of distinct components
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(), // each node is its own root
            rank: vec![0; n],
            count: n,
        }
    }

    // Find root with path compression — O(α(n)) amortized
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    // Union two sets — returns false if already in same set (cycle detected)
    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return false; } // already connected

        // Union by rank: attach smaller tree under larger
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less    => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal   => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        self.count -= 1;
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn components(&self) -> usize { self.count }
}

// Usage: Kruskal's MST — process edges sorted by weight
let mut uf = UnionFind::new(5); // nodes 0..4

let edges = vec![(1, 0, 1), (3, 1, 2), (2, 0, 3), (5, 3, 4)]; // (weight, u, v)
let mut sorted = edges.clone();
sorted.sort();

let mut mst_weight = 0;
for (w, u, v) in sorted {
    if uf.union(u, v) { // only add edge if it connects two components
        mst_weight += w;
    }
}
println!("MST weight: {mst_weight}");
println!("Components: {}", uf.components()); // 1 if fully connected
```

## What This Unlocks

- **Minimum spanning trees (Kruskal's)**: sort edges by weight, add each edge that connects two different components — O(E log E + E·α(n)).
- **Network connectivity queries**: online algorithm — handle "connect A and B" and "are A and B connected?" queries in near-constant time each.
- **Cycle detection**: in undirected graphs, `union()` returning `false` means you tried to connect two already-connected nodes — a cycle.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Union-Find | not in stdlib | custom `Vec`-based |
| Find with path compression | N/A | O(α(n)) amortized |
| Union by rank | N/A | prevents degenerate O(n) trees |
| Cycle detection | manual DFS | `union()` returns `false` |
| Connected query | BFS/DFS each time | O(α(n)) via `find` |
| Component count | manual tracking | maintained in `count` field |
