📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1041-multimap)**

---

# 1041-multimap — Multimap

## Problem Statement

A multimap (or multi-dictionary) maps each key to multiple values. Real-world examples: a person can have multiple phone numbers, a web server can handle multiple routes for the same path prefix, a product can belong to multiple categories. Standard hash maps do not support this — inserting the same key twice overwrites the old value.

The canonical Rust implementation uses `HashMap<K, Vec<V>>` as the underlying storage. This example wraps it in a `MultiMap<K, V>` struct providing ergonomic insert, get (returning a slice), and count operations.

## Learning Outcomes

- Implement `MultiMap<K, V>` using `HashMap<K, Vec<V>>`
- Use the Entry API for efficient multi-value insert
- Provide `get(&key) -> &[V]` returning a slice for zero-copy access
- Implement removal of a specific value from a key's value list
- Understand when `multimap` crate variants are preferred over custom implementations

## Rust Application

`src/lib.rs` defines `MultiMap<K, V>` with `insert`, `get` (returning `&[V]`), `remove_key`, `count_values`, and `total_values`. `insert` uses `entry(key).or_default().push(value)` — the standard pattern. `get` returns `self.inner.get(key).map_or(&[], |v| v.as_slice())` — an empty slice for missing keys avoids `Option` unwrapping at call sites.

The multimap pattern appears in HTTP header parsing (multiple `Set-Cookie` headers), DNS records (multiple A records per hostname), and graph adjacency representations.

## OCaml Approach

OCaml's functional multimap uses a persistent map of lists:

```ocaml
module StringMultiMap = Map.Make(String)

let insert key value m =
  let values = try StringMultiMap.find key m with Not_found -> [] in
  StringMultiMap.add key (value :: values) m

let get key m =
  try StringMultiMap.find key m with Not_found -> []
```

Each insert creates a new map version. The `Base.Map.add_multi` function provides this in one call.

## Key Differences

1. **Persistence**: OCaml's map-of-lists is persistent (insert returns new map); Rust's `HashMap<K, Vec<V>>` mutates in place.
2. **Slice API**: Rust's `get` returns `&[V]` — a zero-copy view; OCaml's `find` returns the list by value.
3. **Entry API**: Rust's `or_default().push()` is one lookup; OCaml requires two operations (`find` + `add`).
4. **Library support**: The `multimap` crate and `indexmap::IndexMap` provide production-ready multimaps; OCaml's `Base.Map.add_multi` is the stdlib equivalent.

## Exercises

1. Add `remove_value(&mut self, key: &K, value: &V)` that removes one specific value from a key, keeping other values.
2. Implement `values_flat(&self) -> impl Iterator<Item=&V>` that iterates all values across all keys.
3. Write `invert<K, V>(m: &MultiMap<K, V>) -> MultiMap<V, K>` that inverts the multimap, mapping each value back to its keys.
