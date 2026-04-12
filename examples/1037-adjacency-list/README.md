📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1037-adjacency-list)**

---

# 1037-adjacency-list — Adjacency List Graph
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The adjacency list representation stores a graph as a map from each node to its list of neighbors. Compared to an adjacency matrix (O(V²) space), the adjacency list uses O(V + E) space, making it optimal for sparse graphs — social networks, web graphs, and road maps all have far fewer edges than the V² maximum.

This is the representation behind breadth-first search (BFS), depth-first search (DFS), Dijkstra's algorithm, and most graph algorithms in practice.

## Learning Outcomes

- Represent a directed graph as `HashMap<usize, Vec<usize>>`
- Add nodes and edges with O(1) amortized complexity
- Implement BFS using `VecDeque` for the frontier queue
- Implement DFS using recursion or an explicit stack
- Understand the trade-offs between adjacency list and adjacency matrix

## Rust Application

`src/lib.rs` implements `Graph` with `adj: HashMap<usize, Vec<usize>>`. `add_edge(from, to)` uses `entry(from).or_default().push(to)` and ensures the destination node exists in the map. `bfs` uses a `VecDeque` queue and a `HashSet` of visited nodes. `dfs` recurses into unvisited neighbors.

The adjacency list is the representation used in `petgraph`, network simulation frameworks, and routing algorithms. BFS finds shortest paths (by hop count) in unweighted graphs; DFS is used for cycle detection and topological ordering.

## OCaml Approach

OCaml's adjacency list uses a map or array:

```ocaml
module IntMap = Map.Make(Int)

type graph = int list IntMap.t

let add_edge g from_ to_ =
  let neighbors = try IntMap.find from_ g with Not_found -> [] in
  IntMap.add from_ (to_ :: neighbors) g
```

OCaml's immutable map means each `add_edge` creates a new graph version — appropriate for purely functional graph algorithms but less efficient for incremental construction.

## Key Differences

1. **Mutability**: Rust's `HashMap`-based graph is mutable (push in place); OCaml's `Map`-based graph is immutable (each update returns a new version).
2. **Node initialization**: Rust's `entry().or_default()` ensures the node exists even with no edges; OCaml requires explicit handling of `Not_found`.
3. **BFS queue**: Rust uses `VecDeque` for O(1) front dequeue; OCaml's `Queue` module provides the same, or `List` is used (O(n) dequeue).
4. **Visited set**: Both use a hash set for visited tracking; Rust's `HashSet` and OCaml's `Hashtbl`-based set serve the same purpose.

## Exercises

1. Add a `has_cycle` method that detects cycles using DFS with a color-marking scheme (white/gray/black).
2. Implement an undirected graph variant where `add_edge(a, b)` automatically adds both `a -> b` and `b -> a`.
3. Write `shortest_path(start: usize, end_: usize) -> Option<Vec<usize>>` using BFS with parent tracking to reconstruct the path.
