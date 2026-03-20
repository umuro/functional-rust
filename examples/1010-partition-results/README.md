📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1010-partition-results)**

---

# 1010-partition-results — Partition Results
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When processing a batch of fallible operations, you often want to continue rather than abort at the first error. A log ingestion pipeline, for example, should record malformed lines separately rather than crashing the entire run. This requires splitting an iterator of `Result` values into two separate collections: the successes and the failures.

Rust's `Iterator::partition` splits any iterator into two `Vec`s based on a predicate. Combined with `Result::is_ok` and `Result::unwrap`/`Result::unwrap_err`, this gives a clean batch-error-handling pattern without imperative loops.

## Learning Outcomes

- Use `Iterator::partition` to separate `Ok` and `Err` values
- Apply `filter_map` with `.ok()` or `.err()` to extract one side only
- Compare partition, fold-based accumulation, and filter_map approaches
- Understand the use case for tolerant batch processing versus fail-fast
- Know when to choose partition over `collect::<Result<Vec<T>, E>>()`

## Rust Application

`src/lib.rs` implements three strategies. `partition_results` calls `.partition(Result::is_ok)` to get two `Vec<Result<...>>`, then maps over each to unwrap the values — a two-pass approach. `partition_fold` uses a single `fold` call with two accumulators, which is slightly more efficient. `only_successes` and `only_errors` use `filter_map` with `.ok()` and `.err()` for when only one side is needed.

Real use cases include CSV parsers that report all bad rows, data migration scripts that log skipped records, and compiler frontends that accumulate all type errors before reporting.

## OCaml Approach

OCaml's `List.partition` is analogous:

```ocaml
let partition_results results =
  let (oks, errs) = List.partition Result.is_ok results in
  (List.filter_map Result.to_option oks,
   List.filter_map (function Error e -> Some e | _ -> None) errs)
```

OCaml's `Base` library provides `List.partition_map` which collapses the two steps into one pass using a `First`/`Second` variant.

## Key Differences

1. **Two-pass vs one-pass**: Rust's `partition` produces `Vec<Result<...>>` requiring a second unwrap pass; `fold` does it in one pass. OCaml's `partition_map` is one-pass by design.
2. **Consuming iterators**: Rust's iterator is consumed by `partition`; in OCaml, lists are persistent and can be re-processed freely.
3. **`filter_map` ergonomics**: Rust's `.ok()` and `.err()` methods make one-sided extraction concise; OCaml needs a lambda or function composition.
4. **Type safety on unwrap**: In Rust, calling `Result::unwrap` after `partition(Result::is_ok)` is technically safe but not statically verified; OCaml's pattern match is exhaustive.

## Exercises

1. Write a generic `partition_results<T, E>` function that takes `Vec<Result<T, E>>` and returns `(Vec<T>, Vec<E>)` without the intermediate `Vec<Result<...>>` step.
2. Extend the example to count how many items of each type were processed and return the counts alongside the partitioned vectors.
3. Implement a `take_while_ok` function that returns all leading `Ok` values from an iterator and stops at the first `Err`, returning both the values and the error.
