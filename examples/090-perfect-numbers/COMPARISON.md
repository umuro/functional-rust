# Perfect Numbers — Comparison

## Core Insight
Number classification shows how both languages handle three-way comparison. OCaml uses chained if/else; Rust can use `match` on `Ordering` for a more structured approach. The optimized sqrt version shows imperative style in both languages.

## OCaml Approach
- `List.init (n-1) (fun i -> i+1)` creates candidate divisors
- `List.filter` + `List.fold_left (+) 0` for sum
- Chained `if s = n then ... else if s > n then ...`
- Optimized version uses `ref` for mutable state (imperative OCaml)

## Rust Approach
- `(1..n).filter().sum()` — lazy range, no list allocation
- `s.cmp(&n)` returns `Ordering` — match on Equal/Greater/Less
- `while i * i <= n` loop for optimized version
- `flat_map` iterator version for functional sqrt approach

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Range | `List.init (n-1)` (allocates) | `(1..n)` (lazy) |
| Three-way | `if/else if/else` | `match s.cmp(&n)` |
| Mutable loop | `ref` + `while` | `let mut` + `while` |
| Invalid | `n <= 0` | `n == 0` (u64 can't be negative) |
| Sum | `List.fold_left (+) 0` | `.sum()` |

## Learner Notes
- `u64` means no negative numbers — Invalid only needs to check zero
- `std::cmp::Ordering` makes three-way comparisons explicit and exhaustive
- The sqrt optimization reduces O(n) to O(√n) — important for large numbers
- `flat_map` with `vec![]` in iterators is less efficient than the while loop
