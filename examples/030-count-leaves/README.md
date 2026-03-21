📖 **[View on hightechmind.io →](https://hightechmind.io/rust/030-count-leaves)**

---

# 030 — Count the Leaves of a Binary Tree
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Counting the leaves (nodes with no children) of a binary tree (OCaml 99 Problems #30) is a simple structural recursion exercise. A leaf is a `Leaf` variant in our tree type. Counting leaves is useful in analyzing tree balance (more leaves = more complete), computing Huffman code lengths (each leaf is one codeword), and measuring the branching factor of search trees.

The problem introduces the pattern of accumulating a count by traversing an entire tree — the foundation for all tree analytics: depth, node count, sum of values, max value, and tree validation all use the same recursive traversal pattern.

## Learning Outcomes

- Count leaf nodes using structural recursion on the `Tree<T>` type
- Distinguish leaf nodes (`Tree::Leaf`) from internal nodes (`Tree::Node`)
- Recognize the base case (Leaf → 1) and recursive case (Node → count left + count right)
- Apply the same traversal skeleton to other counting/accumulation problems
- Understand the relationship between leaf count and the number of internal nodes

## Rust Application

`count_leaves` follows the tree structure directly: `match tree { Tree::Leaf => 1, Tree::Node(_, l, r) => count_leaves(l) + count_leaves(r) }`. The base case counts each leaf as 1; the recursive case sums left and right leaf counts. Note that `Tree::Node` itself does not contribute to the leaf count — only `Tree::Leaf` does. The value stored in a `Node` is ignored via `_`.

## OCaml Approach

OCaml's version: `let rec count_leaves = function | Leaf -> 1 | Node (_, l, r) -> count_leaves l + count_leaves r`. The pattern is identical — the `function` keyword matches directly on the tree. Leaf count is a classic example where the value at nodes is irrelevant, so it is bound to `_`.

## Key Differences

1. **Symmetric structure**: The Rust and OCaml implementations are nearly identical in structure — this is the point. Algebraic data types + pattern matching produce code whose shape mirrors the data.
2. **Leaf definition**: In this tree type, a `Leaf` is a null node (no value). Some tree types define leaves as `Node` values with null children. The definition affects the count.
3. **Catamorphism**: `count_leaves` is a catamorphism — it replaces each constructor with a function. `Leaf` → 1, `Node(_, l, r)` → `l_count + r_count`. Example 080 generalizes this pattern.
4. **Accumulator variant**: A tail-recursive version would use an accumulator: `count_leaves_acc tree acc` adds 1 for each Leaf. But Rust won't TCO this either way since the tree recursion is not tail-recursive.

## Exercises

1. **Count nodes**: Write `count_nodes<T>(tree: &Tree<T>) -> usize` that counts all nodes (both leaves and internal nodes). Verify that `count_nodes(t) == count_leaves(t) + count_internal(t)`.
2. **Count internal nodes**: Write `count_internal<T>(tree: &Tree<T>) -> usize` that counts only nodes that have at least one non-leaf child.
3. **Leaf fraction**: Write `leaf_fraction<T>(tree: &Tree<T>) -> f64` that returns `count_leaves(t) / count_nodes(t) as f64`. For a complete binary tree of depth d, this is approximately 0.5.
