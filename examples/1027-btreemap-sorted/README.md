📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1027-btreemap-sorted)**

---

# 1027-btreemap-sorted — BTreeMap: Sorted Key Iteration

## Problem Statement

Hash maps provide O(1) average lookup but iterate in arbitrary order, making them unsuitable for tasks that require processing keys in sorted order — leaderboards, time-series data, range queries, and sorted output. B-trees, invented by Bayer and McCreight in 1972 for database indexes, maintain sorted order while keeping operations O(log n). Rust's `BTreeMap` is a cache-friendly B-tree optimized for modern hardware.

This is the data structure behind database indexes, `std::map` in C++, and OCaml's `Map.Make` module.

## Learning Outcomes

- Understand when to choose `BTreeMap` over `HashMap`
- Iterate over a `BTreeMap` in guaranteed sorted key order
- Use the `range` method for efficient range queries without scanning all entries
- Use `first_key_value` and `last_key_value` for O(log n) min/max access
- Implement a sorted frequency counter

## Rust Application

`src/lib.rs` demonstrates key `BTreeMap` operations. `sorted_iteration` shows that inserting in arbitrary order and then iterating always yields keys sorted. `range_query` uses `.range(2..=4)` for an inclusive range scan — only O(k + log n) where k is the number of results, not O(n). `min_max` uses `first_key_value()` and `last_key_value()` for O(log n) extremes.

Range queries are the killer feature of `BTreeMap`: finding all events between two timestamps, all customers in a zip code range, or all prices between two values.

## OCaml Approach

OCaml's `Map.Make(Ord)` is always a sorted tree map. It has no hash map variant in the standard library, so range queries are natural:

```ocaml
module IntMap = Map.Make(Int)

let range_query m lo hi =
  IntMap.to_seq_from lo m
  |> Seq.take_while (fun (k, _) -> k <= hi)
  |> List.of_seq
```

`Map.to_seq_from` starts iteration at a given key, enabling efficient range scans without the explicit `range` API.

## Key Differences

1. **Default sort**: OCaml's `Map.Make` is always sorted by key — there is no unsorted alternative; Rust has both `HashMap` (unsorted) and `BTreeMap` (sorted).
2. **Range API**: Rust has an explicit `.range(lo..=hi)` method; OCaml uses `to_seq_from` plus `take_while` for the equivalent.
3. **Cache efficiency**: Rust's B-tree stores multiple keys per node (cache-friendly); OCaml's balanced binary tree stores one key per node.
4. **Persistence**: OCaml's `Map` is persistent (update returns a new version sharing structure); Rust's `BTreeMap` is mutable.

## Exercises

1. Build a time-series store using `BTreeMap<u64, f64>` (timestamp -> value) and write a `window_average(start: u64, end: u64)` function using `range`.
2. Implement a `ranked_top_k(map: &BTreeMap<String, usize>, k: usize) -> Vec<(&str, usize)>` that returns the k most frequent items in alphabetical order.
3. Write a function that splits a `BTreeMap` at a given key into two maps using `split_off`.
