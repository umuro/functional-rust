# Group Elements by Key — Comparison

## Core Insight
Group-by is one of the most common data transformation patterns. Both languages build a map from key to collection of values, but Rust's Entry API makes the pattern a one-liner while OCaml requires explicit option matching.

## OCaml Approach
- `find_opt` + pattern match + `add` with appended list
- `List.fold_left` to accumulate groups
- Appending to list end (`@ [item]`) is O(n) — use cons + reverse for performance
- Generic version uses first-class modules or functors for key type

## Rust Approach
- `entry(key).or_default().push(value)` — single expressive line
- `or_default()` creates empty `Vec` if key absent
- Returns `&mut Vec<V>` so `push` works inline
- Generic version needs `Hash + Eq` bounds on key type

## Comparison Table

| Aspect | OCaml | Rust |
|---|---|---|
| Core pattern | `find_opt` + match + `add` | `entry().or_default().push()` |
| Lines of code | 4-5 per group-by | 1 |
| Key constraint | `Ord` (for `Map`) | `Hash + Eq` (for `HashMap`) |
| Value collection | List (prepend is O(1)) | Vec (append is amortized O(1)) |
| Mutability | Returns new map each step | Mutates in place |
| Iterator method | `fold_left` | for loop or `fold` |
