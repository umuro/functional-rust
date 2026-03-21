📖 **[View on hightechmind.io →](https://hightechmind.io/rust/269-iterator-partition)**

---

# 269: Splitting by Predicate with partition()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Classifying elements into exactly two groups — evens and odds, valid and invalid, passing and failing — is extremely common. The naive approach iterates twice: once to collect matches, once to collect non-matches. The `partition()` adapter solves this by splitting an iterator into two collections in a single pass, reducing both computation time and code verbosity. This appears constantly in data validation, input parsing, and batch processing pipelines.

## Learning Outcomes

- Understand `partition(pred)` as a single-pass split into two collections based on a predicate
- Recognize when `partition()` is more efficient than two separate `filter()` calls
- Use `partition()` to separate `Result::is_ok` from `Result::is_err` in a collection of results
- Combine `partition()` with type annotations to clarify output collection types

## Rust Application

`Iterator::partition(pred)` collects all elements where `pred` returns `true` into the first collection, and all others into the second, in a single iteration:

```rust
let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=6).partition(|&x| x % 2 == 0);
// evens = [2, 4, 6], odds = [1, 3, 5]

// Partition Results into ok and error values
let v: Vec<Result<i32, i32>> = vec![Ok(1), Err(2), Ok(3)];
let (oks, errs): (Vec<_>, Vec<_>) = v.into_iter().partition(Result::is_ok);
// oks = [Ok(1), Ok(3)], errs = [Err(2)]
```

## OCaml Approach

OCaml provides `List.partition` which is exactly equivalent:

```ocaml
let (evens, odds) = List.partition (fun x -> x mod 2 = 0) [1;2;3;4;5;6]
(* evens = [2;4;6], odds = [1;3;5] *)
```

This is a standard library function in OCaml, making it one of the cleaner parallels between the two languages.

## Key Differences

1. **Exact equivalent**: `List.partition` in OCaml and `Iterator::partition()` in Rust are semantically identical — same name, same behavior.
2. **Single pass**: Both implementations split in one traversal; using two `filter()` calls would require two passes.
3. **Result splitting**: A common Rust pattern uses `partition(Result::is_ok)` then `unwrap()` — but `partition_map` from the `itertools` crate offers a cleaner API.
4. **Stable ordering**: Both languages preserve the relative order of elements in each partition.

## Exercises

1. Partition a list of file paths into existing and non-existing files using `Path::exists()` as the predicate.
2. Parse a `Vec<&str>` of numbers, using `partition()` to simultaneously collect successfully parsed integers and the strings that failed to parse.
3. Partition a vector of transactions into credits (positive amounts) and debits (negative amounts), computing the sum of each group.
