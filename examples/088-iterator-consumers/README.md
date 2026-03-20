[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 088 — Iterator Consumers
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Survey the terminal iterator operations that drive a lazy chain to produce a result: `sum`, `product`, `count`, `collect`, `fold`, `min`, `max`, `any`, `all`, `find`, and `for_each`. Demonstrate each with examples on ranges and slices, and compare with OCaml's `Seq.fold_left`-based equivalents.

## Learning Outcomes

- Understand that iterator adapters are lazy; consumers drive evaluation
- Use `sum::<i32>()` and `product::<i32>()` with turbofish type annotation
- Distinguish `collect::<Vec<_>>()` from `collect::<String>()` for different target types
- Use `fold` as the universal consumer from which all others derive
- Apply `min`, `max`, `any`, `all`, `find` as short-circuiting consumers
- Map each Rust consumer to the corresponding `Seq.fold_left` pattern in OCaml

## Rust Application

Rust iterators are lazy: calling `.filter(…).map(…)` on a range produces no values until a consumer drives the chain. `sum` and `product` require a type annotation (turbofish `::<i32>()`) because the compiler cannot infer the return type from the iterator alone. `collect` is polymorphic — it produces a `Vec`, `HashSet`, `String`, `HashMap`, or any `FromIterator` implementor depending on the type annotation. `fold` is the generalisation; `sum` is equivalent to `fold(0, |acc, x| acc + x)`. `any` and `all` short-circuit: they stop as soon as the answer is determined.

## OCaml Approach

OCaml's `Seq` module provides `fold_left`, `iter`, and `find`. Custom consumers like `seq_min`, `seq_max`, `seq_any`, and `seq_all` are implemented in terms of `fold_left` with an `Option` accumulator for min/max. The `Seq.fold_left` is strict: it consumes the entire sequence. Short-circuiting requires exceptions or early-exit patterns. Both approaches achieve the same results; Rust's built-in short-circuit guarantees on `any`/`all`/`find` are language-level.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Short-circuit | `any`/`all`/`find` stop early | `fold_left` always full scan |
| `sum` type | Needs turbofish `::<T>()` | `Seq.fold_left (+) 0 s` |
| `collect` | Polymorphic via `FromIterator` | `List.of_seq` / `Array.of_seq` |
| `min`/`max` | Built-in, returns `Option<&T>` | Custom fold with `None` accumulator |
| `for_each` | `iter.for_each(f)` | `Seq.iter f s` |
| `fold` | `fold(init, f)` | `Seq.fold_left f init s` |

Every lazy chain must end with a consumer. Choosing the right consumer is a code quality decision: `collect` when you need to store results, `for_each` for side effects, `fold` for aggregation, `any`/`all` when a boolean answer suffices. Avoid `collect` followed by indexing when a consumer suffices directly.

## Exercises

1. Write `max_by_key<T, K: Ord>(iter: impl Iterator<Item=T>, f: impl Fn(&T)->K) -> Option<T>` using `fold`.
2. Use `scan` (a stateful adapter) to produce a running sum from a range iterator.
3. Show that `sum` equals `fold(0, Add::add)` by implementing `my_sum` with `fold` and verifying equality.
4. Collect `(1..=5)` into a `HashSet<i32>` and a `BTreeSet<i32>` and compare membership test cost.
5. In OCaml, implement `seq_find : ('a -> bool) -> 'a Seq.t -> 'a option` that short-circuits using an exception internally.
