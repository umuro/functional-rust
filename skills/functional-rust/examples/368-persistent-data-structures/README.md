# 368: Persistent Data Structures

**Difficulty:** 4  **Level:** Expert

Preserve previous versions of a data structure after modification via structural sharing — without copying the whole structure.

## The Problem This Solves

You're building an editor with undo history, a version-controlled key-value store, or a functional language interpreter. Every modification creates a new "version" of the data. Copying the entire structure on every change is O(n) per operation — prohibitively slow for large structures.

Persistent data structures solve this with structural sharing. When you modify a node in a tree, you create a new path from root to the modified node — but all other subtrees are shared (via reference counting) between the old and new versions. A persistent binary tree of n nodes produces a new version in O(log n) by creating O(log n) new nodes and reusing the other n - log n nodes unchanged.

In functional languages like OCaml or Haskell, all data structures are persistent by default — immutability is the norm. In Rust, mutation is the norm and persistence is opt-in via explicit `Rc` (single-threaded) or `Arc` (multi-threaded) shared ownership.

## The Intuition

Imagine a git commit tree. Each commit shares history with its parent; branching doesn't copy all files. Persistent data structures work the same way: a new "version" shares unmodified subtrees with the previous version.

The key tool in Rust is `Rc<T>` (or `Arc<T>` for thread safety). Cloning an `Rc` is O(1) — it just increments a reference count. The underlying data isn't copied. When all `Rc` handles to a node go away, the node is freed. This gives you structural sharing automatically.

The cost: every node access goes through a pointer (cache miss potential), and mutating shared data requires `Rc::make_mut` which may clone if the reference count > 1 — the copy-on-write semantic.

## How It Works in Rust

```rust
use std::rc::Rc;

// A persistent cons-list (the classic functional data structure)
// Each node is either empty or a value + shared tail
#[derive(Clone)]
enum List<T: Clone> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T: Clone> List<T> {
    fn empty() -> Rc<Self> { Rc::new(List::Nil) }

    // Prepend: O(1), shares the entire original list
    fn cons(head: T, tail: Rc<Self>) -> Rc<Self> {
        Rc::new(List::Cons(head, tail))
    }

    fn head(&self) -> Option<&T> {
        match self { List::Cons(v, _) => Some(v), List::Nil => None }
    }

    fn tail(&self) -> Option<Rc<Self>> {
        match self { List::Cons(_, t) => Some(Rc::clone(t)), List::Nil => None }
    }
}

// Two lists can share a common tail — no copying
let shared_tail = List::cons(3, List::cons(4, List::empty()));
let list_a = List::cons(1, Rc::clone(&shared_tail)); // [1,3,4]
let list_b = List::cons(2, Rc::clone(&shared_tail)); // [2,3,4]
// shared_tail's memory is kept alive by both list_a and list_b

// Persistent binary tree (immutable BST)
#[derive(Clone)]
enum Tree<T: Clone + Ord> {
    Leaf,
    Node { val: T, left: Rc<Tree<T>>, right: Rc<Tree<T>> },
}

impl<T: Clone + Ord> Tree<T> {
    fn insert(&self, v: T) -> Rc<Tree<T>> {
        match self {
            Tree::Leaf => Rc::new(Tree::Node {
                val: v,
                left: Rc::new(Tree::Leaf),
                right: Rc::new(Tree::Leaf),
            }),
            Tree::Node { val, left, right } => {
                if v < *val {
                    // Only creates new nodes along the left path — O(log n) new nodes
                    // The right subtree is shared unchanged
                    Rc::new(Tree::Node {
                        val: val.clone(),
                        left: left.insert(v),
                        right: Rc::clone(right), // shared, not copied
                    })
                } else {
                    Rc::new(Tree::Node {
                        val: val.clone(),
                        left: Rc::clone(left),  // shared, not copied
                        right: right.insert(v),
                    })
                }
            }
        }
    }
}

// Version history: each insert produces a new root, old roots still valid
let v0 = Rc::new(Tree::Leaf);
let v1 = v0.insert(5);
let v2 = v1.insert(3);
let v3 = v2.insert(7);
// v0, v1, v2, v3 all valid simultaneously; shared subtrees not duplicated
```

## What This Unlocks

- **Undo/redo systems**: keep a `Vec<Rc<State>>` as your undo stack; each state shares structure with its neighbors — memory usage grows logarithmically, not linearly.
- **Functional language runtimes**: implement immutable collections (list, map, set) with O(log n) update and automatic memory management via `Rc`.
- **Concurrent snapshots**: use `Arc<T>` instead of `Rc<T>` to share structure across threads without locking — readers hold their own version while writers produce new roots.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Immutable by default | yes — all values are immutable | no — mutation is the default |
| Structural sharing | automatic (GC manages lifetime) | explicit `Rc<T>` or `Arc<T>` |
| Persistent list | built-in `list` type | `Rc<List<T>>` (custom) |
| Copy-on-write | N/A (immutable = always shared) | `Rc::make_mut` clones if rc > 1 |
| Thread-safe sharing | GC-managed | `Arc<T>` instead of `Rc<T>` |
| Memory reclaim | garbage collector | reference count drops to zero |
