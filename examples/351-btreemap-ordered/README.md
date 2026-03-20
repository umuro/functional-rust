📖 **[View on hightechmind.io →](https://hightechmind.io/rust/351-btreemap-ordered)**

---

# 351: BTreeMap Ordered

## Problem Statement

`HashMap` gives O(1) average lookup but no ordering guarantees — iterating a `HashMap` yields keys in unpredictable order, and range queries are impossible. `BTreeMap` solves this by storing key-value pairs in a B-tree (Bayer & McCreight, 1972) — a self-balancing tree optimized for block-access patterns. All operations are O(log n) in the worst case, and keys are always iterated in sorted order. `BTreeMap` is the right tool when you need sorted output, range queries, `min_key`/`max_key` in O(log n), or when the number of entries is small enough that cache-friendly sorted data beats hash table overhead.

## Learning Outcomes

- Construct a `BTreeMap<K, V>` from an iterator of pairs
- Use `.range(from..=to)` for efficient range queries without scanning all keys
- Access the minimum key with `.keys().next()` and maximum with `.keys().next_back()`
- Understand that `BTreeMap` requires `K: Ord` (total ordering), unlike `HashMap`'s `K: Hash + Eq`
- Recognize when to prefer `BTreeMap` over `HashMap` based on access patterns
- Use `entry()` API for conditional insertion with ordered keys

## Rust Application

```rust
use std::collections::BTreeMap;

pub fn sorted_map<K: Ord, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V> {
    pairs.into_iter().collect()
}

pub fn range_query<K: Ord + Clone, V: Clone>(
    map: &BTreeMap<K, V>,
    from: &K,
    to: &K,
) -> Vec<(K, V)> {
    map.range(from..=to)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

pub fn min_key<K: Clone, V>(map: &BTreeMap<K, V>) -> Option<K> {
    map.keys().next().cloned()       // O(log n)
}

pub fn max_key<K: Clone, V>(map: &BTreeMap<K, V>) -> Option<K> {
    map.keys().next_back().cloned()  // O(log n)
}
```

`.range(lo..=hi)` returns an iterator over the subtree spanning `[lo, hi]` — much more efficient than scanning all entries. The B-tree's branching factor means the tree is shallow and cache-friendly: a million entries fit in ~4 levels.

## OCaml Approach

OCaml's `Map.Make` functor creates ordered maps parameterized by a comparison module:

```ocaml
module IntMap = Map.Make(Int)

let sorted_map pairs =
  List.fold_left (fun m (k, v) -> IntMap.add k v m) IntMap.empty pairs

(* Range query: filter (no built-in range iterator) *)
let range_query m lo hi =
  IntMap.filter (fun k _ -> k >= lo && k <= hi) m
  |> IntMap.to_seq |> List.of_seq

let min_key m = fst (IntMap.min_binding m)
let max_key m = fst (IntMap.max_binding m)
```

OCaml's `Map` is a purely functional AVL tree — all operations return new maps (persistent). Rust's `BTreeMap` is imperative with in-place modification. OCaml lacks a built-in range query iterator; `Map.filter` scans all entries (O(n)) unless you use a custom fold.

## Key Differences

| Aspect | Rust `BTreeMap` | OCaml `Map.Make` |
|--------|----------------|------------------|
| Underlying structure | B-tree (cache-friendly) | AVL tree (balanced binary tree) |
| Mutability | In-place mutation | Persistent (immutable, functional) |
| Range query | O(log n + k) with `.range()` | O(n) with `filter` |
| Ordering requirement | `K: Ord` | `compare` function via functor |
| Min/max | O(log n) via `next()`/`next_back()` | O(log n) via `min_binding`/`max_binding` |

## Exercises

1. **Word frequency sorted**: Count word frequencies from a string using `BTreeMap<&str, usize>`; print the top 5 words sorted alphabetically, then resort by frequency using a `BinaryHeap`.
2. **Range sum**: Given a `BTreeMap<i32, i32>` (key → value), implement `range_sum(map, lo, hi)` that returns the sum of values for all keys in `[lo, hi]` using `.range()`.
3. **Interval overlap**: Use a `BTreeMap<i32, i32>` where keys are interval starts and values are interval ends; implement a query that finds all intervals overlapping a given point using `.range(..=point)`.
