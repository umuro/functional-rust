📖 **[View on hightechmind.io →](https://hightechmind.io/rust/073-topological-sort)**

---

# 073 — Topological Sort
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Topological sort orders the nodes of a directed acyclic graph (DAG) such that for every directed edge (u → v), node u appears before v in the ordering. It answers the question: "in what order should I do these tasks given their dependencies?" Build systems (Bazel, Make, Cargo), package managers (npm, pip), spreadsheet recalculation, and database schema migrations all use topological sort.

The DFS-based algorithm (Tarjan, 1976) marks each node, visits all its dependencies first (post-order DFS), then adds the node to the result. Kahn's algorithm (BFS-based) is an alternative that also detects cycles.

## Learning Outcomes

- Implement DFS-based topological sort with a visited set and post-order accumulation
- Build an adjacency list from edge pairs using `HashMap<&str, Vec<&str>>`
- Use a `HashSet` for O(1) visited checking
- Understand the post-order property: a node is added AFTER all its descendants
- Recognize that the reversed post-order DFS is a valid topological ordering

## Rust Application

`topo_sort(edges: &[(&str, &str)]) -> Vec<String>` builds an adjacency list, then calls a nested `visit` function for each unvisited node. `visit` marks the node as visited, recurses on all neighbors, then `push`es the node to `order` — post-order. The final `order` is in topological order (dependencies come before dependents after a `reverse()`).

## OCaml Approach

OCaml's version uses a hash table for the visited set and an adjacency list: `let topo_sort edges = let visited = Hashtbl.create 16 in let order = ref [] in let rec visit node = if not (Hashtbl.mem visited node) then begin Hashtbl.add visited node (); List.iter visit (adj node); order := node :: !order end in List.iter (fun (a, b) -> visit a; visit b) edges; !order`. The `order := node :: !order` builds in post-order (prepending naturally reverses).

## Key Differences

1. **Nested function**: Rust's nested `fn visit<'a>(...)` inside `topo_sort` requires explicit lifetime annotations for the borrowed adjacency map. OCaml closures capture the environment implicitly.
2. **`HashSet` vs `Hashtbl`**: Rust's `HashSet` for visited nodes. OCaml's `Hashtbl` serves both set and map purposes. Both provide O(1) lookup.
3. **Cycle detection**: This implementation does not detect cycles (silently handles them by skipping visited nodes, which may produce incorrect output). A full implementation needs a "in progress" visited state (three-color DFS).
4. **Ordering**: The post-order accumulation produces dependencies-last. Reversing gives dependencies-first. Kahn's algorithm naturally produces dependencies-first without reversal.

## Exercises

1. **Cycle detection**: Extend `topo_sort` to return `Result<Vec<String>, Vec<String>>` where the error contains the nodes in the detected cycle. Use three-color DFS: white (unvisited), gray (in progress), black (done).
2. **Kahn's algorithm**: Implement topological sort using Kahn's algorithm: repeatedly remove nodes with in-degree 0. Return `None` if a cycle prevents completion.
3. **Build order**: Given Cargo-style `[[dependencies]]` as a list of `(package, depends_on)` pairs, produce a valid build order using topological sort.
