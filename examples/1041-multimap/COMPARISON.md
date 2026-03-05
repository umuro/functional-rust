# Multimap — Comparison

## Core Insight
A multimap (one key → many values) is a thin wrapper over a map-to-list/vec. Both languages build it the same way — the difference is ergonomics. Rust's Entry API makes insertion clean; OCaml requires explicit find-or-create.

## OCaml Approach
- `'a list StringMap.t` — map to list of values
- `add`: find_opt + append + add
- `remove_value`: filter + conditional remove
- Immutable — each operation returns new multimap
- Module functions operate on the type alias

## Rust Approach
- `HashMap<K, Vec<V>>` wrapped in a struct
- `entry().or_default().push()` for insertion
- `remove_value`: find + position + remove
- Separate impl block for `V: PartialEq` operations
- `map_or(&[], ...)` for safe empty access

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Type | `'a list Map.t` | `HashMap<K, Vec<V>>` |
| Insert | `find_opt` + append + `add` | `entry().or_default().push()` |
| Get | `find_opt` / default `[]` | `get().map_or(&[], ...)` |
| Remove value | `filter` + conditional remove | `position` + `remove` |
| Mutability | Immutable | Mutable |
| Trait bounds | `Ord` (Map functor) | `Hash + Eq` (HashMap) |
