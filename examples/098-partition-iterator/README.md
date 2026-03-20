[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 098 — Partition Iterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Use Rust's `.partition(predicate)` to split an iterator into two collections in a single pass: elements satisfying the predicate go into the first collection, the rest into the second. Compare with OCaml's `List.partition` and `partition_map` using `Either`.

## Learning Outcomes

- Use `iter.partition(|x| pred(x))` to produce two `Vec<T>` simultaneously
- Understand that `partition` consumes the iterator in one pass — no intermediate allocation
- Require a type annotation `(Vec<T>, Vec<T>)` for the result destructuring
- Handle edge cases: empty input, all-match, no-match
- Map Rust's `.partition()` to OCaml's `List.partition`
- Extend to `partition_map` for element-wise classification into two typed collections

## Rust Application

`(1..=6).partition(|x| x % 2 == 0)` returns `(Vec<i32>, Vec<i32>)` — evens and odds collected in one traversal. The type annotation on the destructuring `(Vec<i32>, Vec<i32>)` is required because `partition` is generic. `std::iter::empty().partition(…)` returns two empty vecs — correct handling of empty input. `partition` is equivalent to `fold` collecting into two `Vec`s, but more expressive.

## OCaml Approach

`List.partition pred lst` is built into the standard library and returns a tuple `(list, list)`. `partition_map f lst` uses `Either.Left`/`Either.Right` to route elements to different typed collections via `fold_right`. OCaml's `partition` is lazy-safe only via `Seq.partition` in newer versions.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Built-in | `Iterator::partition(pred)` | `List.partition pred lst` |
| Return type | `(Vec<T>, Vec<T>)` | `'a list * 'a list` |
| Type annotation | Required on destructuring | Inferred |
| `partition_map` | Custom or `itertools` | `List.partition_map` (OCaml 4.12+) |
| Single pass | Yes | Yes |
| Empty input | Returns two empty vecs | Returns `([], [])` |

`partition` is cleaner than filter + filter for two complementary predicates — it avoids traversing the input twice. Use it whenever you need both sides of a classification simultaneously.

## Exercises

1. Implement `partition_map<T, A, B>(iter: impl Iterator<Item=T>, f: impl Fn(T) -> Result<A, B>) -> (Vec<A>, Vec<B>)` using `fold`.
2. Use `partition` to split a `Vec<Result<T, E>>` into `(Vec<T>, Vec<E>)`.
3. Implement `three_way_partition<T>(v: Vec<T>, pred1: impl Fn(&T)->bool, pred2: impl Fn(&T)->bool) -> (Vec<T>, Vec<T>, Vec<T>)`.
4. Show that `partition(pred)` is equivalent to `(filter(pred).collect(), filter(|x| !pred(x)).collect())` and explain the trade-off.
5. In OCaml, implement `partition_fold` that groups list elements into `n` buckets based on `f: 'a -> int` using `Array` of lists.
