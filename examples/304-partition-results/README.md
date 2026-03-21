📖 **[View on hightechmind.io →](https://hightechmind.io/rust/304-partition-results)**

---

# 304: Splitting Ok/Err with partition()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

When processing a batch of inputs where some may fail, sometimes you want both the successes and the failures — not a short-circuit. Importing a CSV where invalid rows are logged and skipped, processing API responses where some fail and others succeed, or batch-validating records while collecting all errors. The `partition(Result::is_ok)` pattern collects all results in one pass, then extracts the `Ok` and `Err` values into separate vectors.

## Learning Outcomes

- Use `partition(Result::is_ok)` to split a `Vec<Result<T, E>>` into successes and failures
- Extract values from `Ok` and `Err` variants after partitioning
- Recognize this as the "harvest all results" counterpart to `collect::<Result<Vec<_>, _>>()`
- Use `fold` for more control over multi-way result classification

## Rust Application

Partition into `Ok` and `Err` groups, then extract inner values:

```rust
pub fn partition_results<T, E>(
    results: Vec<Result<T, E>>
) -> (Vec<T>, Vec<E>) {
    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    let ok_vals: Vec<T> = oks.into_iter().map(|r| r.unwrap()).collect();
    let err_vals: Vec<E> = errs.into_iter().map(|r| r.unwrap_err()).collect();
    (ok_vals, err_vals)
}
// All successes and all failures, in one pass
```

The `itertools` crate provides `partition_map` which avoids the intermediate `Vec<_>` step.

## OCaml Approach

OCaml uses `List.partition_map` (OCaml 4.12+) or a fold:

```ocaml
let partition_results results =
  List.partition_map (function
    | Ok v -> Left v
    | Error e -> Right e
  ) results
```

Earlier OCaml versions use `List.fold_left` with two accumulator lists.

## Key Differences

1. **OCaml 4.12+**: `List.partition_map` (new in 4.12) closely mirrors this Rust pattern with `Left`/`Right` discrimination.
2. **Two passes in Rust**: Rust's `partition` + `map(unwrap)` uses two passes; `itertools::partition_map` does it in one.
3. **Error collection**: This pattern enables collecting all validation errors at once, which is better UX than "fix one error, retry, discover the next error".
4. **Ordering**: Both approaches preserve the relative order of elements within each group.

## Exercises

1. Parse a CSV file where each line may fail, collecting successfully parsed rows and all parse errors into separate lists.
2. Implement a `classify_results<T, E>` function that takes a `Vec<Result<T, E>>` and returns three groups: successes, retryable errors, and permanent errors (based on error kind).
3. Compare memory usage of `partition(Result::is_ok)` followed by `unwrap()` versus a single `fold` that accumulates directly — which avoids more intermediate allocations?
