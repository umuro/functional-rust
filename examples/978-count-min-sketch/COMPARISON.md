# Count-Min Sketch — Comparison

## Core Insight
A Count-Min Sketch maintains a `depth × width` array of counters. `update(key, delta)` increments one counter per row (using d different hash functions). `query(key)` returns the minimum counter across all rows. The minimum gives the best estimate — other counters may be inflated by collisions. Guaranteed to never underestimate (hence "count-min"). Uses O(d × w) space for any number of distinct keys.

## OCaml Approach
- `Array.make_matrix depth width 0` — 2D counter array
- `make_hash seed` returns a closure capturing the seed (higher-order function)
- `make_hashes d` creates an array of `d` hash functions
- `String.fold_left (fun h c -> h * seed lxor Char.code c) seed s` — polynomial hash
- `for i = 0 to sk.depth - 1` loop for update/query
- `min_count := min !min_count v` — track minimum across rows

## Rust Approach
- `Vec<Vec<u64>>` — 2D counter array (depth rows, width columns)
- `seeds: Vec<u64>` — seed values for each row's hash
- `s.bytes().fold(seed, |h, b| h.wrapping_mul(seed).wrapping_add(b as u64) ^ b as u64)`
- `wrapping_mul`/`wrapping_add` — explicit overflow handling (OCaml wraps silently)
- `(0..self.depth).map(...).min().unwrap_or(0)` — functional min over rows
- `FrequencyTracker` wraps sketch + total events for frequency estimation

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Table | `int array array` | `Vec<Vec<u64>>` |
| Hash fn | Closure `fun s -> ...` | `fn hash(seed, key) -> u64` |
| Hash accumulate | `String.fold_left (fun h c -> h * seed lxor code c) seed s` | `s.bytes().fold(seed, \|h,b\| wrapping_mul.wrapping_add^b)` |
| Overflow | Silent wrap | `wrapping_mul`, `wrapping_add` |
| Min query | `for` loop + `ref min_count` | `.map().min().unwrap_or(0)` |
| Multiple hashes | `(string -> int) array` | `seeds: Vec<u64>` |
| Counter type | `int` (63-bit) | `u64` (64-bit) |
| Space | O(d × w × 8 bytes) | O(d × w × 8 bytes) |
| Error bound | `ε = e/w`, `δ = e^(-d)` | Same theoretical guarantee |
