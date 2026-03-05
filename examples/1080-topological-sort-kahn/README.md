# Example 1080: Topological Sort via Kahn's Algorithm

**Difficulty:** ⭐⭐⭐
**Category:** Graphs
**OCaml Source:** https://rosettacode.org/wiki/Topological_sort#OCaml

## Problem Statement

Implement topological sorting of a directed acyclic graph using Kahn's algorithm (in-degree counting). Detect cycles and return `None` if the graph is not a DAG.

## Learning Outcomes

- Kahn's algorithm: iterative topological sort via in-degree tracking
- `HashMap` and `VecDeque` as Rust equivalents of OCaml's `Map` module and lists
- Cycle detection as a natural byproduct of Kahn's algorithm
- DFS-based alternative with coloring for comparison

## OCaml Approach

OCaml uses functorized `Map.Make(String)` for the in-degree map and adjacency structure. The algorithm is expressed recursively: process nodes with zero in-degree, update neighbors, recurse. Lists are used as queues (not ideal for performance but idiomatic OCaml).

## Rust Approach

Rust uses `HashMap` for O(1) lookups and `VecDeque` for efficient queue operations. The iterative `while let` loop replaces OCaml's recursive processing. A DFS-based variant with coloring is also provided, showing how both algorithms detect cycles.

## Key Differences

1. **Map types:** OCaml uses `Map.Make(String)` (balanced tree, O(log n)); Rust uses `HashMap` (hash table, O(1) amortized)
2. **Queue implementation:** OCaml uses lists (O(n) dequeue); Rust uses `VecDeque` (O(1) amortized)
3. **Iteration:** OCaml recurses with pattern matching; Rust uses `while let` loops with mutable state
4. **Cycle detection:** Both return incomplete results when cycles exist; Rust wraps in `Option<Vec>` for explicit error handling
