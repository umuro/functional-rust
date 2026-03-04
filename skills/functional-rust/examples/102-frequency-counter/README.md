# 102: Frequency Counter

**Difficulty:** 1  **Level:** Foundations

Count how often each word appears in text — the idiomatic HashMap pattern every Rust developer reaches for.

## The Problem This Solves

Word frequency is the "hello world" of data analysis. Log parsers count error codes. Analytics systems count page views per URL. Games track how often each item was collected. Any time you need to count occurrences of categorized data, you're writing a frequency counter.

The naive approach — scan the map, update the entry, put it back — requires two lookups. Rust's `entry` API handles insert-or-update in a single operation, cleanly and efficiently.

## The Intuition

The key pattern is `*freq.entry(word).or_insert(0) += 1`:

- `entry(word)` — find the slot for this word (or prepare to create one)
- `.or_insert(0)` — if the slot is empty, put `0` in it; returns a `&mut usize` either way
- `+= 1` — increment whatever is there

This is "get or create then update" in one expression. No double lookup, no awkward `if contains_key` checks.

`HashMap` gives O(1) average lookups with random order. `BTreeMap` gives O(log n) lookups with sorted keys — useful when you want alphabetical output, and closer to OCaml's `Map.Make(String)` semantics.

## How It Works in Rust

```rust
use std::collections::HashMap;

pub fn word_freq(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();     // normalize case
        *freq.entry(w).or_insert(0) += 1;
    }
    freq
}
```

Functional style with `fold` and `BTreeMap` (sorted keys, mirrors OCaml):

```rust
use std::collections::BTreeMap;

pub fn word_freq_btree(text: &str) -> BTreeMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(BTreeMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}
```

Using the result:

```rust
let freq = word_freq("the cat sat on the mat the cat");
println!("{}", freq["the"]);   // 3
println!("{}", freq["cat"]);   // 2
// freq.get("dog") → None (safe, no panic)
```

## What This Unlocks

- **Log analysis** — count error codes, user actions, status codes
- **Text statistics** — term frequency for search indexing, TF-IDF
- **Grouping and aggregation** — any "group by and count" operation

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map type | `Map.Make(String)` functor | `HashMap<String, V>` or `BTreeMap` |
| Mutability | Immutable — every update creates a new map | Mutable by default |
| Insert-or-update | Pattern match + re-insert | `entry().or_insert()` — single lookup |
| Ordering | Sorted (balanced tree) | `HashMap` unordered; `BTreeMap` sorted |
| Missing key | Raises exception | `.get()` returns `Option` |
