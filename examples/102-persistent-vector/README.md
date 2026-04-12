# Example 102: Persistent Vector

**Difficulty:** ⭐⭐⭐
**Category:** Data Structures
**OCaml Source:** Functional Programming in OCaml — balanced tree persistent array

## Problem Statement

Implement a persistent (immutable) vector backed by a balanced binary tree, where `set` returns a new version sharing all unchanged subtrees with the original.

## Learning Outcomes

- How algebraic data types model recursive tree structures in Rust
- `Rc<T>` enables structural sharing — unchanged subtrees are pointer-aliased, not copied
- Functional updates: `set` is O(log n) new allocations, not O(n) full copy
- The difference between `Rc<T>` (shared ownership, no sharing guarantees in OCaml) and OCaml's GC (automatic sharing via value identity)

## OCaml Approach

OCaml uses a sum type `'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec`. Since OCaml is garbage-collected, `set` naturally shares the unchanged branch — the GC handles aliasing automatically. The programmer writes purely functional code and the runtime provides structural sharing for free.

## Rust Approach

Rust requires explicit ownership management. `Rc<PVec<T>>` gives shared ownership so unchanged subtrees can be aliased across versions. `Rc::clone` in the `set` path only bumps a reference count (O(1)), while only the nodes on the path from root to the updated leaf are newly allocated (O(log n) total).

## Key Differences

1. **Memory model:** OCaml GC shares heap nodes automatically; Rust needs `Rc<T>` to express shared ownership explicitly.
2. **Error handling:** OCaml uses `failwith` (exceptions); Rust returns `Option<T>` — out-of-bounds is a value, not a control-flow exception.
3. **Clone semantics:** `Rc::clone` is a pointer bump (O(1)); cloning a `Box<T>` is a deep copy (O(subtree size)).
4. **Type bounds:** OCaml's parametric polymorphism is unconstrained; Rust's `from_slice` and `set` require `T: Clone` explicitly.
