📖 **[View on hightechmind.io →](https://hightechmind.io/rust/892-partition-iterator)**

---

# 892-partition-iterator — Partition Iterator

## Problem Statement

Splitting a collection into groups based on a predicate is a fundamental data classification operation. Database query engines partition rows into matching and non-matching sets. Compilers partition tokens into valid and invalid. Trading systems partition orders into buy and sell. The naive approach uses two separate filter passes, scanning the data twice. `Iterator::partition` does this in one pass, returning two collections simultaneously. OCaml has `List.partition`. Python's `itertools` requires two passes. This single-pass property is critical for large data sets or when the source can only be consumed once.

## Learning Outcomes

- Use `Iterator::partition(pred)` to split into two collections in one pass
- Implement three-way partitioning using `.fold()` for multi-class classification
- Use `Either<L, R>` to route elements with transformation during partitioning
- Understand why partition is preferable to two `.filter()` calls
- Compare with OCaml's `List.partition` and three-way classification idioms

## Rust Application

`split_even_odd` uses `data.iter().partition(|&&x| x % 2 == 0)` — idiomatic one-pass split into two `Vec<i32>`. `partition3` uses `.fold()` with a three-tuple accumulator to split into three groups in one pass — no equivalent in standard iterators. `classify_numbers` calls `partition3` with negative/zero/positive predicates. The `Either<L, R>` enum and `partition_map` allow routing elements to different output types simultaneously, a pattern from Haskell's `Data.Either.partitionEithers`.

## OCaml Approach

`List.partition: ('a -> bool) -> 'a list -> 'a list * 'a list` is the direct equivalent. OCaml also has `List.partition_map: ('a -> ('b, 'c) Either.t) -> 'a list -> 'b list * 'c list` (since 4.14). Three-way partition: `List.fold_left (fun (a, b, c) x -> if p1 x then (x::a, b, c) else if p2 x then (a, x::b, c) else (a, b, x::c)) ([], [], []) xs`. OCaml's lists are singly-linked, so partition is O(n) but the cons-append order reverses the output (requires `List.rev`).

## Key Differences

1. **Output order**: Rust `partition` preserves input order; OCaml `List.partition` using cons-prepend requires `List.rev` to restore order.
2. **Two types**: Rust's `partition_map` with `Either` can partition into two collections of different types; OCaml `partition_map` similarly supports this.
3. **Three-way**: Neither language has built-in three-way partition — both use fold with a triple accumulator.
4. **Ownership**: Rust's `partition` on `iter()` produces `Vec<&T>`; using `into_iter()` produces `Vec<T>` with ownership transfer.

## Exercises

1. Implement `partition_at_index<T: Clone>(data: &[T], n: usize) -> (Vec<T>, Vec<T>)` that splits at position n.
2. Write `classify_words(words: &[&str]) -> (Vec<&str>, Vec<&str>, Vec<&str>)` that separates short (<5), medium (5-8), and long (>8) words.
3. Implement `stable_partition<T: Clone>(data: &[T], pred: impl Fn(&T) -> bool) -> Vec<T>` that moves all matching elements to the front while preserving relative order.
