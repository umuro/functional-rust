📖 **[View on hightechmind.io →](https://hightechmind.io/rust/102-frequency-counter)**

---

# 102-frequency-counter — Frequency Counter
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Counting the frequency of items in a collection is one of the most common data processing operations: word frequency in text analysis, character histograms in compression, event counts in telemetry. The pattern requires a map from item to count that inserts a zero for new keys and increments existing ones atomically.

Rust's `Entry` API makes this pattern concise and allocation-efficient. OCaml's `Map.Make` module provides an immutable tree-based equivalent with different trade-offs.

## Learning Outcomes

- Use `HashMap::entry().or_insert(0)` to count occurrences idiomatically
- Use `BTreeMap` when sorted iteration order is required
- Compare the mutable HashMap approach to a functional fold approach
- Understand the performance implications of hash maps versus tree maps
- Build reusable frequency-counting utilities

## Rust Application

`src/lib.rs` provides three implementations. `word_freq_hashmap` uses `HashMap` with the entry API for O(1) average insert and lookup. `word_freq_btree` uses a `BTreeMap` and `fold` in functional style — iteration over the result will yield keys in alphabetical order, matching OCaml's `Map.Make(String)` behavior. `word_freq_functional` shows the purely functional approach using `fold` without mutation inside the closure.

`BTreeMap` is the right choice when you need sorted output (e.g., generating reports); `HashMap` is better for raw throughput.

## OCaml Approach

OCaml's standard `Map.Make(String)` creates an immutable, sorted map:

```ocaml
module StringMap = Map.Make(String)

let word_freq text =
  String.split_on_char ' ' text
  |> List.fold_left (fun acc w ->
    let count = try StringMap.find w acc with Not_found -> 0 in
    StringMap.add w (count + 1) acc
  ) StringMap.empty
```

Every update produces a new map via structural sharing, so intermediate maps are not copied entirely. The Hashtbl module provides mutable hash tables analogous to Rust's `HashMap`.

## Key Differences

1. **Mutability model**: Rust's `HashMap` is imperatively mutated via the entry API; OCaml's `Map` produces new persistent versions on each update.
2. **Sorted order**: OCaml's `Map.Make` always iterates in key order; Rust's `HashMap` has random order and requires `BTreeMap` for sorted iteration.
3. **Entry API**: Rust's `entry().or_insert(0)` is a single lookup; OCaml's `find` + `add` is conceptually two operations but O(log n) per structural sharing update.
4. **Memory**: Rust's `HashMap` is compact and cache-friendly; OCaml's tree map uses more pointer-chasing but enables sharing between versions.

## Exercises

1. Extend `word_freq_hashmap` to normalize words by stripping punctuation before counting.
2. Write a `top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)>` function that returns the n most frequent words in descending order.
3. Implement a `merge_frequencies` function that combines two frequency maps, summing counts for keys that appear in both.
