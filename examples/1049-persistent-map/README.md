📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1049-persistent-map)**

---

# 1049-persistent-map — Persistent HashMap
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Persistent data structures return modified versions while keeping old versions intact. They are the foundation of functional programming, git-style versioning, undo/redo, and concurrent data sharing without locks. Haskell's `Data.Map`, OCaml's `Map.Make`, and Clojure's hash array mapped trie (HAMT) are all persistent.

Rust's standard `HashMap` is mutable. This example demonstrates the "clone-based" persistent map — simple but O(n) per operation — and discusses the path to efficient structural sharing via HAMT.

## Learning Outcomes

- Understand what "persistent" means: old versions remain valid after updates
- Implement a persistent map via cloning as a conceptual foundation
- Use `Rc<_>` sharing to reduce cloning overhead for immutable maps
- Understand the connection to HAMT (hash array mapped tries)
- Know the `im` crate for production persistent collections in Rust

## Rust Application

`src/lib.rs` implements `PersistentMap<K, V>` where `insert` and `remove` clone the underlying `HashMap` before modifying it, returning a new version. The original map is untouched. Tests demonstrate that `v1.insert(k, val)` returns `v2` while `v1` still contains the original data — verifying the persistence property.

The clone-based approach is O(n) per operation. The `im` crate provides HAMT-based persistent maps with O(log32 n) operations and structural sharing, eliminating full copies.

## OCaml Approach

OCaml's `Map.Make` is persistent by default — every operation returns a new map sharing unchanged structure with the original:

```ocaml
module IntMap = Map.Make(Int)

let m0 = IntMap.empty
let m1 = IntMap.add 1 "one" m0
let m2 = IntMap.add 2 "two" m1
(* m1 is unchanged; m0, m1, m2 all exist simultaneously *)
```

This structural sharing is automatic — no explicit cloning required. The GC manages the shared nodes.

## Key Differences

1. **Persistence by default**: OCaml's `Map` is always persistent; Rust requires explicit design to achieve persistence.
2. **Structural sharing**: OCaml's `Map` shares unchanged subtrees (O(log n) allocation per update); Rust's clone approach copies the entire map (O(n)).
3. **`im` crate**: The `im` crate provides HAMT-based persistent collections matching OCaml's `Map` in asymptotic complexity.
4. **Memory management**: OCaml's GC manages shared nodes automatically; Rust's `Rc<_>`-based approaches need careful cycle avoidance.

## Exercises

1. Add a `version_history` field to `PersistentMap` that stores all previous versions as a `Vec<PersistentMap<K, V>>` and implement `undo()`.
2. Use the `im` crate's `HashMap` (which uses HAMT) to implement the same API and compare performance with the clone-based version.
3. Implement a simple key-value store with transactions: `begin`, `commit`, and `rollback` operations using persistent maps.
