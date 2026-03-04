# 073: Word Count with Map

**Difficulty:** 3  **Level:** Intermediate

Build a word-frequency map from text — tokenize, normalize, and fold into a `HashMap` in one pipeline.

## The Problem This Solves

Word frequency is a foundational text-processing task: search engines, spell checkers, content analysis, and compression algorithms all start here. You need to split text into words, normalize them (lowercase, strip punctuation), and count occurrences.

Doing this naively requires multiple loops and mutable bookkeeping. The functional approach chains tokenization and accumulation into a clean pipeline — and choosing between `HashMap` (fast) and `BTreeMap` (sorted) is just a type swap.

## The Intuition

The OCaml pattern uses `StringMap.add` with functional update: each word addition returns a new map. Rust's `HashMap` uses `.entry().or_insert(0)` to mutate in place — more efficient but slightly more verbose.

The `entry` API is the idiomatic Rust idiom for "insert if missing, then update." It's one atomic operation: find the slot, create it with a default if empty, return a mutable reference to the value.

Python equivalent: `counts[word] = counts.get(word, 0) + 1`. Rust: `*map.entry(word).or_insert(0) += 1`.

## How It Works in Rust

```rust
use std::collections::HashMap;

// Tokenize: lowercase everything, extract alphanumeric words
pub fn tokenize(s: &str) -> Vec<String> {
    let s = s.to_lowercase();
    let mut words = Vec::new();
    let mut buf = String::new();

    for c in s.chars() {
        if c.is_alphanumeric() {
            buf.push(c);
        } else if !buf.is_empty() {
            words.push(buf.clone());
            buf.clear();
        }
    }
    if !buf.is_empty() { words.push(buf); } // flush last word
    words
}

// Imperative style: entry API for in-place mutation
pub fn word_count(sentence: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in tokenize(sentence) {
        *map.entry(word).or_insert(0) += 1;
        //   ^^^^ find or create slot
        //                ^^^^^^^^^ default value if new
        //                           ^^^ dereference to increment
    }
    map
}

// Functional style: fold over tokens
pub fn word_count_fold(sentence: &str) -> HashMap<String, usize> {
    tokenize(sentence).into_iter().fold(
        HashMap::new(),
        |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        },
    )
}

// Top N words by frequency — sort by count descending
pub fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut pairs: Vec<_> = map.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs.truncate(n);
    pairs
}
```

Use `BTreeMap` instead of `HashMap` if you need deterministic ordering (e.g., for tests or sorted output). Same API, O(log n) per operation instead of O(1) amortized.

## What This Unlocks

- **Text analysis**: frequency analysis, n-gram counts, top-k words.
- **Histogram building**: any key-counting pattern uses the same `entry().or_insert(0) += 1` idiom.
- **Caching / memoization**: `entry().or_insert_with(|| expensive_computation())` for lazy population.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map type | `Map.Make(String)` (immutable) | `HashMap<String, V>` (mutable) |
| Insert / update | `Map.add k v m` (returns new map) | `map.entry(k).or_insert(v)` (mutates) |
| Complexity | O(log n) tree | O(1) amortized hash |
| Sorted output | Natural (BST) | `BTreeMap` or sort manually |
| Fold to map | `List.fold_left` | `.fold(HashMap::new(), ...)` |
