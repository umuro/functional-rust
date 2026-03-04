# 352: BTreeSet — Sorted Unique Values

**Difficulty:** 2  **Level:** Intermediate

A sorted set of unique values with O(log n) operations and built-in set algebra.

## The Problem This Solves

You have a collection where duplicates should be silently ignored and you need to iterate in sorted order or find all values in a range. A `Vec` requires manual deduplication and a sort call before every output. A `HashSet` deduplicates automatically but gives you no ordering — iteration order is arbitrary.

`BTreeSet` is the answer when you need **both** deduplication and sorted order. Inserting a value that already exists is a no-op. Iteration is always ascending. Range queries work with `.range()` just like `BTreeMap`. Set algebra — union, intersection, difference, symmetric difference — is built in as iterator adapters.

The second use case is sorted membership testing. When you need to ask "is this value between the 10th and 90th percentile?", `BTreeSet` lets you walk the ordered structure directly rather than sorting a `Vec` on every query.

## The Intuition

Python's `set` is unordered. If you wanted a sorted set, you'd use `sorted(my_set)` when iterating — but that's O(n log n) every time. `BTreeSet` is always sorted, so ordered operations are free after insertion.

Think of it as a `BTreeMap<T, ()>` — a B-tree where values are just the keys themselves. The tradeoff is identical: O(log n) per op instead of HashSet's O(1) average, in exchange for permanent sorted order.

## How It Works in Rust

```rust
use std::collections::BTreeSet;

let mut set: BTreeSet<i32> = BTreeSet::new();
set.insert(30);
set.insert(10);
set.insert(20);
set.insert(10); // duplicate — ignored silently

// Iteration always ascending: 10, 20, 30
for v in &set {
    println!("{v}");
}

// Range query: values between 10 and 25 inclusive
for v in set.range(10..=25) {
    println!("in range: {v}");
}

// Set algebra — built-in iterator adapters
let a: BTreeSet<i32> = [1, 2, 3, 4].into_iter().collect();
let b: BTreeSet<i32> = [3, 4, 5, 6].into_iter().collect();

let union:        Vec<_> = a.union(&b).collect();        // [1,2,3,4,5,6]
let intersection: Vec<_> = a.intersection(&b).collect(); // [3,4]
let difference:   Vec<_> = a.difference(&b).collect();   // [1,2]

// Membership test
println!("{}", set.contains(&20)); // true
```

## What This Unlocks

- **Deduplication with order**: collect a stream of events, deduplicate, and iterate chronologically or alphabetically without a separate sort pass.
- **Range membership queries**: "which tags fall between 'networking' and 'rust'?" — one `.range()` call, O(log n) to start.
- **Set algebra pipelines**: compute union/intersection of two filtered result sets lazily, then collect once.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sorted set | `Set` module (balanced BST) | `BTreeSet<T>` |
| Unordered set | `Hashtbl` (manual) | `HashSet<T>` |
| Set union | `Set.union` | `.union(&other)` (iterator) |
| Set intersection | `Set.inter` | `.intersection(&other)` |
| Set difference | `Set.diff` | `.difference(&other)` |
| Range query | `Set.find_first` + manual | `.range(lo..=hi)` |
| Duplicate insert | compile-time impossible | silent no-op |
