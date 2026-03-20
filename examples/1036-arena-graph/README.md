📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1036-arena-graph)**

---

# 1036-arena-graph — Graph with Arena Allocation
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Graphs are notoriously difficult to represent in Rust's ownership model because nodes can have multiple incoming edges, violating the single-owner rule. The arena allocation pattern sidesteps this by storing all nodes in a `Vec<Node>` (the "arena") and using integer indices as edge references instead of direct pointers. Index-based references have no ownership semantics and cannot dangle as long as the arena lives.

This pattern is the recommended idiomatic approach for graphs in Rust, used in the `petgraph` crate, LLVM's IR (which uses integer node IDs), and compiler intermediate representations.

## Learning Outcomes

- Represent a graph as `Vec<Node>` with integer index edges
- Add nodes and directed edges using indices
- Implement BFS and DFS over an arena-based graph
- Understand why `Vec<Box<Node>>` with pointer edges does not work in safe Rust
- Connect arena allocation to `petgraph`'s `NodeIndex` type

## Rust Application

`src/lib.rs` implements `Graph` with `nodes: Vec<Node>` where each `Node` has a `label: String` and `edges: Vec<usize>`. `add_node` returns the new node's index; `add_edge(from, to)` pushes `to` into `nodes[from].edges`. BFS uses a `VecDeque` queue and a `HashSet` of visited indices. DFS uses the call stack via recursion.

The arena pattern eliminates all unsafe code and borrow checker fights. The entire graph is owned by one `Graph` struct; "pointers" are indices that are valid as long as the `Graph` is alive.

## OCaml Approach

OCaml graphs can use mutable node records with `ref` pointers (GC handles cycles) or the same index-based approach:

```ocaml
type node = { label: string; mutable edges: int list }
type graph = { nodes: node array }

let add_edge g from_ to_ =
  g.nodes.(from_).edges <- to_ :: g.nodes.(from_).edges
```

The mutable pointer approach is natural in OCaml because the GC handles cycles. The index approach is also used for performance-critical code.

## Key Differences

1. **Pointer vs index**: Rust requires index-based edges to avoid ownership problems; OCaml can use direct mutable pointers (GC prevents dangling).
2. **Cycle safety**: Rust's arena cannot have reference cycles (indices are just integers); OCaml's GC handles pointer cycles automatically.
3. **Memory locality**: Both approaches benefit from arena allocation (all nodes in one `Vec`/`array`), but Rust's index approach is idiomatic.
4. **petgraph**: Rust's `petgraph` crate abstracts over `NodeIndex` types and is the production graph library; OCaml's `ocamlgraph` uses a similar design.

## Exercises

1. Add a `remove_edge(from: usize, to: usize)` method that removes a directed edge.
2. Implement a topological sort over the arena graph using DFS post-order.
3. Add edge weights `(to: usize, weight: i64)` and implement Dijkstra's shortest path over the weighted arena graph.
