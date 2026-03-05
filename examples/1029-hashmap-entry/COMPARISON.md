# HashMap Entry API — Comparison

## Core Insight
The "check-then-insert" pattern is so common that Rust dedicates a first-class API to it. The Entry API does one lookup and returns a handle that lets you insert, modify, or inspect — no second lookup needed. OCaml's immutable maps make this less critical (no aliasing bugs), but still require two operations.

## OCaml Approach
- `find_opt` + `add`: check if key exists, then add if not
- No entry API — pattern matching on `Option` is idiomatic
- Helper function `update_or_insert` encapsulates the pattern
- Immutability means no aliasing hazards, so double-lookup is safe

## Rust Approach
- `entry()` returns `Entry<K, V>` enum: `Occupied` or `Vacant`
- `or_insert(val)`: insert default if vacant
- `or_insert_with(|| expr)`: lazy default computation
- `or_default()`: use `Default::default()`
- `and_modify(|v| ...)`: modify if occupied, chain with `or_insert`
- Returns `&mut V` — can mutate in place

## Comparison Table

| Pattern | OCaml | Rust |
|---|---|---|
| Insert if absent | `find_opt` + `add` | `entry(k).or_insert(v)` |
| Lazy default | `find_opt` + `add (f ())` | `entry(k).or_insert_with(f)` |
| Modify or insert | custom helper | `entry(k).and_modify(f).or_insert(v)` |
| Lookups needed | 2 (find + add) | 1 (entry) |
| Return value | new map | `&mut V` |
| Thread safety | N/A (immutable) | Requires `&mut HashMap` |
