📖 **[View on hightechmind.io →](https://hightechmind.io/rust/263-avl-tree)**

---

# Example 263: AVL Tree — Self-Balancing BST

**Difficulty:** ⭐⭐⭐  
**Category:** Trees  
**OCaml Source:** https://rosettacode.org/wiki/AVL_tree#OCaml

## Problem Statement

Implement an AVL tree — a self-balancing binary search tree where the heights of left and right subtrees differ by at most 1. The tree automatically rebalances via rotations after each insert.

## Learning Outcomes

- Implementing complex recursive data structures with named enum fields
- Translating OCaml's nested pattern matching on constructors to Rust
- Using `Box` ownership transfer for tree rotations (consuming and restructuring)
- Maintaining invariants (balance factor) in persistent data structures

## OCaml Approach

OCaml stores the height in each node: `Node of 'a avl * 'a * 'a avl * int`. Rotations are pattern matches that destructure two levels of the tree. The `rebalance` function checks the balance factor and applies the appropriate rotation.

## Rust Approach

Rust uses named struct fields in the `Node` variant for clarity. Rotations consume `self` (move semantics) and reconstruct the tree, which naturally expresses the restructuring. The `Ord` trait provides generic comparison.

## Key Differences

1. **Move semantics:** Rotations in Rust consume the tree (`self` by value), making restructuring explicit; OCaml creates new nodes implicitly
2. **Named fields:** Rust's `Node { left, value, right, height }` is more readable than OCaml's positional tuple
3. **Nested destructuring:** OCaml matches two levels in one arm; Rust needs nested `match` blocks
4. **Height caching:** Both store height in nodes; Rust's `i32` vs OCaml's `int` — same idea, explicit type

## Exercises

1. Add a `rank` method that returns how many elements in the tree are strictly less than a given value, by augmenting each node with its subtree count.
2. Implement `split` that divides the AVL tree into two balanced trees: one with all elements less than a pivot, and one with all elements greater.
3. Write a property-based test suite that verifies: (1) BST ordering, (2) balance factor ≤ 1 at every node, and (3) `to_vec` returns elements in sorted order — all after a random sequence of inserts and deletes.
