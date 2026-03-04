# 046: Result Map

**Difficulty:** 1  **Level:** Foundations

Transform `Result` values without unwrapping them — keep errors flowing through cleanly.

## The Problem This Solves

You've called a function that returned `Result<i64, AppError>`. Now you want to double that number. The naïve approach: unwrap the result, double it, wrap it back. But that forces you to write error-handling boilerplate at every transformation step, even when you're just doing math on the success value.

This is the "pyramid of doom" problem familiar from JavaScript callbacks and Python nested `try/except` blocks. Each transformation adds another level of nesting. With five transformations, you've got deeply nested code that's hard to read and easy to get wrong.

`Result::map` solves this elegantly. It applies a function to the `Ok` value, and passes `Err` through untouched. You transform the happy path; errors take care of themselves. The result is flat, readable code where errors are handled once at the end, not at every step.

## The Intuition

You already know `Array.map()` in JavaScript or Python's list comprehension: apply a function to each element. `Result::map` is the same idea, but for a "container" that holds either a success value or an error.

The rule is simple:
- `Ok(x).map(f)` → `Ok(f(x))` — transform the value
- `Err(e).map(f)` → `Err(e)` — pass error through unchanged

`map_err` is the mirror image: transform the *error* if present, pass `Ok` through.

`and_then` (called `flatMap` in other languages) is where it gets powerful: instead of a plain function `T → U`, you give it a function `T → Result<U, E>`. This lets you chain operations that can *themselves* fail — like Promise.then() with async functions, but synchronous and fully explicit.

## How It Works in Rust

```rust
fn parse_int(s: &str) -> Result<i64, AppError> { ... }
fn check_range(x: i64, min: i64, max: i64) -> Result<i64, AppError> { ... }

// map: transform Ok value (plain function, can't fail)
let doubled = parse_int("5").map(|x| x * 2);
// Ok(10) — the closure never produces an Err

// map_err: transform the error type (e.g. to log-friendly String)
let stringified = parse_int("bad").map_err(|e| format!("{:?}", e));
// Err("ParseError(...)")

// and_then: chain a fallible operation
// Use this when the next step can ALSO return Err
let result = parse_int("100")
    .map(|x| x / 4)                          // infallible: just math
    .and_then(|x| check_range(x, 0, 50));     // fallible: might be out of range
// Ok(25) — 100/4=25, 25 is in [0,50]

// Errors short-circuit: the chain stops at the first Err
let result2 = parse_int("1000")
    .map(|x| x / 4)                          // Ok(250)
    .and_then(|x| check_range(x, 0, 50));     // Err(OutOfRange(250))
// Err — check_range fails, map step was wasted but harmless

// Collect a Vec of Results into Result<Vec<...>>
// None if ANY element is Err — the whole thing fails
let parsed: Result<Vec<i64>, AppError> =
    vec!["1", "2", "3"].iter().map(|s| parse_int(s)).collect();
// Ok([1, 2, 3])
```

## What This Unlocks

- **Data transformation pipelines:** Parse → validate → normalize → format, all as a single chain with one error-handling point at the end.
- **Error type adaptation:** `map_err` lets you convert between error types (e.g., library error → your domain error) without changing the logic.
- **Bulk operations:** Collecting `Iterator<Item=Result<T,E>>` into `Result<Vec<T>,E>` is traverse — process a list and fail fast on the first error.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Transform Ok value | `Result.map f r` | `r.map(f)` |
| Transform Err value | `Result.map_error f r` | `r.map_err(f)` |
| Chain fallible op | `Result.bind r f` or `r >>= f` | `r.and_then(f)` |
| Recover from error | `Result.catch` / custom | `r.or_else(\|e\| ...)` |
| All-or-nothing collect | `List.map f xs \|> Result.all` | `xs.iter().map(f).collect::<Result<Vec<_>,_>>()` |
| Chaining style | `>>=` operator (infix) | Method chain (`.and_then(...)`) |
