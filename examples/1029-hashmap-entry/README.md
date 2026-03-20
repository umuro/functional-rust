📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1029-hashmap-entry)**

---

# 1029-hashmap-entry — HashMap Entry API
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The naive approach to insert-or-update a hash map performs two lookups: one to check if the key exists, one to insert or update. This is O(2 log n) for tree maps and O(2) amortized for hash maps — and worse, it requires the key to be hashed twice. For hot code paths like frequency counting or cache updates, this overhead matters.

Rust's `Entry` API solves this with a single lookup that returns a handle to either the existing entry or a vacancy. Operations on the handle modify the map in-place without a second lookup. This pattern originated in C++'s `std::unordered_map::emplace` and was refined in Rust.

## Learning Outcomes

- Use `entry().or_insert(default)` for insert-if-absent with a constant default
- Use `entry().or_insert_with(|| expr)` to compute the default lazily
- Use `entry().and_modify(|v| *v += 1).or_insert(1)` for increment-or-insert
- Understand why the Entry API avoids double lookups
- Apply the Entry API to frequency counting and cache initialization

## Rust Application

`src/lib.rs` demonstrates `or_insert`, `or_insert_with`, `or_default`, and `and_modify`. The frequency counter `word_frequency` uses `and_modify(|c| *c += 1).or_insert(1)` — a single operation that either increments an existing count or inserts 1. `or_insert_with` delays computation of the default value to avoid running it when the key is already present.

The Entry API is used throughout the standard library and ecosystem: `HashMap::entry` appears in `serde`, `rayon`, and virtually every nontrivial Rust application.

## OCaml Approach

OCaml's immutable `Map` requires two operations:

```ocaml
let increment key m =
  let count = try Map.find key m with Not_found -> 0 in
  Map.add key (count + 1) m
```

`Hashtbl` (mutable) is closer to Rust's HashMap but has no equivalent Entry API — you always do two operations. The `Base.Hashtbl` module adds `find_or_add` as a partial equivalent.

## Key Differences

1. **Single lookup**: Rust's Entry API guarantees one hash computation; OCaml's `Map` and `Hashtbl` require two separate operations.
2. **Lazy default**: `or_insert_with` runs the closure only on miss; OCaml's `Option.value ~default:(compute ())` is eager.
3. **and_modify chain**: Rust's `and_modify(f).or_insert(v)` is a fluent chain unavailable in OCaml's stdlib.
4. **Mutable reference**: `or_insert` returns `&mut V`, allowing direct mutation of the value without re-hashing; OCaml always rebuilds the map.

## Exercises

1. Use the Entry API to implement a `group_by<K, V, F>(items: &[V], key_fn: F) -> HashMap<K, Vec<V>>` function that avoids double lookups.
2. Write a `cache<K, V, F>(cache: &mut HashMap<K, V>, key: K, compute: F) -> &V` function using `entry().or_insert_with(compute)`.
3. Implement `word_pairs(text: &str) -> HashMap<(String, String), usize>` that counts consecutive word pairs using the Entry API.
