📖 **[View on hightechmind.io →](https://hightechmind.io/rust/034-complete-tree)**

---

# 034 — Construct a Complete Binary Tree

## Problem Statement

A complete binary tree of n nodes (OCaml 99 Problems #34) fills levels from left to right — every level except possibly the last is fully filled, and the last level has all nodes to the left. This is the structural property of binary heaps and array-based trees. Index `i` has children at `2i+1` and `2i+2`; parent at `(i-1)/2`.

Complete binary trees are used in heap data structures (priority queues: `BinaryHeap` in Rust's standard library), binary indexed trees (Fenwick trees for prefix sums), and segment trees. The construction algorithm distributes n nodes optimally to minimize height, which is `⌊log₂(n)⌋ + 1`.

## Learning Outcomes

- Construct a complete binary tree from n node values
- Understand the relationship between node count, tree height, and completeness
- Use the recursive formula: left subtree gets `(n-1)/2` nodes (or `(n)/2` depending on fullness of last level)
- Verify completeness: all levels full except possibly the last, which is left-justified
- Connect complete binary trees to heap data structures

## Rust Application

The construction proceeds recursively: for n nodes, the left subtree gets `l = (n - 1 + ((n-1) % 2)) / 2` nodes and the right gets `n - 1 - l` nodes. The root takes one node. This distributes nodes to ensure left-justification. Building from a slice: `complete_tree(values, 0, n)` where index 0 is the root and children of index i are at `2i+1` and `2i+2`.

## OCaml Approach

OCaml's version: `let rec complete_binary_tree n = if n = 0 then Leaf else let l = (n - 1) / 2 + (if (n - 1) mod 2 > 0 then 1 else 0) in Node ('x', complete_binary_tree l, complete_binary_tree (n - 1 - l))`. The formula ensures the left subtree is at least as large as the right, maintaining left-justification.

## Key Differences

1. **Array representation**: Complete binary trees map perfectly to arrays (level-order indexing). Rust's `BinaryHeap` uses `Vec` internally. The recursive tree is rarely used in practice for heaps.
2. **Height calculation**: For n nodes, height is `⌊log₂(n)⌋`. Rust: `(n as f64).log2().floor() as usize`. OCaml: `int_of_float (log (float_of_int n) /. log 2.0)`.
3. **Left vs right subtree size**: The exact formula for how many nodes go to each subtree depends on which level is partial. Off-by-one errors here produce trees that are complete but not left-justified.
4. **Heapify**: Rust's standard `BinaryHeap::from(vec)` uses the Floyd heapify algorithm (O(n)) rather than constructing a complete tree first. Understanding the tree structure helps reason about heap operations.

## Exercises

1. **Verify completeness**: Write `is_complete<T>(tree: &Tree<T>) -> bool` that checks if a tree is complete. Use level-order traversal — once you see a non-full node, all subsequent nodes must be leaves.
2. **From sorted array**: Build a complete binary search tree from a sorted `Vec<i32>`. For a sorted array, the median becomes the root, left half becomes the left subtree, right half the right subtree.
3. **Heap operations**: Using your complete tree, implement `heap_insert` and `heap_extract_min` that maintain the heap property. Compare with Rust's built-in `BinaryHeap`.
