# BTreeMap: Sorted Key Iteration — Comparison

## Core Insight
Both languages provide tree-based sorted maps. OCaml's `Map` uses a balanced binary tree (AVL); Rust's `BTreeMap` uses a B-tree optimized for cache locality. Both guarantee sorted iteration.

## OCaml Approach
- Functor-based: `Map.Make(Ord)` creates a map module for a given key type
- Immutable by default — `add` returns a new map
- `split` for range queries: splits map into (below, at, above)
- `min_binding` / `max_binding` for extremes
- `fold` for ordered traversal

## Rust Approach
- Generic: `BTreeMap<K: Ord, V>` — no functor needed
- Mutable with `insert`, or built from iterators via `collect()`
- `range()` accepts `RangeBounds` for efficient sub-range iteration
- `iter().next()` / `iter().next_back()` for min/max
- B-tree layout means better cache performance than binary trees

## Comparison Table

| Feature | OCaml (`Map`) | Rust (`BTreeMap`) |
|---|---|---|
| Structure | AVL tree | B-tree |
| Mutability | Immutable | Mutable |
| Key constraint | `Ord` module (functor) | `Ord` trait (generic) |
| Sorted iteration | `fold` / `iter` | `iter()` / `keys()` / `values()` |
| Range query | `split` (returns 3 parts) | `range(a..=b)` (lazy iterator) |
| Min/Max | `min_binding` / `max_binding` | `first_key_value()` / `last_key_value()` |
| Cache locality | Poor (pointer-heavy) | Good (B-tree nodes) |
