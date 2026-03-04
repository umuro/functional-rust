# 068: Frequency Counter

**Difficulty:** ⭐  **Level:** Foundations

Count how many times each word (or any value) appears — using `HashMap` and the `entry` API.

## The Problem This Solves

You have a list of items and you want to know how often each one appears. Word counts in a document. Event types in a log file. HTTP status codes in a request trace. Every programmer needs this pattern constantly.

The painful way: for each unique item, scan the whole list and count. O(n²). The dictionary approach is O(n) and every modern language has it. Python: `collections.Counter`. JavaScript: build an object with `{}`. Java: `HashMap<String, Integer>` with a null-check.

Rust's `HashMap` is the same idea, but the update pattern is more explicit — and more expressive. The `entry()` API is idiomatic Rust: "get the entry for this key, inserting a default if it's absent, then give me a mutable reference to the value."

## The Intuition

The key insight is `entry().or_insert(0)`. It means: "look up this key; if it's not there, insert 0; either way, give me a mutable reference to the value." Then you `+= 1` through that reference.

In Python: `counts[word] = counts.get(word, 0) + 1`. In JavaScript: `counts[word] = (counts[word] || 0) + 1`. Rust's version is more structured but expresses the same idea, and it avoids two hash lookups (one to check, one to insert) by doing it in one step.

## How It Works in Rust

```rust
use std::collections::HashMap;

pub fn word_freq(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let lower = word.to_lowercase();
        // entry(): find or create the slot for this key
        // or_insert(0): if absent, insert 0 and return &mut 0
        *freq.entry(lower).or_insert(0) += 1;
    }
    freq
}
```

Functional style with `fold`:

```rust
pub fn word_freq_functional(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
```

Use `BTreeMap` when you need sorted output (like OCaml's `Map`, which is always ordered):

```rust
use std::collections::BTreeMap;

pub fn word_freq_sorted(text: &str) -> BTreeMap<String, usize> {
    let mut freq = BTreeMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    freq
}
```

## What This Unlocks

- **Log analysis** — count error codes, HTTP methods, user agents from raw log lines
- **Histogram building** — any "distribution of X" problem is a frequency counter
- **Duplicate detection** — after counting, filter for `count > 1` to find duplicates

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map type | `Map.Make(String)` (ordered, immutable) | `HashMap` (unordered) or `BTreeMap` (ordered) |
| Update pattern | `find` + `add` (two steps, replace whole map) | `entry().or_insert()` (one step, mutable ref) |
| Missing key | `try … with Not_found -> 0` | `.or_insert(0)` handles it inline |
| Ordered output | Default (always ordered) | Use `BTreeMap` explicitly |
