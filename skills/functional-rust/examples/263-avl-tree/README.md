# 263: AVL Tree

**Difficulty:** 3  **Level:** Advanced

A self-balancing binary search tree that guarantees O(log n) operations by automatically rotating after every insert.

## The Problem This Solves

A plain binary search tree degenerates on sorted input. Insert `[1, 2, 3, 4, 5]` into an unbalanced BST and you get a linked list — O(n) search, O(n) insert, O(n) everything. That's fine for random data but catastrophic for sorted or nearly-sorted data, which is exactly what production systems encounter (log timestamps, user IDs, sorted imports).

An AVL tree fixes this by tracking the height of every subtree and rebalancing via rotations whenever the left/right heights differ by more than 1. The invariant holds after every insertion, so you're always within one rotation of balance. The result: O(log n) search, insert, and delete — guaranteed, regardless of input order.

This example builds a complete AVL tree from scratch in Rust, showing how move semantics make structural sharing explicit, how named enum fields improve readability over positional tuples, and how `Box` ownership drives the tree reconstruction during rotations.

## The Intuition

A self-balancing BST that tracks heights and applies left/right rotations after each insert to keep the tree no more than 1 level out of balance.

## How It Works in Rust

```rust
// Each node stores: left child, value, right child, and cached height.
// height = 1 + max(left.height, right.height)
pub enum Avl<T> {
    Empty,
    Node { left: Box<Avl<T>>, value: T, right: Box<Avl<T>>, height: i32 },
}

// balance_factor = left.height - right.height
// > 1: left-heavy → rotate right
// < -1: right-heavy → rotate left

fn rebalance(self) -> Self {
    let bf = self.balance_factor();
    if bf > 1      { self.rotate_right() }  // left subtree too tall
    else if bf < -1 { self.rotate_left()  }  // right subtree too tall
    else            { self }                  // already balanced
}

// insert is recursive + rebalance on the way back up
pub fn insert(&self, x: T) -> Self {
    match self {
        Avl::Empty => Self::node(Avl::Empty, x, Avl::Empty),
        Avl::Node { left, value, right, .. } => match x.cmp(value) {
            Ordering::Less    => Self::node(left.insert(x), value.clone(), (**right).clone()).rebalance(),
            Ordering::Greater => Self::node((**left).clone(), value.clone(), right.insert(x)).rebalance(),
            Ordering::Equal   => self.clone(),  // no duplicates
        },
    }
}
```

Rotations consume `self` by value (move semantics) and reconstruct the tree. This makes the structural change explicit — you can see exactly which subtrees move where.

## What This Unlocks

- **Sorted iteration in O(n):** `inorder()` traversal yields elements in sorted order without any extra sorting step.
- **Balanced search in O(log n):** Unlike `HashMap`, AVL trees maintain order — you can find min/max, predecessor, and successor efficiently.
- **Foundation for interval trees, segment trees:** AVL is the starting point for augmented BSTs that answer range queries.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Node type | `type 'a avl = Empty \| Node of 'a avl * 'a * 'a avl * int` | `enum Avl<T> { Empty, Node { left, value, right, height } }` |
| Named fields | Positional (tuple in variant) | Named struct fields in enum variant |
| Rotation | Pattern match, create new nodes (GC handles old) | Consume `self` by move, reconstruct — borrow checker enforces |
| Cloning | Implicit sharing (GC) | Explicit `.clone()` required |
| Height type | `int` (native) | `i32` (explicit) |
