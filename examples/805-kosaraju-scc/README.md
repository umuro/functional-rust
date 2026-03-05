📖 **[View on hightechmind.io →](https://hightechmind.io/rust/805-kosaraju-scc)**

---

# 805: Kosaraju's Two-Pass SCC Algorithm

**Difficulty:** 5  **Level:** Master

Find all strongly connected components in a directed graph using two iterative DFS passes in O(V+E).

## The Problem This Solves

A strongly connected component (SCC) is a maximal set of vertices such that every vertex is reachable from every other vertex in the set. SCCs partition any directed graph into clusters of mutual reachability — the building blocks of dependency analysis, deadlock detection, and compiler data-flow.

Use Kosaraju's algorithm when you need to decompose a directed graph into its SCCs, condense it into a DAG, or analyze cyclic dependencies. It applies directly to module dependency graphs (finding circular imports), web link analysis (finding clusters of mutually-linked pages), and program analysis (detecting loops and reachable code).

The output is a list of SCCs, each containing the vertex indices belonging to that component. After condensation, the resulting DAG gives you a topological order over the component structure.

## The Intuition

The core insight: finish order in DFS reveals the SCC structure. In a DFS of the original graph, a vertex in a "sink" SCC (no outgoing cross-SCC edges) finishes last. If you then DFS the *transposed* graph starting from the highest-finish-order vertex, you stay within that SCC — because the transposed edges can't escape it.

Two passes:
1. **Pass 1** on original graph: run DFS, push each vertex to a stack on finish. Stack top = vertex that finished last.
2. **Pass 2** on reversed graph: pop from stack, DFS from each unvisited vertex. Each DFS tree = one SCC.

Time: O(V+E). Space: O(V+E) for the reversed adjacency list.

In OCaml you'd use recursive DFS naturally. In Rust, recursion risks stack overflow on large graphs, so iterative DFS with an explicit stack and a `returning` flag to distinguish pre/post-visit is the idiomatic approach.

## How It Works in Rust

```rust
fn kosaraju(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n); // finish order

    // Pass 1: iterative DFS on original graph, collect finish order
    for start in 0..n {
        if visited[start] { continue; }
        let mut stack = vec![(start, false)]; // (node, returning)
        while let Some((v, ret)) = stack.pop() {
            if ret {
                order.push(v); // finished — push to order
                continue;
            }
            if visited[v] { continue; }
            visited[v] = true;
            stack.push((v, true)); // push return marker
            for &w in &adj[v] {
                if !visited[w] { stack.push((w, false)); }
            }
        }
    }

    // Build reversed graph
    let mut radj = vec![vec![]; n];
    for v in 0..n {
        for &w in &adj[v] { radj[w].push(v); }
    }

    // Pass 2: DFS on reversed graph in reverse finish order
    let mut comp = vec![usize::MAX; n];
    let mut sccs: Vec<Vec<usize>> = Vec::new();
    let mut visited2 = vec![false; n];

    for start in order.into_iter().rev() {
        if visited2[start] { continue; }
        let scc_id = sccs.len();
        sccs.push(vec![]);
        let mut stack = vec![start];
        while let Some(v) = stack.pop() {
            if visited2[v] { continue; }
            visited2[v] = true;
            comp[v] = scc_id;
            sccs[scc_id].push(v);
            for &w in &radj[v] {
                if !visited2[w] { stack.push(w); }
            }
        }
    }
    sccs
}
```

Key Rust detail: the `(node, returning)` tuple in the stack replaces the implicit call-stack frame that recursive DFS uses. When `returning = true`, the node has been fully explored — equivalent to code after the recursive call returns.

## What This Unlocks

- **Circular dependency detection** in build systems, package managers, and module graphs — each SCC with >1 node is a cycle.
- **DAG condensation** for topological scheduling: condense SCCs into single nodes, then topo-sort the resulting DAG.
- **2-SAT solving**: SCCs in the implication graph directly encode satisfiability — a variable and its negation in the same SCC means UNSAT.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| DFS implementation | Natural recursion (risk: stack overflow on large graphs) | Iterative with explicit `(node, returning)` stack |
| Visited tracking | `Hashtbl` or mutable array via `ref` | `vec![false; n]` — direct indexed access |
| Graph representation | `Array.make n []` with `List` adjacency | `Vec<Vec<usize>>` — owned, cache-friendly |
| Reversed graph | Functional map/fold to build transpose | Imperative loop building `radj: Vec<Vec<usize>>` |
| SCC output | List of lists | `Vec<Vec<usize>>` with component id array |
