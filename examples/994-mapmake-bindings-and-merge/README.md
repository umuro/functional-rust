# Example 994: Map.Make — Bindings and Merge

**Difficulty:** ⭐⭐
**Category:** stdlib-map
**OCaml Source:** OCaml stdlib `Map.Make` functor — `bindings` and `union`

## Problem Statement

Create two string-keyed integer maps, extract their sorted key-value pairs (`bindings`), and merge them so that values at shared keys are summed.

## Learning Outcomes

- `BTreeMap` is the idiomatic Rust analog to OCaml's `Map.Make(String)` — both maintain sorted key order
- The `entry` API enables ergonomic upsert without double-lookup
- `Iterator::fold` expresses map merge as a purely functional accumulation
- Higher-order conflict resolution (`map_union_with`) mirrors OCaml's `Map.union` callback signature

## OCaml Approach

OCaml uses the `Map.Make` functor to build a typed, ordered, persistent map module. `SMap.bindings` returns sorted `(key, value)` pairs as a list. `SMap.union` accepts a callback `(key -> v -> v -> v option)` for conflict resolution, returning a new immutable map.

## Rust Approach

`BTreeMap<String, i64>` provides the same ordered-key guarantee as `Map.Make(String)`. `.iter()` already iterates in sorted order, so `map_bindings` is a straightforward iterator map. Merge is expressed three ways: imperative loop with `entry`, a functional `fold`, and a generic higher-order `map_union_with` that mirrors OCaml's callback API.

## Key Differences

1. **Persistence:** OCaml maps are immutable/persistent — every operation returns a new map. Rust's `BTreeMap` is mutable; we clone to simulate non-destructive merge.
2. **Ordered iteration:** Both `Map.Make` and `BTreeMap` iterate keys in sorted order — `bindings` and `.iter()` are direct analogs.
3. **Entry API:** Rust's `entry().and_modify().or_insert()` replaces OCaml's `find_opt` + `add` pattern with a single, allocation-efficient operation.
4. **Generic callbacks:** OCaml's `Map.union` callback returns `'a option` (None = remove key). Rust's `map_union_with` replicates this: `None` from the closure removes the conflicting key.
