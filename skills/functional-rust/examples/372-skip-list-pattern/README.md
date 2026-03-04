# 372: Skip List Concept in Rust

**Difficulty:** 5  **Level:** Master

Probabilistic sorted data structure with O(log n) operations and better cache behavior than tree-based alternatives.

## The Problem This Solves

`BTreeMap` gives you sorted keys, `O(log n)` insert and lookup, and range queries. So why would you want a skip list? Cache performance. A `BTreeMap` node contains 11–16 keys and requires pointer chasing through arbitrary heap addresses. A skip list is a layered linked list — sequential memory access patterns make it much friendlier to CPU caches on modern hardware.

Skip lists also support lock-free concurrent implementations more naturally than trees, which is why they appear in databases (LevelDB, RocksDB), language runtimes (Java's `ConcurrentSkipListMap`), and high-throughput data stores. The `crossbeam-skiplist` crate provides a production-quality concurrent skip list for Rust.

For purely single-threaded code, `BTreeMap` is usually the right choice. Skip lists become interesting when you need concurrent sorted access or are tuning cache behavior at scale.

## The Intuition

A skip list is a sorted linked list with multiple layers. The bottom layer is a regular sorted linked list. Each higher layer is a "fast lane" — a sparser version of the layer below, skipping over most elements. To search, you start at the top layer and walk right as long as the next element is ≤ your target, then drop down a layer and repeat. You find your target by binary-search-like jumps without a tree structure.

Each element is promoted to higher layers with probability `p` (typically 0.5). This randomness gives probabilistic balance: no rebalancing needed, unlike red-black trees or AVL trees. The expected height of the tallest tower is `O(log n)`.

## How It Works in Rust

The `crossbeam-skiplist` crate provides a production concurrent skip list:

```rust
use crossbeam_skiplist::SkipMap;

let map = SkipMap::new();

map.insert(3, "three");
map.insert(1, "one");
map.insert(4, "four");
map.insert(1, "one-again"); // updates value for key 1

// Sorted iteration
for entry in map.iter() {
    println!("{}: {}", entry.key(), entry.value());
}
// Output: 1: one-again, 3: three, 4: four

// Range query
for entry in map.range(1..=3) {
    println!("{}", entry.key());
}
```

For a conceptual single-threaded implementation, each node carries a `Vec<Option<*mut Node>>` of forward pointers — one per level the node participates in.

## What This Unlocks

- **Concurrent sorted maps** — `SkipMap` supports multiple threads without a global lock.
- **Database internals** — LSM tree memtables (LevelDB, RocksDB) use skip lists for the in-memory sorted buffer.
- **Cache-friendly sorted access** — sequential memory access patterns versus tree pointer chasing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sorted map | `Map` (balanced BST, functional, persistent) | `BTreeMap` (B-tree) or `SkipMap` (skip list) |
| Concurrent sorted map | No stdlib equivalent | `crossbeam_skiplist::SkipMap` (lock-free) |
| Rebalancing | Structural (persistent trees) | Not needed (probabilistic balance) |
| Cache behavior | Poor (pointer chasing) | Better (more sequential layout in skip lists) |
