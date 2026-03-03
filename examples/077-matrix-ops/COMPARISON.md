# Matrix Operations — Comparison

## Core Insight
Matrix operations reveal the difference between OCaml's persistent lists (shared by default) and Rust's owned vectors (moved by default). Transpose in OCaml creates new lists sharing structure; in Rust you must choose between consuming the input or borrowing it.

## OCaml Approach
- Nested lists `int list list` — immutable, GC-managed
- `List.init` + `List.nth` for transpose (O(n²) access)
- `List.fold_left2` elegantly zips and accumulates dot product
- No concern about who "owns" the matrix

## Rust Approach
- `Vec<Vec<i64>>` — heap-allocated, owned
- Two transpose variants: one takes ownership, one borrows with `&[Vec<i64>]`
- Iterator chains with `.map()`, `.zip()`, `.sum()`
- Must decide: does multiply consume or borrow inputs?

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Collection | `int list list` | `Vec<Vec<i64>>` |
| Access | `List.nth` O(n) | Index `[i]` O(1) |
| Ownership | GC shared | Move or borrow |
| Zip+fold | `fold_left2` | `.zip().map().sum()` |
| Transpose | Always new list | Can consume or borrow |

## Learner Notes
- Rust slices `&[Vec<i64>]` let you borrow without ownership transfer
- `.iter().map().collect()` is the Rust equivalent of `List.map`
- OCaml `List.nth` is O(n); Rust indexing is O(1) — different performance profile
- Consider using a flat `Vec<i64>` with stride for real matrix work
