📖 **[View on hightechmind.io →](https://hightechmind.io/rust/304-partition-results)**

---

# 304: Splitting Ok/Err with partition()

**Difficulty:** 2  **Level:** Intermediate

Process all items in a batch and separate the successes from the failures.

## The Problem This Solves

You're processing a batch of user records — parsing ages, validating emails, reading config values. Some will fail. With `collect::<Result<Vec>>()`, the first failure stops everything: you get one error and lose all the successes. For batch processing, that's wrong — you want to know *all* the failures, not just the first one.

Real production batch jobs need a different contract: process everything, collect the successes, collect the failures, report both. An import script that stops at the first bad row and loses the other 999 valid ones is broken. A script that processes all rows and reports "imported 847, failed 153 (see attached log)" is useful.

`partition(Result::is_ok)` splits a `Vec<Result<T, E>>` into two `Vec`s in a single pass — all `Ok` items in one, all `Err` items in the other. No short-circuiting. Everything is processed.

## The Intuition

`partition(Result::is_ok)` is the "process all, separate outcomes" alternative to `collect::<Result<Vec>>()` — it never stops early, and you get both successes and failures.

## How It Works in Rust

```rust
let inputs = ["1", "two", "3", "four", "5"];

// Step 1: map to Result
let results: Vec<Result<i32, &str>> = inputs.iter()
    .map(|s| s.parse::<i32>().map_err(|_| *s))
    .collect();

// Step 2: partition into Ok and Err groups (single pass, processes ALL)
let (successes, failures): (Vec<_>, Vec<_>) = results
    .into_iter()
    .partition(Result::is_ok);

// Step 3: unwrap each group (safe — partition guarantees type)
let nums: Vec<i32> = successes.into_iter().flatten().collect();
//  ↑ .flatten() on Result<T, E> yields the Ok value
let bad: Vec<&str> = failures.into_iter().map(|r| r.unwrap_err()).collect();

println!("Parsed: {:?}", nums);       // [1, 3, 5]
println!("Unparseable: {:?}", bad);   // ["two", "four"]

// Practical: batch report
println!("{} succeeded, {} failed", nums.len(), bad.len());
```

The `.flatten()` trick works because `IntoIterator for Result<T, E>` yields zero items for `Err` and one item for `Ok`. After partition, you know all items in `successes` are `Ok`, so `flatten()` safely extracts the values.

## What This Unlocks

- **Batch processing with full reporting** — import jobs, validation pipelines, and bulk transforms that need to see all failures, not just the first
- **Graceful degradation** — process what you can, log what you can't, continue without aborting
- **Separate error handling** — deduplicate errors, group them by type, or write failed items to a dead-letter queue

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Split into two lists | Manual fold into pair | `partition(Result::is_ok)` |
| Short-circuits? | No (manual) | No — all items processed |
| vs. collect::\<Result\> | Stops at first Err | Never stops — collects all |
| Extract values after | Manual pattern match | `.flatten()` on Ok group; `.map(unwrap_err)` on Err group |
