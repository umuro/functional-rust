# Accumulate — Comparison

## Core Insight
Implementing `map` from scratch reveals how both languages handle recursion, list construction, and higher-order functions. The recursive pattern is identical, but idiomatic Rust favors iterators over manual recursion.

## OCaml Approach
- `let rec accumulate f = function | [] -> [] | h :: t -> f h :: accumulate f t`
- Pattern matching on list constructors (`[]` and `h :: t`)
- Tail-recursive version reverses accumulator at the end
- `List.rev acc` is the standard tail-recursive pattern

## Rust Approach
- Slice patterns `[head, tail @ ..]` mirror OCaml list patterns
- `Vec::with_capacity` pre-allocates for known size
- Iterator version: `.into_iter().map(f).collect()` — one line
- Closure takes `&T` (borrow) or `T` (ownership) depending on variant

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Pattern | `h :: t` | `[head, tail @ ..]` |
| Build result | `::` cons | `vec![]` + extend |
| Tail-recursive | `List.rev acc` | `Vec::push` (already efficient) |
| Idiomatic | `List.map` | `.iter().map().collect()` |
| Closure | `fun x -> ...` | `|x| ...` or `\|x\| ...` |

## Learner Notes
- Rust slice patterns are nightly-stable since 1.42 — use them!
- `Vec::push` is amortized O(1), no need for reverse trick
- The iterator version is what you'd actually write in production Rust
- OCaml's `::` cons is O(1); Rust's `vec!` + extend is O(n) total
