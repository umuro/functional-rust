# 302: Option::transpose() — Collecting Optional Results

**Difficulty:** 2  **Level:** Intermediate

Convert `Option<Result<T, E>>` into `Result<Option<T>, E>` — the key to clean iterator pipelines.

## The Problem This Solves

You're iterating over optional values — perhaps from a config map where some keys exist and others don't. For the keys that exist, you need to parse their values. You get back `Option<Result<T, E>>` from each lookup — present-and-parseable, present-but-invalid, or absent. Now you want to propagate parse errors while filtering out absences.

Without `transpose()`, you'd write a nested match at every step. With it, you can `filter_map` cleanly: transpose the `Option<Result<T,E>>` into `Result<Option<T>,E>`, and the transpose's semantics do the right thing — `None` becomes `Ok(None)` (which `filter_map` skips), `Some(Ok(v))` becomes `Ok(Some(v))` (which `filter_map` keeps), and `Some(Err(e))` becomes `Err(e)` (which propagates).

The practical payoff: a one-liner that looks up optional config values, parses them, filters missing ones, and propagates any parse error — all without a single explicit `match`.

## The Intuition

`Option::transpose()` moves the `Result` layer *outside* the `Option` — turning "maybe a result" into "either an error, or maybe a value."

## How It Works in Rust

```rust
// Option::transpose() rules:
let some_ok: Option<Result<i32, &str>> = Some(Ok(42));
let some_err: Option<Result<i32, &str>> = Some(Err("bad"));
let none:    Option<Result<i32, &str>> = None;

some_ok.transpose()   // => Ok(Some(42))  — present and valid
some_err.transpose()  // => Err("bad")    — present but invalid: error propagates
none.transpose()      // => Ok(None)      — absent: treated as success with no value

// The killer use case: filter_map with error propagation
fn lookup_and_parse(
    map: &HashMap<&str, &str>,
    key: &str,
) -> Result<Option<i32>, ParseIntError> {
    map.get(key)              // Option<&&str>
       .map(|s| s.parse())   // Option<Result<i32, ParseIntError>>
       .transpose()          // Result<Option<i32>, ParseIntError>
}

// Collect a list: skip None, fail on bad parse, keep good values
let inputs: Vec<Option<&str>> = vec![Some("1"), None, Some("2"), Some("bad")];
let result: Result<Vec<i32>, _> = inputs.into_iter()
    .filter_map(|opt| opt.map(|s| s.parse::<i32>()).transpose())
    //  ↑ None → filtered out; Some(Err) → short-circuits; Some(Ok(v)) → kept
    .collect();
```

The `filter_map` + `transpose` idiom is the idiomatic way to "parse values that might not exist, fail on bad ones."

## What This Unlocks

- **Optional config values** — `lookup_and_parse` returns `Ok(None)` for missing keys and `Err` for bad values — exactly what callers need
- **Mixed iterators** — process collections where some entries might not apply and others might fail
- **`?` after transpose** — once you have `Result<Option<T>>`, use `?` normally and handle `None` with `unwrap_or` or `ok_or`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `Some(Ok(v))` → `Ok(Some(v))` | Manual match | `Option::transpose()` |
| `Some(Err(e))` → `Err(e)` | Manual match | `Option::transpose()` |
| `None` → `Ok(None)` | Manual match | `Option::transpose()` |
| filter_map + error propagation | Manual fold | `filter_map(|o| o.map(f).transpose()).collect()` |
