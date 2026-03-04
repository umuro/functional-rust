# 356: Advanced HashMap Patterns

**Difficulty:** 2  **Level:** Intermediate

Grouping, counting, caching, and inverting — the idioms that turn HashMap from "key → value" into a data processing workhorse.

## The Problem This Solves

You know how to insert and look up values in a `HashMap`. But real programs need more: group a list of items by category, count word frequencies, build an inverted index from value back to key, or cache expensive computation results. Done naively, these all involve awkward "does this key exist?" checks followed by either insert or update.

Rust's `HashMap` has a richer API than most developers discover. The entry API (`entry().or_insert()`) handles insert-or-update in a single lookup. The `or_insert_with` and `and_modify` variants let you run closures only when needed. Understanding these patterns turns what would be 5-line conditional blocks into single expressive statements.

A second issue is performance. Rust's default hasher (SipHash) is DoS-resistant but not the fastest for integer keys or short strings. When you need raw throughput and the keys come from a trusted source, swapping the hasher with a type alias costs one line of code.

## The Intuition

Python's `collections.Counter` and `collections.defaultdict` are the closest equivalent. `Counter` auto-initializes counts to zero; `defaultdict` auto-initializes any missing key with a factory. Rust's entry API gives you both behaviors on a plain `HashMap` — the `or_insert(0)` idiom is the Counter pattern, and `or_insert_with(Vec::new)` is the defaultdict pattern.

The key mental shift: stop thinking of `.get()` + `.insert()` as the fundamental operations. The entry API is the idiomatic foundation — it enters the map slot once and lets you read, modify, or initialize it in one coherent operation.

## How It Works in Rust

```rust
use std::collections::HashMap;

// Pattern 1: Word frequency count (the Counter pattern)
let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
let mut freq: HashMap<&str, u32> = HashMap::new();
for word in &words {
    *freq.entry(word).or_insert(0) += 1; // insert 0 if missing, then increment
}
// freq: {"apple": 3, "banana": 2, "cherry": 1}

// Pattern 2: Group items by key (defaultdict(list) pattern)
let items = vec![("fruit", "apple"), ("veggie", "carrot"), ("fruit", "banana")];
let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();
for (category, item) in &items {
    groups.entry(category).or_insert_with(Vec::new).push(item);
}
// groups: {"fruit": ["apple", "banana"], "veggie": ["carrot"]}

// Pattern 3: Conditional update (and_modify chained with or_insert)
let mut scores: HashMap<&str, i32> = HashMap::new();
scores.entry("alice")
    .and_modify(|s| *s += 10) // update if present
    .or_insert(10);            // insert if absent

// Pattern 4: Invert a map (value → key)
let original: HashMap<&str, u32> = [("alice", 1), ("bob", 2)].into_iter().collect();
let inverted: HashMap<u32, &str> = original.iter().map(|(&k, &v)| (v, k)).collect();

// Pattern 5: Memoization / lazy caching
fn expensive(n: u64) -> u64 { n * n } // placeholder
let mut cache: HashMap<u64, u64> = HashMap::new();
let result = *cache.entry(42).or_insert_with(|| expensive(42));
```

## What This Unlocks

- **Data aggregation pipelines**: group-by, count, sum — all with one pass and zero sorting.
- **Inverted indexes**: build word → list of document IDs from a corpus in O(n) with the `or_insert_with(Vec::new)` pattern.
- **Memoization**: `entry().or_insert_with(|| compute(key))` implements lazy single-computation caching with no unsafe code.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hash map | `Hashtbl` | `HashMap<K, V>` |
| Auto-init missing key | `find` + `add` manually | `.entry().or_insert(default)` |
| Group-by | manual | `.entry().or_insert_with(Vec::new).push(v)` |
| Conditional update | `find` + `replace` | `.entry().and_modify(f).or_insert(v)` |
| Iteration | `Hashtbl.iter` | `.iter()` / `.keys()` / `.values()` |
| Custom hasher | N/A | `HashMap<K,V,BuildHasher>` type param |
