📖 **[View on hightechmind.io →](https://hightechmind.io/rust/933-map-functor)**

---

# 933-map-functor — Map / Functor

## Problem Statement

A balanced binary search tree map (dictionary) is one of the most important data structures in functional programming: O(log n) insert, lookup, and delete with ordered iteration. OCaml's `Map.Make` functor instantiates a BST map for any totally-ordered key type — this is a module-level generic. Rust uses `BTreeMap<K, V>` where `K: Ord` serves the same role with generics. The key contrast: OCaml's functor approach allows creating map modules specialized to a key type at the module level; Rust's generics parameterize at the function/struct level. Both produce efficient ordered maps with similar APIs.

## Learning Outcomes

- Use `BTreeMap<K, V>` for ordered key-value mapping (equivalent to OCaml's `Map.Make`)
- Use `HashMap<K, V>` for O(1) average lookup (equivalent to OCaml's `Hashtbl`)
- Implement functional map operations: filter_by_value, map_values
- Understand the BTreeMap/HashMap trade-off (order vs performance)
- Compare Rust's generics with OCaml's `Map.Make` functor for parameterized maps

## Rust Application

`word_lengths` builds a `BTreeMap<String, usize>` — ordered by key, iteration is sorted. `filter_by_value` uses `.iter().filter().collect()` to produce a new filtered map. `map_values` uses `.iter().map((k, v) -> (k.clone(), f(v))).collect()`. `word_lengths_hash` uses `HashMap` — unordered but O(1) average. The iterator-based functional operations mirror OCaml's `Map.filter` and `Map.map`. Both `BTreeMap` and `HashMap` implement the same high-level interface pattern.

## OCaml Approach

`module StringMap = Map.Make(String)` creates a specialized map type. `StringMap.empty`, `StringMap.add key value map`, `StringMap.find key map`, `StringMap.filter pred map`, `StringMap.map f map`. OCaml's `Map.Make` is a functor — it takes a module with `type t` and `compare: t -> t -> int` and returns a full map module. This is a module-level generic, more powerful than Rust's type-level generics (it can specialize the comparison semantics at module creation time).

## Key Differences

1. **Functor vs generics**: OCaml's `Map.Make(String)` creates a specialized module type; Rust's `BTreeMap<String, V>` uses type-level generics — both achieve parameterized maps.
2. **Ordering requirement**: Both require total ordering on keys: Rust uses the `Ord` trait, OCaml's functor requires a `compare` function.
3. **Immutability**: OCaml maps are immutable — `Map.add` returns a new map; Rust `BTreeMap` is mutable — `.insert` modifies in place.
4. **Type-level specialization**: OCaml functors can specialize the comparison logic (case-insensitive keys, etc.) at module creation; Rust requires implementing `Ord` on the key type.

## Exercises

1. Implement `merge_maps<K: Ord + Clone, V: Clone, F: Fn(V, V) -> V>(a: &BTreeMap<K, V>, b: &BTreeMap<K, V>, combine: F) -> BTreeMap<K, V>` that merges duplicate keys using `combine`.
2. Build a `word_frequency_sorted(text: &str) -> Vec<(String, usize)>` that returns words sorted by frequency descending, then alphabetically.
3. Implement a `bimap<K: Ord + Clone, V: Clone, K2: Ord, V2>(map: &BTreeMap<K, V>, key_fn: F, val_fn: G) -> BTreeMap<K2, V2>` that transforms both keys and values.
