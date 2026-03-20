📖 **[View on hightechmind.io →](https://hightechmind.io/rust/038-tree-inorder)**

---

# 038 — Inorder Traversal Sequence

## Problem Statement

Inorder traversal visits nodes in left-subtree, root, right-subtree order. For a binary search tree (BST), inorder traversal produces a sorted sequence — this is the key property that makes BSTs useful for range queries. For expression trees, inorder with parentheses produces the standard algebraic notation `"3 + 4"`.

Unlike preorder, inorder alone does not uniquely determine a binary tree (many trees can produce the same inorder sequence). However, combining preorder + inorder uniquely determines any binary tree. This pair is used in tree serialization protocols and in algorithms that reconstruct trees from traversal data.

## Learning Outcomes

- Implement inorder traversal: left subtree, then root, then right subtree
- Understand that inorder of a BST produces a sorted sequence
- Use inorder + preorder to reconstruct a unique binary tree
- Contrast the three traversal orders: pre, in, post
- Recognize inorder as the "natural" reading order for mathematical expressions

## Rust Application

`inorder(tree: &Tree<char>) -> Vec<char>`: `Leaf` → `vec![]`, `Node(c, l, r)` → `collect inorder(l), then c, then inorder(r)`. The implementation extends a `Vec`: `let mut result = inorder(l); result.push(*c); result.extend(inorder(r)); result`. As a string version: concatenate `inorder_str(l) + c + inorder_str(r)`.

## OCaml Approach

OCaml's version: `let rec inorder = function | Leaf -> [] | Node (x, l, r) -> inorder l @ [x] @ inorder r`. For a BST where the tree maintains sorted order, this returns values in ascending order. The string version: `inorder_str l ^ String.make 1 x ^ inorder_str r`. The `@` concatenation is O(|left|) — use accumulator style for efficiency.

## Key Differences

1. **BST sorted output**: Rust's `BTreeMap` uses inorder traversal internally to implement `iter()` — the elements come out sorted. Understanding inorder traversal explains why BTree iteration is sorted.
2. **Non-uniqueness**: Inorder sequence alone does not uniquely determine a tree. The trees `Node(1, Node(2, Leaf, Leaf), Leaf)` and `Node(2, Leaf, Node(1, Leaf, Leaf))` both have inorder `[1, 2]`. Always use preorder+inorder or inorder+postorder pairs for reconstruction.
3. **Accumulator style**: Both languages benefit from threading an accumulator through inorder traversal to avoid repeated list concatenation. The accumulator collects in reverse order; reverse at the end.
4. **In-place**: For arrays, inorder traversal can be done iteratively using an explicit stack, avoiding O(log n) recursion overhead.

## Exercises

1. **BST sorted check**: Write `is_bst(tree: &Tree<i32>) -> bool` using inorder traversal: the tree is a BST iff its inorder sequence is strictly increasing.
2. **Tree from in+post**: Given inorder and postorder sequences, reconstruct the unique binary tree. The last element of postorder is the root; find it in inorder to determine left/right subtree sizes.
3. **Threaded tree**: Research threaded binary trees, where `Leaf` pointers are replaced with pointers to the inorder predecessor/successor. This enables O(1) inorder step without recursion.
