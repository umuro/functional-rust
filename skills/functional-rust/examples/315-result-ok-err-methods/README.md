# 315: Result ok() and err() Methods

**Difficulty:** 2  **Level:** Intermediate

`.ok()` and `.err()` convert `Result` into `Option` — use them when you only care about one side of an outcome.

## The Problem This Solves

Sometimes you have a `Result<T, E>` but only care about whether it succeeded, not why it failed. Or you're building a collection and want to skip errors silently. Or you have a function returning `Option<T>` that you need to feed from a `Result`-based API. The `match` pattern for these conversions is repetitive and verbose.

`Result` has two "projection" methods that collapse it into an `Option`: `.ok()` keeps the success value and discards the error; `.err()` keeps the error and discards the success. They're conceptually symmetric and often used together with `filter_map` on iterators.

Understanding these alongside the full `Result` and `Option` method families — `map`, `and_then`, `or_else`, `flatten`, `zip` — eliminates most `match` expressions in favor of clean, chainable code. This example surveys the API that makes Rust's error handling feel like functional programming.

## The Intuition

`.ok()` and `.err()` are "I only care about one outcome" methods. When you call `.ok()`, you're saying "give me the value if it worked, or `None` if it didn't — I don't care about the error details." When you call `.err()`, you're saying "give me the error if it failed, or `None` if it succeeded."

The rest of the `Result` API follows a consistent algebra: `map` transforms the value, `map_err` transforms the error, `and_then` chains operations (like `flatMap`), `or_else` provides fallbacks, `and`/`or` combine results logically.

## How It Works in Rust

```rust
let ok: Result<i32, &str> = Ok(42);
let err: Result<i32, &str> = Err("oops");

// .ok() → Option<T>: keeps value, discards error
ok.ok()   // → Some(42)
err.ok()  // → None

// .err() → Option<E>: keeps error, discards value
ok.err()  // → None
err.err() // → Some("oops")

// Common use: filter_map to skip errors
let results = vec![Ok(1), Err("bad"), Ok(3)];
let values: Vec<i32> = results.into_iter().filter_map(|r| r.ok()).collect();
// → [1, 3]

// Chaining combinators
Ok(5_i32)
    .map(|x| x * 2)          // Ok(10)
    .and_then(|x| if x > 5 { Ok(x) } else { Err("too small") })
    .unwrap_or(0);            // → 10
```

## What This Unlocks

- **Seamless integration** between Result-producing APIs and Option-consuming ones — no boilerplate match expressions at the boundary
- **Clean iterator pipelines** — `.filter_map(|r| r.ok())` is the idiomatic "skip failures, collect successes" pattern
- **Chainable transformations** — the full `Result` and `Option` method algebra replaces nested `match` with readable combinator chains

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Result → Option (value) | `Option.of_result` / pattern match | `result.ok()` |
| Result → Option (error) | Pattern match | `result.err()` |
| Transform value | `Result.map` | `result.map(f)` |
| Transform error | `Result.map_error` | `result.map_err(f)` |
| Chain operations | `Result.bind` / `let*` | `result.and_then(f)` |
| Fallback on error | Manual match | `result.or_else(f)` |
