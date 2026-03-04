# 358: Insertion-Order Maps (IndexMap Pattern)

**Difficulty:** 3  **Level:** Advanced

Insertion-order-preserving HashMap — iterate keys in the order you inserted them.

## The Problem This Solves

The standard `HashMap` gives you O(1) lookups but no iteration order guarantee. Sometimes order matters: you're building a JSON serializer that must emit fields in declaration order, a CLI that shows options in the sequence a user added them, or a configuration system where the last-write-wins but you still want to display settings in original order.

Python solved this with `OrderedDict` (and made it the default `dict` behavior in 3.7+). Rust's `HashMap` never will — randomized order is a feature, not a bug, preventing hash-flooding attacks and encouraging you not to depend on it.

The `indexmap` crate fills this gap. It gives you a hash map with `O(1)` insert, lookup, and remove, plus `O(n)` iteration in insertion order. Think of it as `HashMap` with a sidecar `Vec` that tracks key order.

## The Intuition

An `IndexMap` is backed by two structures: a hash table that maps keys to indices, and a dense `Vec` of `(key, value)` pairs in insertion order. Lookup goes: hash the key → find the index → jump into the Vec. Iteration just walks the Vec. The tradeoff: slightly more memory than `HashMap`, and remove is `O(n)` unless you use `swap_remove` (which breaks insertion order but stays `O(1)`).

If you need both sorted order *and* fast lookup, use `BTreeMap`. If you need insertion order *and* fast lookup, use `IndexMap`. If you only need fast lookup, `HashMap` is fine.

## How It Works in Rust

```rust
use indexmap::IndexMap;

let mut map = IndexMap::new();
map.insert("banana", 3);
map.insert("apple", 5);
map.insert("cherry", 1);

// Iterates in insertion order: banana, apple, cherry
for (key, val) in &map {
    println!("{key}: {val}");
}

// O(1) lookup still works
assert_eq!(map["apple"], 5);

// Index-based access (unique to IndexMap)
assert_eq!(map.get_index(0), Some((&"banana", &3)));
```

Add to `Cargo.toml`: `indexmap = "2"`

## What This Unlocks

- **Deterministic serialization** — JSON/TOML output matches source order, making diffs readable.
- **Index-based access** — `get_index(n)` lets you treat the map like a sorted `Vec` of pairs.
- **Sorted variant** — `IndexMap::with_hasher` + manual sort, or use `IndexSet` for sets.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Ordered map | `Map` from stdlib (always sorted by key) | `IndexMap` (insertion order) or `BTreeMap` (sorted) |
| Default map | `Hashtbl` (unordered) | `HashMap` (unordered) |
| Iteration order | Determined by comparison function | Insertion order (`IndexMap`) or key order (`BTreeMap`) |
| Index access | Not built-in | `map.get_index(n)` on `IndexMap` |
