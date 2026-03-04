# 379: DAG and Topological Sort

**Difficulty:** 3  **Level:** Advanced

Directed Acyclic Graph with topological ordering — the foundation of build systems, dependency resolution, and task scheduling.

## The Problem This Solves

Dependencies form a DAG. Package A requires B and C. B requires D. C requires D. You need to build them in an order where every dependency is built before its dependents — that's topological order. You also need to detect cycles: if A depends on B and B depends on A, there's no valid build order.

Makefiles, Gradle, Cargo, Webpack — every build system solves this problem. So does every task scheduler, spreadsheet formula evaluator, and database query planner. The DAG is arguably the most practically important graph structure in software engineering.

Rust's ownership model creates a challenge for DAGs: you can't have two owners point at the same node. The standard solutions are: store nodes in a `Vec` and use indices as edges, use `Rc<RefCell<Node>>` for shared ownership, or use a graph library like `petgraph`.

## The Intuition

A topological sort produces a linear ordering of nodes such that for every directed edge `(u, v)`, node `u` comes before `v`. Kahn's algorithm: find all nodes with zero incoming edges (in-degree 0), process them in any order, decrement in-degrees of their successors, repeat. If all nodes are processed, you have a valid topological order. If nodes remain with non-zero in-degree, there's a cycle.

DFS-based topological sort is the alternative: run DFS, add each fully-explored node to the front of the result. Both are `O(V + E)`.

## How It Works in Rust

```rust
use std::collections::{HashMap, VecDeque};

struct Dag {
    edges: HashMap<usize, Vec<usize>>,  // node → successors
    in_degree: HashMap<usize, usize>,
    nodes: Vec<usize>,
}

impl Dag {
    fn topological_sort(&self) -> Result<Vec<usize>, &'static str> {
        let mut in_deg = self.in_degree.clone();
        let mut queue: VecDeque<usize> = self.nodes.iter()
            .filter(|&&n| *in_deg.get(&n).unwrap_or(&0) == 0)
            .copied().collect();
        let mut order = Vec::new();

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &succ in self.edges.get(&node).unwrap_or(&vec![]) {
                let deg = in_deg.entry(succ).or_insert(0);
                *deg -= 1;
                if *deg == 0 { queue.push_back(succ); }
            }
        }

        if order.len() == self.nodes.len() { Ok(order) }
        else { Err("cycle detected") }
    }
}
```

For production use, `petgraph::algo::toposort` handles this with full cycle detection.

## What This Unlocks

- **Build systems** — compile targets in dependency order; detect circular dependencies early.
- **Task schedulers** — parallelize independent tasks while respecting ordering constraints.
- **Spreadsheet engines** — evaluate formula cells in dependency order to avoid stale values.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph library | `ocamlgraph` | `petgraph` |
| Node sharing | GC handles aliasing | Use indices (`usize`) or `Rc<RefCell<T>>` |
| Topological sort | `ocamlgraph` algorithms | `petgraph::algo::toposort` or manual Kahn's |
| Cycle detection | Algorithm-level | `petgraph::algo::is_cyclic_directed` |
