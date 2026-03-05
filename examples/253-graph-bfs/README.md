📖 **[View on hightechmind.io →](https://hightechmind.io/rust/253-graph-bfs)**

---

# Example 253: Graph BFS

**Difficulty:** ⭐⭐
**Category:** Graphs
**OCaml Source:** https://cs3110.github.io/textbook/chapters/ds/bst.html

## Problem Statement

Perform a breadth-first search (BFS) on a graph represented as an adjacency list, returning all reachable nodes in level-order (closest nodes first).

## Learning Outcomes

- How to model graphs in Rust using `HashMap<&str, Vec<&str>>` vs OCaml's association lists
- Why `VecDeque` is the natural replacement for OCaml's mutable `Queue`
- How `HashSet::insert` returns a boolean, enabling a clean visited-check-and-mark in one step
- The difference between mutable imperative BFS (OCaml + Rust both support it) and purely functional BFS

## OCaml Approach

OCaml uses a mutable `Hashtbl` for visited tracking and a mutable `Queue` for the frontier, iterating with a `while` loop. The graph is an association list `(string * string list) list`, looked up with `List.assoc`. The result accumulates in a `ref` list that is reversed at the end.

## Rust Approach

Rust mirrors the imperative OCaml style closely: `HashSet` replaces `Hashtbl`, `VecDeque` replaces `Queue`, and `HashMap` replaces the association list. The key idiom is `if visited.insert(neighbor)` — `HashSet::insert` returns `false` if the element was already present, combining the membership test and insertion into one expression.

## Key Differences

1. **Graph representation:** OCaml uses `(key, value) list` with `List.assoc` (O(n) lookup); Rust uses `HashMap` (O(1) average lookup).
2. **Queue:** OCaml's `Queue` is doubly-ended by default; Rust's `VecDeque` is explicit about push/pop ends (`push_back` / `pop_front`).
3. **Visited set:** OCaml uses `Hashtbl.mem` + `Hashtbl.add` (two operations); Rust's `HashSet::insert` returns a `bool`, combining both into one.
4. **Result accumulation:** OCaml uses a `ref` list prepended in reverse then `List.rev`; Rust uses `Vec::push` directly in order.
