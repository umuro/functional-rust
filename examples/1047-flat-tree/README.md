📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1047-flat-tree)**

---

# 1047-flat-tree — Flat Binary Tree in Vec
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Binary heaps and segment trees store binary trees in arrays using the heap indexing formula: for a node at index `i`, the left child is at `2*i+1`, the right child at `2*i+2`, and the parent at `(i-1)/2`. This array-based tree representation eliminates pointer overhead, maximizes cache locality, and enables O(1) parent/child navigation without storing explicit pointers.

This pattern is the implementation technique behind binary heaps (`BinaryHeap`), segment trees for range queries, and complete binary trees in competitive programming.

## Learning Outcomes

- Represent a binary tree as a flat `Vec<T>` using heap indexing
- Navigate parent, left child, and right child using the index formulas
- Identify leaf nodes by checking whether the child index is in bounds
- Perform in-order, pre-order, and level-order traversals
- Understand the cache locality advantages over pointer-based trees

## Rust Application

`src/lib.rs` implements `FlatTree<T>` backed by `Vec<T>`. `left_child(i) = 2*i+1`, `right_child(i) = 2*i+2`, `parent(i) = (i-1)/2` are pure arithmetic functions. `is_leaf(i)` checks `left_child(i) >= data.len()`. Tree operations on complete binary trees (heap sort, segment tree queries) are built from these three formulas.

The flat tree is used in Rust's `BinaryHeap`, in game AI (minimax trees), and in range query structures.

## OCaml Approach

OCaml's flat tree uses an array with the same index arithmetic:

```ocaml
let left_child i = 2 * i + 1
let right_child i = 2 * i + 2
let parent i = (i - 1) / 2

let is_leaf arr i = left_child i >= Array.length arr

let get arr i =
  if i < Array.length arr then Some arr.(i) else None
```

The formulas are identical. OCaml arrays are mutable by default, enabling in-place heap sort.

## Key Differences

1. **Identical formulas**: Both languages use the same `2*i+1`, `2*i+2`, `(i-1)/2` index arithmetic — this is mathematical, not language-specific.
2. **Bounds checking**: Rust's `Vec::get(i)` returns `Option<&T>` for safe out-of-bounds access; OCaml requires manual bounds checking or raises `Invalid_argument`.
3. **Heap sort**: Rust's `BinaryHeap::sort` uses this exact representation internally; OCaml's `Array.sort` uses a different algorithm.
4. **Generic constraints**: Rust's `FlatTree<T: Ord>` uses trait bounds for comparison; OCaml uses polymorphic comparison or a functor.

## Exercises

1. Implement `heapify(&mut self)` that turns any `FlatTree` into a max-heap in O(n) time using the sift-down procedure.
2. Write `in_order_traversal(&self) -> Vec<&T>` that returns elements in sorted order from a heap-organized tree.
3. Build a `SegmentTree` on top of `FlatTree<i64>` that supports range-sum queries and point updates in O(log n).
