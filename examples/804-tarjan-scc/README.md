📖 **[View on hightechmind.io →](https://hightechmind.io/rust/804-tarjan-scc)**

---

# 804-tarjan-scc — Tarjan's Strongly Connected Components
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A Strongly Connected Component (SCC) is a maximal subgraph where every vertex can reach every other vertex. Tarjan's algorithm (1972) finds all SCCs in O(V+E) time using a single DFS, tracking discovery times and low-link values. SCCs are used in compiler optimization (detecting cycles in dataflow graphs), social network analysis (finding tight-knit communities), deadlock detection, and 2-SAT (satisfiability) solvers.

## Learning Outcomes

- Track discovery time (`disc`) and low-link value (`low`) per vertex during DFS
- Use an explicit stack to track vertices that could form an SCC
- Identify SCC roots: vertices where `low[v] == disc[v]` after all descendants are processed
- Pop the SCC from the stack when a root is found
- Understand the difference between tree edges, back edges, and cross edges in the DFS

## Rust Application

`tarjan_scc(n, edges)` implements iterative DFS (avoiding stack overflow on large graphs) with nested functions for the recursive DFS logic. `indices[v]` stores discovery time; `low[v]` stores the minimum reachable discovery time. When `low[v] == indices[v].unwrap()`, pop all vertices from the stack until `v` to form an SCC. Returns `Vec<Vec<usize>>` of SCCs.

## OCaml Approach

OCaml implements Tarjan's with `ref` cells for `index_counter`, `on_stack: bool array`, and `stack: int list ref`. Recursive DFS using `let rec dfs v = ...` is natural in OCaml. The `Ocamlgraph` library provides `SCC.scc_list` using Tarjan's or Kosaraju's algorithm. OCaml's exception mechanism can implement the "pop until root" with cleaner control flow.

## Key Differences

1. **Recursion vs iteration**: Recursive Tarjan's risks stack overflow on large graphs; Rust's implementation uses an explicit stack to avoid this; OCaml's `ulimit -s` or `Thread.create` can work around the limit.
2. **Mutable state**: Tarjan's requires mutable arrays for `disc`, `low`, `on_stack`; both languages use mutable arrays directly.
3. **Applications**: 2-SAT solvers use Tarjan's SCC to check satisfiability in O(n+m) time; Rust constraint solvers use this pattern.
4. **Condensation**: The DAG of SCCs (the "condensation") has no cycles; computing it from Tarjan's output enables topological processing of cyclic graphs.

## Exercises

1. Implement the SCC condensation: build a DAG where each node is an SCC and edges represent inter-SCC connections. Verify it is a DAG.
2. Use SCCs to solve 2-SAT: given a 2-CNF formula, check satisfiability using Tarjan's and compute a satisfying assignment if one exists.
3. Detect and report all cycles in a directed graph using the SCCs: any SCC of size > 1 or any SCC containing a self-loop is a cycle.
