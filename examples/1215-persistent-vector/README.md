# Example 1215: Persistent Vector

**Difficulty:** ⭐⭐⭐
**Category:** Persistent Data Structures
**OCaml Source:** Simplified persistent vector using a balanced binary tree.

## Problem Statement

Build a persistent (immutable) vector as a binary tree — `Nil`, `One`, or
`Two(left, right)` — such that every `push` and `pop` yields a fresh
version while older versions remain valid and unchanged.

## Learning Outcomes

- Persistence via **structural sharing** instead of mutation.
- Using `Rc<T>` in Rust to share subtrees cheaply between versions.
- Representing a sequence as a tree and indexing it by walking the spine.
- How `#[derive(Default)]` with `#[default]` picks the canonical empty variant.

## OCaml Approach

OCaml algebraic data types are immutable by default: rebuilding a `Two (l, r)`
node reuses `l` and `r` without copying.  `push` returns a new tree,
`pop` returns `Some (value, new_tree)`, and the original value keeps
pointing at the subtrees it already had.  Garbage collection frees
subtrees once no version references them.

## Rust Approach

Rust does not ship a GC, so we replace each child pointer with `Rc<PVec<T>>`.
Cloning the vector is an `Rc::clone` — O(1), no deep copy.  `push` wraps
the old vector in a new `Two` node in O(1); `pop` descends the right
spine and collapses `Two(l, Nil)` back to `l` so the tree does not
accumulate empty nodes.  Generics use `T: Clone` because `pop` moves
a value out by cloning (we cannot own the inside of an `Rc` uniquely).

## Key Differences

1. **Sharing mechanism:** OCaml shares transparently via GC; Rust uses `Rc`
   with explicit reference counting.
2. **Clone bound:** OCaml can return `'a` freely; Rust needs `T: Clone`
   to produce an owned value from behind an `Rc`.
3. **Default empty variant:** OCaml writes `Nil`; Rust derives `Default`
   and marks `Nil` with `#[default]`.
4. **Length:** both are O(n) here — a real persistent vector (Clojure/Scala)
   caches length per node for O(1); this simplified tree does not.
