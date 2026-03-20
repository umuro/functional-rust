📖 **[View on hightechmind.io →](https://hightechmind.io/rust/029-binary-tree)**

---

# 029 — Binary Tree (Algebraic Data Type)

## Problem Statement

A binary tree is the most important recursive data structure in computer science. It underlies binary search trees (std::collections::BTreeMap), heaps (priority queues), Huffman coding trees (compression), parse trees (compilers), and spatial partitioning (k-d trees, R-trees). The recursive definition — a tree is either a leaf or a node with a value and two subtrees — maps perfectly to algebraic data types.

OCaml 99 Problems #29 introduces the `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree` as the central data type for problems 29-40. This is the canonical example of an algebraic data type (ADT), a sum type with two constructors. Understanding how to define and process ADTs is foundational to functional programming.

## Learning Outcomes

- Define a generic recursive enum `Tree<T>` in Rust as the equivalent of OCaml's `'a tree`
- Understand why `Box` is required for recursive enum variants in Rust
- Use `Tree::node()` and `Tree::leaf()` helper constructors for cleaner tree construction
- Implement basic operations: size, depth, membership via pattern matching
- Recognize how the tree structure mirrors the recursive function structure

## Rust Application

The `Tree<T>` enum has two variants: `Leaf` (no data) and `Node(T, Box<Tree<T>>, Box<Tree<T>>)`. The `Box` is required because Rust must know the size of each variant at compile time — a `Node` containing a `Tree` directly would be infinitely large. Helper constructors `Tree::node(val, left, right)` and `Tree::leaf()` hide the boxing. `size` and `depth` use pattern matching recursion mirroring the structure of the type. `mem` adds a `PartialEq` bound for value comparison.

## OCaml Approach

OCaml defines `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree`. No boxing is needed because OCaml values are uniformly represented as tagged pointers on the heap — the GC handles recursive types automatically. Functions follow the same recursive structure: `let rec size = function Leaf -> 0 | Node (_, l, r) -> 1 + size l + size r`. The `function` keyword is shorthand for `fun x -> match x with`.

## Key Differences

1. **`Box` for recursion**: Rust needs `Box<Tree<T>>` because the compiler must compute the stack frame size. OCaml's uniform heap representation avoids this — all values are pointer-sized.
2. **Type parameter syntax**: Rust: `Tree<T>`. OCaml: `'a tree`. Both are parametric polymorphism; the syntax differs.
3. **Helper constructors**: Rust often defines `fn node(val, left, right)` to hide boxing. OCaml's `Node (v, l, r)` is direct — no hiding needed.
4. **`PartialEq` for `mem`**: Rust requires explicit `T: PartialEq` trait bound for value comparison. OCaml's structural equality works on all types automatically.

## Exercises

1. **Mirror**: Write `mirror(tree: Tree<T>) -> Tree<T>` that swaps left and right children at every node. Verify that `mirror(mirror(t)) == t`.
2. **Balanced**: Write `is_balanced<T>(tree: &Tree<T>) -> bool` that returns true if no two leaves differ in depth by more than 1. Use `depth` from the implementation.
3. **Path to node**: Write `path_to<T: PartialEq>(tree: &Tree<T>, target: &T) -> Option<Vec<bool>>` that returns the path from root to the target (false = go left, true = go right), or `None` if not found.
