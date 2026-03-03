# Isogram Check — OCaml vs Rust Comparison

## Core Insight

Duplicate detection is fundamentally a set membership problem. OCaml uses `List.sort_uniq` to compare lengths, while Rust's `HashSet::insert()` returns a boolean indicating whether the element was new — enabling early termination.

## OCaml Approach

Filters characters to lowercase alpha, converts to list, applies `List.sort_uniq`, and compares lengths. The sort-based dedup is O(n log n). Alternatively, a recursive approach with `Set.Make(Char)` gives O(n log n) with early exit.

## Rust Approach

Three approaches: (1) collect to Vec and HashSet, compare lengths — O(n) average; (2) early-exit using `HashSet::insert()` return value; (3) bitflag for zero-allocation O(n). The early-exit pattern is idiomatic Rust and has no direct OCaml equivalent without mutable state.

## Comparison Table

| Aspect        | OCaml                            | Rust                                  |
|---------------|----------------------------------|---------------------------------------|
| **Memory**    | List + sorted copy               | HashSet or u32 bitflag                |
| **Null safety** | N/A                           | N/A                                   |
| **Errors**    | Not applicable                   | Not applicable                        |
| **Iteration** | `Seq.filter` + `List.of_seq`     | `.chars().filter().map()`             |
| **Dedup**     | `List.sort_uniq` (O(n log n))    | `HashSet` (O(n) average)             |

## Things Rust Learners Should Notice

1. **`HashSet::insert()` returns bool** — this is a key API design that enables early exit patterns
2. **Three allocation tiers**: HashSet (heap), Vec+HashSet (heap), bitflag u32 (stack-only)
3. **`copied()` iterator adapter** — converts `&char` to `char` for collecting into HashSet
4. **Filtering and mapping are lazy** — the iterator chain doesn't allocate until `collect()`
5. **Rust's `return` in closures** — early return only exits the closure, use a `for` loop for early function exit

## Further Reading

- [HashSet::insert](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.insert)
- [Exercism: Isogram](https://exercism.org/tracks/rust/exercises/isogram)
