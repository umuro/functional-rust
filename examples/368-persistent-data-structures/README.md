📖 **[View on hightechmind.io →](https://hightechmind.io/rust/368-persistent-data-structures)**

---

# 368: Persistent Data Structures

**Difficulty:** 4  **Level:** Expert

Immutable data structures with structural sharing - modifications return new versions while preserving the old.

## The Problem This Solves

In functional programming, data structures should be immutable. But naive immutability means copying entire structures on every modification - O(n) for a simple update.

Persistent data structures solve this by sharing unchanged parts between versions. A persistent list shares its tail between all versions that end the same way. A persistent tree shares all unchanged subtrees.

## Structural Sharing

When you "modify" a persistent structure:
1. Create new nodes only for the changed path
2. Reuse references to unchanged subtrees
3. Both old and new versions remain valid

This achieves O(log n) updates for trees and O(1) for list prepend.

## OCaml vs Rust

OCaml lists are naturally persistent - `h :: t` creates a new cons cell pointing to the same tail.

Rust requires explicit `Rc<T>` (or `Arc<T>`) to enable sharing, since ownership is exclusive by default.
