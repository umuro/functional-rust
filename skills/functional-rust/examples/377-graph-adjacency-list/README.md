# 377: Graph as Adjacency List

**Difficulty:** 3  **Level:** Advanced

Represent a graph as a map from node to neighbors — O(V+E) space, O(degree) neighbor access, the natural structure for BFS, DFS, and shortest paths.

## The Problem This Solves

You need to model a network: cities connected by roads, users connected by follows, tasks connected by dependencies. The two classic representations are the adjacency matrix (a V×V grid) and the adjacency list (a list of neighbors per node).

An adjacency matrix is O(V²) space and gives O(1) edge existence checks — but V² is 10^10 for a million-node graph. Real-world graphs are sparse: each node typically has a handful of neighbors, not V. An adjacency list uses O(V+E) space and scales to hundreds of millions of nodes.

In Rust, the most flexible adjacency list is `HashMap<NodeId, Vec<NodeId>>`. For dense integer-labeled graphs (nodes 0..n), a plain `Vec<Vec<usize>>` is faster and simpler — no hash overhead, better cache locality. Both representations support the same algorithms; the choice is about whether node IDs are integers or arbitrary types.

## The Intuition

Python's closest equivalent is `defaultdict(list)`: `graph[u].append(v)` to add an edge, `for v in graph[u]` to visit neighbors. Rust's `HashMap<K, Vec<K>>` with the entry API gives you exactly this.

The tradeoff vs adjacency matrix: O(degree) to check if edge (u, v) exists (scan neighbor list) vs O(1) with matrix. But you rarely need "does edge (u, v) exist?" without context — usually you're iterating all neighbors of u, which is O(degree) either way. For dense graphs (many edges relative to V²), consider a matrix or a bitset adjacency representation.

## How It Works in Rust

```rust
use std::collections::{HashMap, VecDeque, HashSet};

struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self { Graph { edges: HashMap::new() } }

    // Add directed edge u → v
    fn add_edge(&mut self, u: usize, v: usize) {
        self.edges.entry(u).or_default().push(v);
        self.edges.entry(v).or_default(); // ensure v exists even with no out-edges
    }

    // Add undirected edge (both directions)
    fn add_undirected(&mut self, u: usize, v: usize) {
        self.edges.entry(u).or_default().push(v);
        self.edges.entry(v).or_default().push(u);
    }

    fn neighbors(&self, u: usize) -> &[usize] {
        self.edges.get(&u).map(|v| v.as_slice()).unwrap_or(&[])
    }

    // BFS: shortest path (by hop count) from source
    fn bfs(&self, src: usize) -> HashMap<usize, usize> {
        let mut dist = HashMap::new();
        let mut queue = VecDeque::new();
        dist.insert(src, 0);
        queue.push_back(src);

        while let Some(u) = queue.pop_front() {
            for &v in self.neighbors(u) {
                if !dist.contains_key(&v) {
                    dist.insert(v, dist[&u] + 1);
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    // DFS: iterative, returns visited nodes in DFS order
    fn dfs(&self, src: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = vec![src];
        let mut order = Vec::new();

        while let Some(u) = stack.pop() {
            if visited.insert(u) {
                order.push(u);
                // Push neighbors in reverse order to visit in natural order
                for &v in self.neighbors(u).iter().rev() {
                    if !visited.contains(&v) {
                        stack.push(v);
                    }
                }
            }
        }
        order
    }
}

// Usage
let mut g = Graph::new();
g.add_undirected(0, 1);
g.add_undirected(0, 2);
g.add_undirected(1, 3);
g.add_undirected(2, 3);
g.add_undirected(3, 4);

let distances = g.bfs(0);
println!("Distance 0→4: {}", distances[&4]); // 3

let path = g.dfs(0);
println!("DFS order: {path:?}"); // [0, 1, 3, 4, 2] (or similar)
```

## What This Unlocks

- **BFS for shortest paths**: unweighted shortest path between any two nodes; also used for finding connected components and level-order traversal.
- **DFS for topological sort and cycle detection**: process directed acyclic graphs (DAGs) in dependency order; detect cycles in dependency graphs.
- **Foundation for weighted graph algorithms**: add edge weights with `Vec<(usize, i32)>` (neighbor, weight) and you have Dijkstra's and Bellman-Ford input format.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph representation | adjacency list as `('a * 'a list) list` | `HashMap<NodeId, Vec<NodeId>>` |
| Add edge | manual list construction | `.entry(u).or_default().push(v)` |
| BFS | manual queue + visited set | `VecDeque` + `HashSet` |
| DFS | recursive (stack risk on deep graphs) | iterative with explicit `Vec` stack |
| Weighted edges | `(node * weight) list` | `Vec<(usize, i32)>` per node |
| Dense integer graphs | same | `Vec<Vec<usize>>` — faster, no hash |
