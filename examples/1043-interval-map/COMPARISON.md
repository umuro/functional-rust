# Interval Map — Comparison

## Core Insight
An interval map associates non-overlapping ranges with values. The key insight is using a sorted map keyed by interval start — point queries find the floor entry and check if the point falls within its range. Both languages use their sorted map (`Map`/`BTreeMap`).

## OCaml Approach
- `IntMap.t` with `(end, value)` as stored value
- `split` for point queries: find largest start ≤ point
- `filter` to remove overlapping intervals on insert
- Immutable — returns new map on each operation

## Rust Approach
- `BTreeMap<i64, (i64, V)>` — start → (end, value)
- `range(..=point).next_back()` for floor entry lookup
- `range(..hi).filter()` to find overlapping intervals
- Mutable with clean ownership semantics
- `assert!(lo < hi)` for invariant checking

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Backing map | `IntMap.t` | `BTreeMap` |
| Point query | `split` + `max_binding_opt` | `range(..=p).next_back()` |
| Overlap removal | `filter` | `range` + collect + remove |
| Insert | Immutable rebuild | Mutable in-place |
| Interval repr | `(start, (end, value))` | `(start, (end, value))` |
| Complexity | O(log n) query | O(log n) query |
