📖 **[View on hightechmind.io →](https://hightechmind.io/rust/061-binary-tree)**

---

# 061 — Binary Tree (Size, Membership, Traversal)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

This example implements a generic binary tree with the core operations from OCaml's CS3110 course: size, depth, membership, and traversal. Unlike the 99 Problems tree (examples 029-040) which focuses on structural puzzles, this tree is a programming exercise in generics and trait bounds — the foundation for building a binary search tree (BST).

Binary trees are the basis for BSTs (`BTreeMap`, `BTreeSet`), heaps (`BinaryHeap`), Huffman coding, and expression trees in compilers. Understanding the generic `Tree<T>` with trait bounds (`PartialEq` for `mem`, `Ord` for BST) is prerequisite for implementing any tree-based data structure.

## Learning Outcomes

- Define `Tree<T>` as a generic recursive enum with `Box` for heap allocation
- Use `T: PartialEq` as a trait bound for membership testing
- Implement `size`, `depth`, `mem`, and in/pre/post-order traversal
- Use helper constructors to avoid repeated `Box::new` at call sites
- Recognize how the tree structure shapes the recursive function structure

- Define `Tree<T>` with `Box<Tree<T>>` subtrees and implement `size`, `depth`, and `mem` operations
- Use `T: PartialEq` as a trait bound only on `mem` — leave other operations without bounds when they don't need comparison

## Rust Application

`Tree<T>` uses `Tree::Leaf` (no value) and `Tree::Node(T, Box<Tree<T>>, Box<Tree<T>>)`. `size` counts nodes, `depth` computes height, `mem` searches for a value requiring `T: PartialEq`. Traversal functions (`inorder`, `preorder`, `postorder`) return `Vec<T>` using recursive extend. The `node` and `leaf` helper constructors package the `Box::new` calls.

## OCaml Approach

OCaml's CS3110 tree: `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree`. Operations: `let rec size = function Leaf -> 0 | Node (_, l, r) -> 1 + size l + size r`. Membership: `let rec mem x = function Leaf -> false | Node (v, l, r) -> v = x || mem x l || mem x r`. No boxing needed — OCaml's GC handles heap allocation for recursive types.

## Key Differences

1. **`Box` for recursion**: Rust requires `Box<Tree<T>>` in Node. OCaml allocates all heap values uniformly — no explicit boxing.
2. **Trait bounds**: Rust requires explicit `T: PartialEq` for `mem`. OCaml's structural equality works on all types automatically.
3. **Helper constructors**: Rust's `Tree::node(v, l, r)` hides `Box::new`. OCaml: `Node (v, l, r)` is direct.
4. **Derive macros**: Rust can `#[derive(Debug, Clone, PartialEq)]` on Tree. OCaml's structural equality and printing work automatically.

1. **Generic `T` with trait bounds:** `Tree<T>` where `T: PartialEq` for membership. The bound is required only for `mem` — other operations don't need it. Rust's monomorphization generates separate code for each concrete `T`.
2. **`Box<Tree<T>>` overhead:** Each heap allocation (one per node) has overhead. For performance-critical trees, arena allocation (`typed-arena` crate) or `Vec`-based trees are more efficient.
3. **OCaml's polymorphic equality:** OCaml's `(=)` works on any type for structural equality — no trait bound needed. Rust requires explicit `PartialEq` because the compiler doesn't know if two values of type `T` can be compared.
4. **`size` vs `len`:** Trees use `size()` or `count()` for node count. Rust's built-in collections use `len()`. The naming convention helps distinguish data structure operations from collection operations.

## Exercises

1. **BST operations**: Add `insert(x: T, tree: Tree<T>) -> Tree<T>` and `search(x: &T, tree: &Tree<T>) -> bool` for a BST (requiring `T: Ord`). Maintain the BST invariant: left < root < right.
2. **Level-order**: Write `level_order(tree: &Tree<T>) -> Vec<Vec<T>>` using a queue-based BFS. Compare output with `preorder` and `inorder`.
3. **To sorted vec**: Write `to_sorted_vec<T: Ord + Clone>(tree: &Tree<T>) -> Vec<T>` by inorder traversal of a BST. Verify it produces a sorted sequence.

4. **BST insert**: Implement `insert<T: Ord>(tree: Tree<T>, value: T) -> Tree<T>` that inserts a value into a binary search tree, maintaining the BST ordering invariant.
5. **Tree equality with `PartialEq`**: Implement `PartialEq` for `Tree<T> where T: PartialEq` and write tests verifying `t == t` (reflexivity), `t1 == t2 => t2 == t1` (symmetry), and `t1 == t2 && t2 == t3 => t1 == t3` (transitivity).
