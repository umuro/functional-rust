# Pangram Check — OCaml vs Rust Comparison

## Core Insight

Both languages excel at set operations on characters, but Rust offers a zero-allocation bitflag approach alongside the idiomatic HashSet version. OCaml's `Set.Make` functor creates a balanced tree set, while Rust's `HashSet` is hash-based.

## OCaml Approach

Uses the `Set.Make(Char)` functor to create a character set module. Characters are filtered with `Seq.filter`, collected with `CS.of_seq`, and checked with `CS.subset`. The functor system requires declaring a module upfront.

## Rust Approach

Iterator chain: `.chars().filter().map().collect::<HashSet<_>>()` — the type drives `collect()` to deduplicate automatically. The bitflag variant uses `u32` as a 26-bit set with zero heap allocation, which has no direct OCaml equivalent without manual bit manipulation.

## Comparison Table

| Aspect        | OCaml                         | Rust                              |
|---------------|-------------------------------|-----------------------------------|
| **Memory**    | Tree-based set (heap nodes)   | HashSet (heap) or u32 bitflag (stack) |
| **Null safety** | N/A (bool result)          | N/A (bool result)                 |
| **Errors**    | Not applicable                | Not applicable                    |
| **Iteration** | `Seq.filter` + `CS.of_seq`   | `.chars().filter().map().collect()` |
| **Set type**  | `Set.Make` functor (balanced tree) | `HashSet` (hash table)       |

## Things Rust Learners Should Notice

1. **`collect()` is polymorphic** — collecting into `HashSet` vs `Vec` changes behavior (dedup vs preserve)
2. **Bitflag sets** are a common Rust idiom for small fixed domains — zero allocation, cache-friendly
3. **`is_ascii_alphabetic()`** — Rust has rich character classification methods built in
4. **No module functor needed** — `HashSet<char>` works directly without declaring a module
5. **`to_ascii_lowercase()`** works on individual chars, not just strings

## Further Reading

- [std::collections::HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [Exercism: Pangram](https://exercism.org/tracks/rust/exercises/pangram)
- [Bit manipulation in Rust](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators)
