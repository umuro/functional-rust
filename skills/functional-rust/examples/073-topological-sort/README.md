# 073: Topological Sort

**Difficulty:** 3  **Level:** Advanced

Order nodes in a directed acyclic graph (DAG) so every dependency comes before the thing that depends on it — the algorithm behind build systems, package managers, and task schedulers.

## The Problem This Solves

You have tasks where some must complete before others can start: compile `utils` before `main`, install `tokio` before `axum`, run database migrations before starting the server. The dependency graph might have complex chains and fan-outs, but it must have no cycles (a cycle means task A requires B which requires A — impossible to satisfy).

Topological sort produces a linear ordering where every edge points forward: if there's an edge from A to B (A depends on B), then B appears before A in the result. Without it, you'd need to manually calculate the right order — tedious and error-prone for large graphs.

Two classic algorithms: DFS-based (post-order reversal) and Kahn's algorithm (BFS with in-degree counting). Rust's `HashMap` and `HashSet` make both natural to implement, though the explicit ownership of mutable state makes the imperative style more visible than OCaml's purely functional approach.

## The Intuition

In OCaml's purely functional style, visited state and the accumulation list are threaded through recursive calls — nothing is mutated in place. In Rust, you use `HashSet` for `visited` and `Vec` for `order` with mutable references — explicit about who owns and who mutates. The algorithm is the same; the ownership discipline is different.

DFS post-order: visit all descendants of a node before adding it to the result. Reverse the result. Kahn's: find all nodes with no incoming edges, remove them (decrement neighbors' in-degrees), repeat.

## How It Works in Rust

```rust
// DFS-based: post-order traversal, then reverse
pub fn topo_sort(edges: &[(&str, &str)]) -> Vec<String> {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut all_nodes = HashSet::new();
    for &(from, to) in edges {
        all_nodes.insert(from); all_nodes.insert(to);
        adj.entry(from).or_default().push(to);
    }

    let mut visited = HashSet::new();
    let mut order = Vec::new();

    fn visit<'a>(node: &'a str, adj: &HashMap<&'a str, Vec<&'a str>>,
                 visited: &mut HashSet<&'a str>, order: &mut Vec<String>) {
        if visited.contains(node) { return; }
        visited.insert(node);
        for &neighbor in adj.get(node).into_iter().flatten() {
            visit(neighbor, adj, visited, order);
        }
        order.push(node.to_string()); // post-order: add AFTER all descendants
    }

    for node in all_nodes { visit(node, &adj, &mut visited, &mut order); }
    order.reverse(); // reverse post-order = topological order
    order
}
```

```rust
// Kahn's algorithm: BFS with in-degree counting — also detects cycles (leftover nodes)
pub fn topo_sort_kahn(edges: &[(&str, &str)]) -> Vec<String> {
    // Count incoming edges for each node
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    // ... start with zero-in-degree nodes, process and decrement neighbors
}
```

The DFS approach is recursive and elegant but can stack-overflow on very deep graphs. Kahn's is iterative, naturally detects cycles (if `result.len() != total_nodes`, there's a cycle), and is often preferred in production.

## What This Unlocks

- **Build systems**: Cargo, Make, Gradle all use topological sort to determine compilation order.
- **Task scheduling**: CI pipelines, data pipeline orchestration (Airflow, Prefect) — any DAG of jobs.
- **Dependency resolution**: package managers use topological sort to determine install order, ensuring dependencies are installed before the packages that need them.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Visited state | Threaded through recursion as parameter | Mutable `HashSet` passed by `&mut` reference |
| Result accumulation | Accumulated in return value or parameter | Mutable `Vec` passed by `&mut` reference |
| Adjacency representation | Association list or `Hashtbl` | `HashMap<&str, Vec<&str>>` |
| Recursive helper | Local `let rec` inside the function | Nested `fn` (no closures for mutual recursion) |
| Cycle detection | Extra state in recursive function | Kahn's: check `result.len() != node_count` |
