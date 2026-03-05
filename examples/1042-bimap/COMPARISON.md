# Bidirectional Map — Comparison

## Core Insight
A bimap enforces a one-to-one relationship between keys and values. It's built from two maps synchronized on insert/remove. The tricky part is handling overwrites — if you insert a key that already exists, you must clean up the old value's backward entry, and vice versa.

## OCaml Approach
- Two functor-instantiated maps: `StringMap.t` and `IntMap.t`
- Record type `{ forward; backward }` holds both
- Insert checks both directions for existing mappings
- Immutable — returns new bimap on each operation
- Different map modules needed for different types

## Rust Approach
- `HashMap<K, V>` + `HashMap<V, K>` in a struct
- Generic over `K: Hash + Eq + Clone, V: Hash + Eq + Clone`
- `Clone` needed because values stored in both maps
- `remove` + `insert` for overwrite handling
- Single impl block covers all type combinations

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Forward map | `StringMap.t` (functor) | `HashMap<K, V>` |
| Backward map | `IntMap.t` (functor) | `HashMap<V, K>` |
| Generics | Need different functors per type | Single generic struct |
| Clone | N/A (immutable sharing) | Required for both K and V |
| Insert complexity | Two map rebuilds | Two hash inserts + cleanup |
| Mutability | Immutable | Mutable |
