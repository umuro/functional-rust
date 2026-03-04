# 359: Multimap Pattern

**Difficulty:** 2  **Level:** Intermediate

Map each key to multiple values using `HashMap<K, Vec<V>>` — the idiomatic Rust multimap.

## The Problem This Solves

Sometimes one key legitimately maps to many values. A tag index maps each tag to many posts. An inverted search index maps each word to many document IDs. An event system maps each event type to many listeners. You could reach for a dedicated `MultiMap` type, but Rust doesn't have one in the standard library — and you don't need one.

The pattern `HashMap<K, Vec<V>>` covers most cases directly. It's simple, obvious, and composes cleanly with iterators. When you need sorted keys or sorted values, swap in `BTreeMap<K, BTreeSet<V>>`. When you need uniqueness within a key's values, use `HashSet<V>` instead of `Vec<V>`.

The main friction point is the "insert if key not present, then push" dance — which `entry().or_insert_with(Vec::new).push(v)` handles elegantly in one line.

## The Intuition

There's no magic here. A multimap is just a map where each value is a collection. The `Entry` API is the key insight: instead of checking "does this key exist?", you grab a handle to the entry (occupied or vacant) and act on it atomically. This avoids a double-lookup and keeps the borrow checker happy.

For sorted multimaps, `BTreeMap<K, BTreeSet<V>>` gives you keys in sorted order and deduplicated, sorted values per key — at the cost of `O(log n)` operations instead of `O(1)`.

## How It Works in Rust

```rust
use std::collections::HashMap;

let mut index: HashMap<&str, Vec<u32>> = HashMap::new();

// The idiomatic insert pattern
let entries = [("rust", 1), ("rust", 2), ("ocaml", 3), ("rust", 4)];
for (tag, id) in entries {
    index.entry(tag).or_insert_with(Vec::new).push(id);
}

// Lookup
if let Some(ids) = index.get("rust") {
    println!("rust posts: {:?}", ids); // [1, 2, 4]
}

// Iterate all pairs
for (tag, ids) in &index {
    println!("{tag}: {} entries", ids.len());
}
```

For a sorted, deduplicated variant:
```rust
use std::collections::{BTreeMap, BTreeSet};
let mut sorted: BTreeMap<&str, BTreeSet<u32>> = BTreeMap::new();
sorted.entry("rust").or_default().insert(1);
```

## What This Unlocks

- **Inverted indexes** — word → document IDs, the foundation of every search engine.
- **Event dispatching** — event type → list of handlers, without a framework.
- **Grouping** — equivalent to SQL `GROUP BY`, using `entry().or_default().push()`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Multimap | No stdlib type; use `Hashtbl` with list values | `HashMap<K, Vec<V>>` with `entry().or_default()` |
| Insert-or-append | Manual check + add | `.entry(k).or_insert_with(Vec::new).push(v)` |
| Sorted multimap | `Map` with `Set` values | `BTreeMap<K, BTreeSet<V>>` |
| Unique values per key | `Hashtbl` with list dedup | `HashMap<K, HashSet<V>>` |
