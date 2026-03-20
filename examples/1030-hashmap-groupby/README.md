📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1030-hashmap-groupby)**

---

# 1030-hashmap-groupby — Group By with HashMap
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Grouping items by a key — SQL's `GROUP BY`, Python's `itertools.groupby`, Haskell's `Data.Map.fromListWith` — is one of the most frequent data processing operations. You want to transform a flat list into a map from key to list of matching elements: group customers by country, group log events by severity, group words by length.

The canonical Rust implementation uses `HashMap<K, Vec<V>>` with the Entry API's `or_default()` helper. This avoids double lookups and is idiomatic across the Rust ecosystem.

## Learning Outcomes

- Implement group-by using `HashMap<K, Vec<V>>` and the Entry API
- Use `or_default()` for ergonomic initialization of empty `Vec`s
- Write a generic `group_by` function parameterized over key type and key function
- Compare the push-based approach to `Iterator::partition` for binary grouping
- Understand when `BTreeMap` is better than `HashMap` for grouped output

## Rust Application

`src/lib.rs` shows grouping by first character (`group_by_first_letter`) and by parity (`group_by_parity`). Both use `entry(key).or_default().push(value)` — the Entry API ensures one hash lookup per item. The generic `group_by<T, K, F>` function abstracts the pattern for any key type and extractor function.

This pattern is used throughout the ecosystem: in `diesel` for query results, in `rayon` for parallel aggregation, and in data pipeline crates like `polars`.

## OCaml Approach

OCaml's functional approach uses `List.fold_left` with a persistent map:

```ocaml
module StringMap = Map.Make(String)

let group_by key_fn items =
  List.fold_left (fun acc item ->
    let k = key_fn item in
    let current = try StringMap.find k acc with Not_found -> [] in
    StringMap.add k (item :: current) acc
  ) StringMap.empty items
```

Each update creates a new map version. The `Base.Map.add_multi` function provides a one-liner for the same pattern.

## Key Differences

1. **Mutability**: Rust's HashMap mutates in place via `push`; OCaml's Map creates new versions on each insert.
2. **or_default**: Rust's `or_default()` inserts an empty `Vec<_>` on first access; OCaml requires explicit `Not_found` handling.
3. **Sorted vs unsorted**: Rust's `HashMap` groups in arbitrary order; use `BTreeMap` for sorted group keys, matching OCaml's `Map.Make` behavior.
4. **Generic API**: Rust's generic `group_by` with a closure is natural; OCaml uses first-class functions via `List.fold_left`.

## Exercises

1. Write `group_and_count<T: Hash + Eq>(items: &[T]) -> HashMap<&T, usize>` that counts occurrences without storing the items.
2. Implement a `bucket_by_range(numbers: &[i32], bucket_size: i32) -> BTreeMap<i32, Vec<i32>>` that groups numbers into buckets of fixed width.
3. Write a `invert_map<K, V>(map: HashMap<K, V>) -> HashMap<V, Vec<K>>` that inverts a map, grouping keys that share a value.
