📖 **[View on hightechmind.io →](https://hightechmind.io/rust/033-at-level)**

---

# 033 — Collect the Nodes at a Given Level
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Collecting all nodes at a specific depth (OCaml 99 Problems #33) — where the root is at level 1 — is a level-order query that appears in breadth-first search, layer-by-layer neural network processing, and tree visualization. It is also the key to level-order traversal (BFS on trees).

Level queries are the bridge between depth-first structural recursion (which processes all of a subtree before moving to the next) and breadth-first level-by-level processing. Collecting all nodes at a given level can be done either with DFS (pass the target level as a decreasing counter) or with BFS (queue-based).

## Learning Outcomes

- Traverse a tree to a target depth, collecting values only at that level
- Use a decreasing level counter: `level - 1` at each Node, collect when `level == 1`
- Understand the connection between level-queries and breadth-first traversal
- Recognize that collecting all levels produces a level-order traversal
- Handle the edge case: level > tree depth returns empty list

## Rust Application

`at_level<T: Clone>(tree: &Tree<T>, level: usize) -> Vec<T>`: if `level == 1`, return the current node's value (if it's a Node). If `level > 1`, recurse with `level - 1` on both children and extend the results. Base case: `Tree::Leaf` always returns empty. The counter decrements by 1 at each level, reaching 1 at the target depth.

## OCaml Approach

OCaml's version: `let rec at_level tree level = match tree with | Leaf -> [] | Node (x, _, _) when level = 1 -> [x] | Node (_, l, r) -> at_level l (level - 1) @ at_level r (level - 1)`. The `when level = 1` guard returns the value. Otherwise it recurses deeper with decremented level.

## Key Differences

1. **Level vs depth**: The problem uses 1-based level numbering (root = level 1). Rust's natural 0-based depth (root = depth 0) requires adjustment. Choose a convention and document it.
2. **Guard syntax**: OCaml's `when level = 1` guard in the match arm. Rust: `if level == 1 { return ... }` or a separate match arm with a guard `k if k == 1`.
3. **`@` efficiency**: OCaml's `at_level l (level-1) @ at_level r (level-1)` copies the left result. Rust's `extend` approach is more memory-efficient.
4. **BFS alternative**: For collecting all levels, BFS with a queue is more efficient (O(n) total) than calling `at_level` for each level (O(n·d) total).

## Exercises

1. **Level-order traversal**: Write `level_order<T: Clone>(tree: &Tree<T>) -> Vec<Vec<T>>` that returns `[nodes_at_level_1, nodes_at_level_2, ...]` using a queue-based BFS.
2. **Maximum sum level**: Write `max_sum_level(tree: &Tree<i32>) -> usize` that returns the level with the highest sum of node values.
3. **Zigzag traversal**: Write `zigzag<T: Clone>(tree: &Tree<T>) -> Vec<Vec<T>>` like level-order but alternating left-to-right and right-to-left on each level (a common interview problem).
