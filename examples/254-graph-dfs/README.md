📖 **[View on hightechmind.io →](https://hightechmind.io/rust/254-graph-dfs)**

---

# Example 254: Graph DFS
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Perform a depth-first search (DFS) on a directed graph represented as an adjacency list, returning the nodes in pre-order visit sequence with no duplicates — even when multiple paths lead to the same node.

## Learning Outcomes

- How Rust's `HashSet` replaces OCaml's purely functional `Set.Make` module
- Iterative DFS with an explicit stack vs. recursive DFS that mirrors OCaml's structure
- Lifetime annotations on graph references: `&'a HashMap<&str, Vec<&str>>`
- Why `HashSet::insert` returning `bool` eliminates an extra `contains` check

## OCaml Approach

OCaml threads the visited set as a **pure value** through recursion — `go` returns `(new_visited, path)` and the caller receives the updated set, never mutating in place. `List.fold_left` chains neighbor visits, accumulating both the growing visited set and the partial path. The result is the clean separation of state passing that characterises pure functional code.

## Rust Approach

The idiomatic Rust solution uses an iterative stack-based DFS with a `HashSet` for O(1) visited lookup. Neighbors are pushed in reverse order so the first neighbor is processed first, matching OCaml's left-to-right traversal. The functional variant uses a recursive inner function with `&mut HashSet` — mutation is contained, the interface stays pure, and it directly parallels the OCaml `go` helper.

## Key Differences

1. **Visited set:** OCaml uses an immutable `Set.Make(String)` threaded through returns; Rust uses a mutable `HashSet<&str>` passed by `&mut` reference.
2. **Recursion vs. iteration:** OCaml's natural expression is recursive; Rust prefers an iterative stack to avoid stack-overflow on deep graphs.
3. **Graph representation:** OCaml uses association lists (`(string * string list) list`) mirroring `List.assoc`; Rust uses `HashMap<&str, Vec<&str>>` for O(1) neighbor lookup.
4. **Insert semantics:** `HashSet::insert` returns `false` when already present, letting us skip the `contains` + `insert` double-check OCaml needs with `SS.mem` + `SS.add`.

## Exercises

1. Implement iterative DFS using an explicit `Vec`-based stack instead of recursion, ensuring the traversal order matches the recursive version.
2. Use DFS to detect cycles in a directed graph by tracking the current recursion path (gray nodes) and back edges.
3. Implement Tarjan's strongly connected components algorithm using DFS with a low-link array and a stack, returning each SCC as a `Vec<NodeId>`.
