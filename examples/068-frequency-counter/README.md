📖 **[View on hightechmind.io →](https://hightechmind.io/rust/068-frequency-counter)**

---

# 068 — Frequency Counter (Map Module)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Counting word frequencies is the "Hello World" of data analysis — used in text indexing (Elasticsearch), word clouds, spell-checking frequency tables, natural language processing (term frequency for TF-IDF), and compression (Huffman coding frequencies). The Rust equivalent of OCaml's `Map.Make` module is `HashMap` (unordered, O(1) ops) or `BTreeMap` (ordered, O(log n) ops).

The `entry().or_insert(0)` API is the idiomatic Rust pattern for updating-or-inserting — a single lookup that either modifies an existing value or inserts a default. This avoids the two-lookup pattern (check if exists, then insert or update).

## Learning Outcomes

- Use `HashMap<String, usize>` as a frequency counter
- Apply the `entry().or_insert(default)` pattern for update-or-insert
- Use `BTreeMap` when sorted output is needed (like OCaml's ordered `Map.Make`)
- Implement word frequency with iterators and `fold` for a functional style
- Understand the difference between `HashMap` (unordered) and `BTreeMap` (sorted)

## Rust Application

`word_freq` splits on whitespace, lowercases, and uses `*freq.entry(word).or_insert(0) += 1` — the idiomatic pattern. `word_freq_functional` uses `fold(HashMap::new(), |mut acc, word| { *acc.entry(word).or_insert(0) += 1; acc })` — functional style with no mutation outside the closure. `word_freq_sorted` uses `BTreeMap` for sorted output, equivalent to OCaml's ordered map.

## OCaml Approach

OCaml's Map: `module StringMap = Map.Make(String)`. Frequency counting: `List.fold_left (fun map word -> let count = try StringMap.find word map with Not_found -> 0 in StringMap.add word (count + 1) map) StringMap.empty words`. Or with `find_opt`: `let count = Option.value (StringMap.find_opt word map) ~default:0`.

## Key Differences

1. **`HashMap` vs `Map.Make`**: Rust's `HashMap` uses hashing (O(1)). OCaml's `Map.Make` uses a balanced BST (O(log n)) — always sorted. Use `BTreeMap` for sorted Rust maps.
2. **`entry` API**: Rust's `entry().or_insert()` avoids double lookup. OCaml's `Map.add key (Option.value (Map.find_opt key m) ~default:0 + 1) m` does two lookups.
3. **Functional vs mutable**: OCaml's `Map.add` returns a new map — fully immutable, structural sharing. Rust's `HashMap::insert` mutates in place. The functional fold style in Rust accumulates a new HashMap per step — less efficient.
4. **Ordering**: OCaml's `Map.Make` iterates in sorted key order. Rust's `HashMap` has no guaranteed iteration order; use `BTreeMap` or `sort_by_key` on the entries.

## Exercises

1. **Top N words**: Write `top_n_words(text: &str, n: usize) -> Vec<(String, usize)>` that returns the n most frequent words. Sort by frequency descending.
2. **Co-occurrence**: Count how often pairs of adjacent words appear together in a text, building a `HashMap<(String, String), usize>`.
3. **Histogram**: Write `print_histogram(freq: &HashMap<String, usize>)` that prints each word with a bar of `*` characters proportional to its frequency (max bar length = 40 chars).
