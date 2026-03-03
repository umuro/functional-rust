# Difference of Squares — Comparison

## Core Insight
Simple mathematical computations highlight the difference between OCaml's list-based iteration and Rust's range iterators. Rust ranges are lazy and allocation-free; OCaml's `List.init` creates a temporary list.

## OCaml Approach
- `List.init n (fun i -> i + 1)` creates [1..n] as a list (heap allocation)
- `List.fold_left (+) 0` sums the list
- `|>` pipe operator chains transformations
- Closed-form formulas avoid list creation

## Rust Approach
- `(1..=n)` creates a lazy range (no allocation)
- `.sum()` and `.map(|x| x * x).sum()` consume the iterator
- Ranges are zero-cost abstractions
- Same closed-form formulas work identically

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Range | `List.init n (fun i -> i+1)` | `(1..=n)` |
| Sum | `List.fold_left (+) 0` | `.sum()` |
| Map+Sum | `fold_left (fun acc x -> ...)` | `.map().sum()` |
| Allocation | List created in memory | Zero allocation (lazy) |
| Overflow | Silent | Panic in debug mode |

## Learner Notes
- Rust ranges `1..=n` are inclusive; `1..n` is exclusive (like Python)
- `.sum()` requires type annotation sometimes: `let s: u64 = (1..=n).sum()`
- OCaml `List.init` is O(n) space; Rust ranges are O(1) space
- Both closed-form versions are O(1) time and space
