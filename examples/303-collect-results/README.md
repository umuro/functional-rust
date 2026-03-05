📖 **[View on hightechmind.io →](https://hightechmind.io/rust/303-collect-results)**

---

# 303: Collecting Iterator<Result<T>> into Result<Vec<T>>

**Difficulty:** 2  **Level:** Intermediate

Transform a sequence of fallible operations into an all-or-nothing result in one expression.

## The Problem This Solves

You have a list of strings to parse, a list of files to read, or a list of records to validate. Each individual operation returns `Result<T, E>`. You want the full list if everything succeeds, or the first error if anything fails. The naive approach is a `for` loop with early return — verbose and hard to compose with the rest of your iterator pipeline.

Rust's `FromIterator` implementation for `Result` solves this. When you write `.collect::<Result<Vec<T>, E>>()`, the iterator stops at the first `Err` and returns it — or returns `Ok(Vec)` containing all the values if every item was `Ok`. It's a short-circuit fold that's built into the type system.

This matters because it composes cleanly with the rest of the iterator toolkit. You can `map`, `filter`, and `flat_map` before the collect — the whole pipeline runs on the happy path, and any error short-circuits at collect time.

## The Intuition

`collect::<Result<Vec<T>, E>>()` is a short-circuit fold: accumulate all `Ok` values into a `Vec`, but stop and return the first `Err` encountered.

## How It Works in Rust

```rust
// The type annotation on collect() is what triggers this behavior
let inputs = ["1", "2", "3", "4"];
let parsed: Result<Vec<i32>, _> = inputs.iter()
    .map(|s| s.parse::<i32>())  // Iterator<Item = Result<i32, ParseIntError>>
    .collect();                  // Result<Vec<i32>, ParseIntError> — all or nothing

// Ok if everything succeeds
assert_eq!(parsed, Ok(vec![1, 2, 3, 4]));

// Err on first failure — later items aren't processed
let bad_inputs = ["1", "two", "3"];
let result: Result<Vec<i32>, _> = bad_inputs.iter()
    .map(|s| s.parse::<i32>())
    .collect();
// => Err(ParseIntError for "two") — "3" was never attempted

// The turbofish form when the type can't be inferred:
let result = inputs.iter()
    .map(|s| s.parse::<i32>())
    .collect::<Result<Vec<_>, _>>();
```

The short-circuit behavior is important: if you have expensive operations after the first bad item, they won't run. If you need to *process all items and collect all errors*, use `partition` (example 304) instead.

## What This Unlocks

- **Parse all-or-nothing** — validate a batch of inputs: either get all results or know the first failure
- **Composable pipeline** — chain with `filter`, `map`, `flat_map` before the collect; errors flow through naturally
- **One-liner batch processing** — replace a for-loop-with-early-return with a single expression

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| All-or-nothing fold | Manual fold with early return | `.collect::<Result<Vec<_>, _>>()` |
| Short-circuit on error | Manual | Automatic — first `Err` stops iteration |
| Type annotation | N/A | Required to select the `Result`-collecting `FromIterator` |
| vs. collect all errors | Manual two-pass | Use `partition(Result::is_ok)` instead |
