📖 **[View on hightechmind.io →](https://hightechmind.io/rust/061-binary-tree)**

---

# 061 — Binary Tree (Size, Membership, Traversal)

## Problem Statement

This example implements a generic binary tree with the core operations from OCaml's CS3110 course: size, depth, membership, and traversal. Unlike the 99 Problems tree (examples 029-040) which focuses on structural puzzles, this tree is a programming exercise in generics and trait bounds — the foundation for building a binary search tree (BST).

Binary trees are the basis for BSTs (`BTreeMap`, `BTreeSet`), heaps (`BinaryHeap`), Huffman coding, and expression trees in compilers. Understanding the generic `Tree<T>` with trait bounds (`PartialEq` for `mem`, `Ord` for BST) is prerequisite for implementing any tree-based data structure.

## Learning Outcomes

- Define `Tree<T>` as a generic recursive enum with `Box` for heap allocation
- Use `T: PartialEq` as a trait bound for membership testing
- Implement `size`, `depth`, `mem`, and in/pre/post-order traversal
- Use helper constructors to avoid repeated `Box::new` at call sites
- Recognize how the tree structure shapes the recursive function structure

## Rust Application

`Tree<T>` uses `Tree::Leaf` (no value) and `Tree::Node(T, Box<Tree<T>>, Box<Tree<T>>)`. `size` counts nodes, `depth` computes height, `mem` searches for a value requiring `T: PartialEq`. Traversal functions (`inorder`, `preorder`, `postorder`) return `Vec<T>` using recursive extend. The `node` and `leaf` helper constructors package the `Box::new` calls.

## OCaml Approach

OCaml's CS3110 tree: `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree`. Operations: `let rec size = function Leaf -> 0 | Node (_, l, r) -> 1 + size l + size r`. Membership: `let rec mem x = function Leaf -> false | Node (v, l, r) -> v = x || mem x l || mem x r`. No boxing needed — OCaml's GC handles heap allocation for recursive types.

## Key Differences

1. **`Box` for recursion**: Rust requires `Box<Tree<T>>` in Node. OCaml allocates all heap values uniformly — no explicit boxing.
2. **Trait bounds**: Rust requires explicit `T: PartialEq` for `mem`. OCaml's structural equality works on all types automatically.
3. **Helper constructors**: Rust's `Tree::node(v, l, r)` hides `Box::new`. OCaml: `Node (v, l, r)` is direct.
4. **Derive macros**: Rust can `#[derive(Debug, Clone, PartialEq)]` on Tree. OCaml's structural equality and printing work automatically.

## Exercises

1. **BST operations**: Add `insert(x: T, tree: Tree<T>) -> Tree<T>` and `search(x: &T, tree: &Tree<T>) -> bool` for a BST (requiring `T: Ord`). Maintain the BST invariant: left < root < right.
2. **Level-order**: Write `level_order(tree: &Tree<T>) -> Vec<Vec<T>>` using a queue-based BFS. Compare output with `preorder` and `inorder`.
3. **To sorted vec**: Write `to_sorted_vec<T: Ord + Clone>(tree: &Tree<T>) -> Vec<T>` by inorder traversal of a BST. Verify it produces a sorted sequence.
