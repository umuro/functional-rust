📖 **[View on hightechmind.io →](https://hightechmind.io/rust/359-multimap-pattern)**

---

# 359: Multimap Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Standard maps store one value per key. A multimap stores multiple values per key — every tag in an index has multiple documents, every author writes multiple books, every event has multiple handlers. While no `MultiMap` exists in Rust's standard library, the pattern is straightforwardly implemented as `HashMap<K, Vec<V>>` with the entry API. This is the basis for inverted indexes in full-text search, adjacency lists in graph algorithms, and event subscription registries. Understanding the pattern helps recognize when it applies vs using `(K, V)` pair collections.

## Learning Outcomes

- Implement `MultiMap<K, V>` wrapping `HashMap<K, Vec<V>>`
- Use `entry(key).or_default().push(value)` to append values
- Retrieve all values for a key with `get(&key) -> Option<&Vec<V>>`
- Remove one value from a key's list without removing the entire key
- Count values per key with `count(&key)`
- Recognize the multimap as an inverted index and adjacency list

## Rust Application

```rust
use std::collections::HashMap;

pub struct MultiMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Eq + std::hash::Hash, V> MultiMap<K, V> {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.entry(key).or_default().push(value);
    }

    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    pub fn get_all(&self, key: &K) -> Vec<&V> {
        self.inner.get(key).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn remove_one(&mut self, key: &K) -> Option<V> {
        self.inner.get_mut(key).and_then(|v| v.pop())
    }

    pub fn count(&self, key: &K) -> usize {
        self.inner.get(key).map(|v| v.len()).unwrap_or(0)
    }
}
```

`or_default()` creates an empty `Vec<V>` if the key is absent, then `push(value)` appends. All subsequent inserts for the same key append to the existing vec. This avoids re-allocating the key on each insert — the entry API ensures a single lookup.

## OCaml Approach

OCaml's `Hashtbl` natively supports multiple bindings per key:

```ocaml
let mm = Hashtbl.create 16

(* Hashtbl.add allows multiple values per key *)
let insert tbl k v = Hashtbl.add tbl k v

(* Retrieve all values for a key *)
let get_all tbl k = Hashtbl.find_all tbl k

(* Count values for a key *)
let count tbl k = List.length (Hashtbl.find_all tbl k)
```

`Hashtbl.find_all` returns all values associated with a key as a list. This differs from Rust's explicit `HashMap<K, Vec<V>>` — OCaml's multimap is implicit in `Hashtbl`'s design (multiple `add` calls for the same key stack bindings).

## Key Differences

| Aspect | Rust `HashMap<K, Vec<V>>` | OCaml `Hashtbl` (multimap) |
|--------|--------------------------|---------------------------|
| Multiple values | Explicit `Vec<V>` per key | Implicit stacked bindings |
| Get all values | `get(k)` → `&Vec<V>` | `find_all k` → `list` |
| Ordered values | Yes (insertion order in `Vec`) | Yes (reverse insertion order) |
| Remove one | `Vec::pop()` on the inner vec | `Hashtbl.remove` removes one |
| Remove all | `HashMap::remove(k)` | `Hashtbl.remove` repeatedly |

## Exercises

1. **Inverted index**: Build a multimap from `Vec<(word, doc_id)>` pairs; query "all documents containing 'rust'" and "all documents containing both 'rust' and 'async'" (intersection of two value lists).
2. **Adjacency list graph**: Represent a directed graph as `MultiMap<usize, usize>` where `insert(from, to)` adds an edge; implement `reachable_from(start)` using BFS.
3. **Dedup multimap**: Add a `insert_unique` method that only adds the value if it's not already present for that key (requires `V: PartialEq`); implement using `.contains()` check before `.push()`.
