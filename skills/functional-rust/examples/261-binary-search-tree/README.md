# 261: Binary Search Tree — Insert and Search

**Difficulty:** 2  **Level:** Intermediate

Pure functional BST — insert, search, and traverse without mutation, preserving the original tree on every operation.

## The Problem This Solves

A binary search tree maintains sorted order automatically: every node's left subtree holds smaller values, every node's right subtree holds larger values. This ordering makes search O(log n) on balanced trees — far better than scanning a sorted list sequentially.

The *functional* version never mutates. Every `insert` returns a new tree that shares structure with the original. The original tree remains unchanged — useful when you need snapshots, undo history, or concurrent reads without locks. This is *persistence* in the data structures sense: old versions persist alongside new ones.

OCaml's GC handles persistence transparently — unchanged subtrees are shared by reference at no cost. In Rust, shared immutable substructure requires `Arc<T>` or explicit cloning. This example uses cloning to keep the code simple and readable; the cost of persistence is visible and explicit.

## The Intuition

A BST is organised by a simple invariant: for any node with value `v`, everything in its left subtree is less than `v`, and everything in its right subtree is greater than `v`. Searching follows this invariant: if your target is less than the current node, go left; if greater, go right; if equal, you found it.

Functional insert follows the same path down the tree, rebuilding each node along the path from root to insertion point. Nodes *off* the path are cloned unchanged. The result is a new tree that shares most of its structure with the old one — only the path from root to the new leaf is new.

`Box<Bst<T>>` is required because `Bst<T>` contains itself. Without the `Box`, the compiler can't determine the size of `Bst<T>` at compile time — it would be infinite. `Box` allocates on the heap, making the size known (one pointer).

## How It Works in Rust

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Bst<T> {
    Leaf,                              // empty tree / empty subtree
    Node(Box<Bst<T>>, T, Box<Bst<T>>), // left, value, right
}

impl<T: Ord + Clone> Bst<T> {
    // Functional insert: returns a new tree, original unchanged
    pub fn insert(&self, x: T) -> Self {
        match self {
            Bst::Leaf => Bst::Node(Box::new(Bst::Leaf), x, Box::new(Bst::Leaf)),
            Bst::Node(left, val, right) => match x.cmp(val) {
                Ordering::Less =>
                    Bst::Node(Box::new(left.insert(x)), val.clone(), right.clone()),
                Ordering::Greater =>
                    Bst::Node(left.clone(), val.clone(), Box::new(right.insert(x))),
                Ordering::Equal => self.clone(), // duplicate — no change
            },
        }
    }

    // Search: borrow the tree, O(log n) for balanced trees
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Bst::Leaf => false,
            Bst::Node(left, val, right) => match x.cmp(val) {
                Ordering::Less    => left.contains(x),
                Ordering::Greater => right.contains(x),
                Ordering::Equal   => true,
            },
        }
    }

    // In-order traversal yields sorted sequence
    pub fn to_sorted_vec(&self) -> Vec<T> {
        match self {
            Bst::Leaf => vec![],
            Bst::Node(left, val, right) => {
                let mut result = left.to_sorted_vec();
                result.push(val.clone());
                result.extend(right.to_sorted_vec());
                result
            }
        }
    }
}
```

## What This Unlocks

- **Persistent sorted sets** — insert into a BST and keep the old version; compare two snapshots without copying the whole structure.
- **Sorted stream processing** — insert arriving events by timestamp; in-order traversal gives them back sorted.
- **Foundation for balanced trees** — once BST invariants are clear, extending to AVL or red-black trees is a well-defined next step.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive type | GC handles transparently | `Box<Bst<T>>` required for known size |
| Comparison | `<`, `>`, `=` work on all types | `Ord` trait bound explicit; `.cmp()` returns `Ordering` |
| Persistence cost | Unchanged subtrees shared via GC | Must `.clone()` each node on the insert path |
| Type parameters | `'a bst` — no constraints | `T: Ord + Clone` — bounds stated explicitly |
| Leaf node | `Leaf` constructor | `Bst::Leaf` — same pattern |
