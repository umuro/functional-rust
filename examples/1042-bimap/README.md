📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1042-bimap)**

---

# 1042-bimap — Bidirectional Map
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A bidirectional map (bimap) maintains a one-to-one correspondence between keys and values, enabling O(1) lookup in both directions: given a key, find the value; given a value, find the key. Use cases: ID to name lookups where you need reverse resolution, encoding/decoding tables, symbol tables in compilers.

The classic implementation uses two hash maps — one forward, one backward — and keeps them synchronized on insert and removal. Inserting a key that already exists must clean up both the old key-to-value and value-to-key entries.

## Learning Outcomes

- Implement a bidirectional map with two synchronized `HashMap`s
- Handle the invariant that both key and value must be unique
- Provide O(1) forward and reverse lookup
- Ensure consistency on insert: remove stale entries in both directions
- Understand when `bimap` crate provides additional guarantees

## Rust Application

`src/lib.rs` defines `BiMap<K, V>` with `forward: HashMap<K, V>` and `backward: HashMap<V, K>`. `insert` first removes any stale entries for both the key (forward direction) and value (backward direction) before inserting, ensuring consistency. `get_by_key` and `get_by_value` provide O(1) lookup in both directions.

Bidirectional maps appear in language parsers (token ↔ string), network code (port ↔ service name), and UI frameworks (widget ID ↔ widget instance).

## OCaml Approach

OCaml's persistent map requires coordination between two maps:

```ocaml
module StringMap = Map.Make(String)
module IntMap = Map.Make(Int)

type ('k, 'v) bimap = {
  forward: 'v StringMap.t;
  backward: 'k IntMap.t;
}
```

Persistent maps make the two-map approach natural — both maps are updated together when building a new bimap version. OCaml's `Base.Bimap` module provides this in the standard Base library.

## Key Differences

1. **Invariant maintenance**: Rust's `insert` must manually clean up stale entries in both directions; OCaml's persistent approach naturally handles this by rebuilding both maps.
2. **Uniqueness enforcement**: Both implementations silently overwrite conflicting entries; a strict variant would return `Err` on conflicts.
3. **Library support**: The `bimap` crate provides a production-ready Rust implementation; OCaml's `Base.Bimap` is the equivalent.
4. **Clone bounds**: Rust's `BiMap` requires `K: Clone + Hash + Eq` and `V: Clone + Hash + Eq` because both maps need copies of keys and values; OCaml's GC makes sharing free.

## Exercises

1. Add a `remove_by_key(&mut self, key: &K) -> Option<V>` method that removes an entry and keeps both maps consistent.
2. Implement `BiMap::from_iter(pairs: impl IntoIterator<Item=(K,V)>) -> Result<BiMap<K,V>, String>` that returns an error if any key or value appears twice.
3. Write a symbol table for a simple language using `BiMap<String, u32>` where the `u32` is a numeric ID.
