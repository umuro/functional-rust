# Trie-based Persistent Vector for O(log n) Random Access

**Source:** https://cs3110.github.io/textbook/chapters/ds/sequences.html

**Difficulty:** Advanced

## Problem Statement

Purely functional data structures must never mutate in place — every update produces a new version while preserving the old one. Naive approaches copy the entire structure on each update, giving O(n) time. A trie-based persistent vector solves this by sharing unchanged subtrees between versions, achieving O(log n) reads and writes while remaining fully immutable. This structure underpins Clojure's persistent vector and any system requiring cheap snapshots or time-travel debugging.

## Learning Outcomes

- Understand why persistent data structures require structural sharing to be efficient
- Learn how a 32-way branching trie achieves O(log₃₂ n) practical amortized access
- See how `Arc` in Rust enables safe structural sharing without a garbage collector
- Appreciate the trade-off between mutability performance and version-preserving guarantees

## Rust Application

Rust's ownership model pairs naturally with persistent structures: `Arc<Node>` lets multiple versions share subtrees without deep cloning. When a path needs updating, only the O(log n) nodes along that path are cloned; all other subtrees are shared via `Arc::clone`. The compiler enforces that no node is mutated after being shared, guaranteeing the persistence invariant at compile time with no runtime checks.

## OCaml Approach

OCaml's garbage collector handles structural sharing automatically — the GC keeps shared nodes alive as long as any version references them. Persistent vectors are typically implemented as trees of arrays (`type 'a t = Leaf of 'a array | Node of 'a t array`). Because OCaml values are immutable by default, sharing is the natural result of returning old sub-nodes without copying.

## Key Differences

1. **Memory management**: OCaml relies on GC for sharing; Rust uses `Arc<T>` with explicit reference counting and no GC pauses.
2. **Mutability default**: OCaml values are immutable by default, making persistence natural; Rust requires `Arc` wrapping to opt into sharing.
3. **Performance profile**: Rust's `Arc` has atomic overhead on clone/drop; OCaml's GC has periodic pause overhead — different latency trade-offs.
4. **Type encoding**: OCaml uses polymorphic recursive types directly; Rust requires `Box` or `Arc` to break recursive type size cycles.

## Exercises

1. Implement `get(index)` on a simple binary trie (branching factor 2) and verify O(log n) node visits with a counter.
2. Add `set(index, value)` returning a new root, sharing all unchanged subtrees via `Arc::clone`.
3. Extend to support `push_back` that grows the trie by one level when the current depth is exhausted.
