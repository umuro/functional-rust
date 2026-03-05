# Edit Distance (Levenshtein) — Comparison

## Core Insight
Edit distance finds the minimum number of single-character edits (insert, delete, replace) to transform one string into another. The 2D DP table has a clean recurrence with three operations mapping to three neighbors.

## OCaml Approach
- `Array.init` with conditional initialization for base cases
- `Array.blit` for row copying in space-optimized version
- Nested `min` calls: `min (min a b) c`
- `Hashtbl` memoization with tuple keys

## Rust Approach
- Explicit base case loops, then nested iteration
- `std::mem::swap` for efficient row swapping (zero-copy)
- Chained `.min()` calls: `a.min(b).min(c)`
- `HashMap` with tuple keys for memoized version

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Row swap | `Array.blit` (copies) | `std::mem::swap` (zero-copy) |
| Min of 3 | `min (min a b) c` | `a.min(b).min(c)` |
| Base cases | `Array.init` with conditionals | Explicit init loops |
| Range collect | `(0..=n).collect()` equivalent via `Array.init` | `(0..=n).collect::<Vec<_>>()` |
| Space optimization | Two arrays + blit | Two vecs + swap |
