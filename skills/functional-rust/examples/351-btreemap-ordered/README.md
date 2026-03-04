# 351: BTreeMap — Ordered Key-Value Storage

**Difficulty:** 2  **Level:** Intermediate

A sorted map where iteration always walks keys in ascending order and range queries are O(log n).

## The Problem This Solves

You're building a leaderboard, a time-series store, or a word frequency report. You reach for `HashMap` first — fast lookups, easy to use. But then you need to print entries alphabetically, or find all scores between 100 and 200, or get the ten most recent timestamps. With `HashMap` you're stuck sorting on every output, scanning the whole map for every range query.

`BTreeMap` solves this class of problem. Because it stores keys in a B-tree (balanced, sorted), iteration is always in ascending order at zero extra cost. Range queries like `.range(start..=end)` are O(log n) to reach the start node, then O(k) to walk the k matching entries — exactly what a sorted structure should give you.

The tradeoff is that individual operations cost O(log n) instead of HashMap's O(1) average. For most real-world sizes (thousands to millions of keys) the difference is negligible. Pick `BTreeMap` when order or range queries matter; pick `HashMap` when only raw lookup speed matters and you don't care about order.

## The Intuition

Think of `BTreeMap` as a sorted dictionary. In Python, `dict` is insertion-ordered but not sorted by key — you'd use `sorted(d.items())` to iterate in key order. `BTreeMap` is always sorted, no sorting step needed.

The key tradeoff: **order costs a constant factor**. `BTreeMap` ops are O(log n), `HashMap` ops are O(1) average. But `BTreeMap` has much better cache locality than a linked tree structure, and the log factor is tiny (log₂(1,000,000) ≈ 20).

## How It Works in Rust

```rust
use std::collections::BTreeMap;

// Build a word-count map — iteration will be alphabetical
let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
counts.insert("zebra", 3);
counts.insert("apple", 7);
counts.insert("mango", 2);

// Always iterates in sorted key order: apple, mango, zebra
for (word, count) in &counts {
    println!("{word}: {count}");
}

// Range query: words from "apple" to "mango" inclusive
for (word, count) in counts.range("apple"..="mango") {
    println!("in range: {word} → {count}");
}

// Min and max keys in O(log n)
let first = counts.iter().next();         // smallest key
let last  = counts.iter().next_back();    // largest key
```

The `.range()` method accepts any `RangeBounds<K>`, so `"a".."m"`, `"a"..="m"`, `"a"..`, and `..="z"` all work.

## What This Unlocks

- **Time-series data**: store events by timestamp and query any time window with `.range(t1..=t2)` — no full scans.
- **Leaderboards and rankings**: sorted iteration means printing top-N is just `.iter().rev().take(N)`.
- **Auto-sorted indexes**: maintain a sorted secondary index alongside a primary store without explicit sort calls.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Ordered map | `Map` module (balanced BST) | `BTreeMap<K, V>` |
| Range query | `Map.find_first` / manual | `.range(lo..=hi)` |
| Min/max key | `Map.min_binding` / `Map.max_binding` | `.iter().next()` / `.iter().next_back()` |
| Lookup complexity | O(log n) | O(log n) |
| Iteration order | ascending by key | ascending by key |
| Unordered equivalent | `Hashtbl` | `HashMap<K, V>` (O(1) avg) |
