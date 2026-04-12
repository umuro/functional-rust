📖 **[View on hightechmind.io →](https://hightechmind.io/rust/032-internal-nodes)**

---

# 032 — Collect the Internal Nodes of a Binary Tree
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Internal nodes (non-leaf nodes) of a binary tree (OCaml 99 Problems #32) are those with at least one child that is not a `Leaf`. Collecting internal nodes is the complement of collecting leaves — together they enumerate all nodes in the tree. In a binary search tree, internal nodes are where routing decisions are made; in a parse tree, they represent grammatical rules rather than terminals.

This problem reinforces the pattern of filtering nodes by structural property during traversal. The structural condition — has at least one non-leaf child — is expressed naturally through pattern matching, without explicit null checks.

## Learning Outcomes

- Distinguish internal nodes from leaves using pattern matching on tree structure
- Collect internal node values using recursive left-right traversal
- Understand the structural predicate: a node is internal if either child is a `Node`
- Recognize that `count_internal + count_leaves = count_nodes` (by definition)
- Apply the collect-and-filter pattern to tree traversal

- Match `Tree::Node(v, _, _)` where at least one child is not `Leaf` to identify internal nodes
- Verify the complementary relationship: leaves + internal_nodes = total_nodes in property tests

## Rust Application

A node is internal if either its left or right child is `Tree::Node(...)` rather than `Tree::Leaf`. Pattern matching: `Tree::Node(v, Tree::Leaf, Tree::Leaf)` is a leaf node; `Tree::Node(v, _, _)` otherwise is internal. `collect_internal` collects `v` for each internal node and extends with results from recursive calls on left and right subtrees.

## OCaml Approach

OCaml's version: `let rec internals = function | Leaf -> [] | Node (_, Leaf, Leaf) -> [] | Node (x, l, r) -> x :: internals l @ internals r`. The second case explicitly excludes nodes with two leaf children. The third case collects the value and recurses. Like leaves, use accumulator style for efficiency: `x :: internals l @ internals r` is O(|left|) for the `@`.

## Key Differences

1. **Exhaustive matching**: Rust's `match` must cover all cases. You cannot forget the `Node(_, Leaf, Leaf)` case — omitting it is a compile error. OCaml's match is also exhaustive by default.
2. **Pattern nesting**: Matching on `Tree::Node(v, Tree::Leaf, Tree::Leaf)` vs `Tree::Node(v, _, _)` requires nested patterns. Both languages support this naturally.
3. **Value borrowing**: Rust's `v` in `Tree::Node(v, l, r)` borrows the value by reference. To collect owned copies, the bound must be `T: Clone` and you must call `.clone()`.
4. **Performance**: Collecting into a `Vec` with recursive `extend` is O(n). OCaml's `@` inside the recursive call can make this O(n·d) in the worst case if not using an accumulator.

1. **Structural predicate:** "Is internal" = "has at least one non-Leaf child". This is expressed as a pattern: `Node(_, _, _)` where at least one branch is not `Leaf`. Rust and OCaml use identical structural patterns.
2. **Complement of leaves:** `collect_internal` + `collect_leaves` = all node values. This invariant can be verified in tests.
3. **Traversal order:** Both implementations collect internal nodes in pre-order (root before children). Post-order or in-order collection is equally valid — the order depends on the use case.
4. **Efficiency:** Using an accumulator avoids the O(n·d) cost of `extend` on deep trees. Pass a `&mut Vec<T>` instead of returning a `Vec<T>` for the most efficient form.

## Exercises

1. **One-child nodes**: Write `one_child_nodes<T: Clone>(tree: &Tree<T>) -> Vec<T>` that collects nodes where exactly one child is a non-Leaf. Match on `Node(v, Leaf, Node(...))` and `Node(v, Node(...), Leaf)`.
2. **Partition tree**: Write `partition_tree<T: Clone>(tree: &Tree<T>) -> (Vec<T>, Vec<T>)` that returns `(internal_nodes, leaf_node_values)` in a single traversal.
3. **Tree statistics**: Write `tree_stats<T>(tree: &Tree<T>) -> (usize, usize, usize)` returning `(leaves, internal, total)` in one pass without making multiple calls.

4. **Internal node predicate**: Write `is_internal<T>(tree: &Tree<T>) -> bool` that returns `true` if the root node has at least one non-Leaf child — the core condition used in `collect_internal`.
5. **Internal node count**: Implement `count_internal<T>(tree: &Tree<T>) -> usize` and verify that `count_internal + count_leaves == count_nodes` in property tests using generated trees.
