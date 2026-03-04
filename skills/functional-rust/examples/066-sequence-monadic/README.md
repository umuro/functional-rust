# 066: Sequence Monadic

**Difficulty:** ⭐⭐  **Level:** Foundations

Turn a list of optional or fallible values into a single optional or fallible list — and understand why `collect()` is Rust's universal sequence operator.

## The Problem This Solves

You have a `Vec<Option<T>>` — maybe from a list of lookups that might not find anything — and you want a single `Option<Vec<T>>`. If any lookup returned `None`, the whole thing should be `None`. If all succeeded, you want the collected values.

Same pattern for errors: `Vec<Result<T, E>>` → `Result<Vec<T>, E>`. This is called `sequence` in functional programming. It's the pattern behind "if everything succeeds, give me the results; otherwise give me the failure."

Without sequence, you write a loop with early returns, accumulating into a temporary vec, checking for failures, returning the right variant. Every time. With sequence, it's one line.

## The Intuition

`sequence` is `traverse` with the identity function — you're not transforming values, just flipping the container nesting. A list of Optionals becomes an Optional list. A list of Results becomes a Result of list.

In OCaml, you write this with `fold_right`, pattern-matching on each element. In Rust, `collect()` does it for you — it's polymorphic over the output container type, and both `Option` and `Result` implement the `FromIterator` trait that makes this work.

Think of `collect()` as "assemble these pieces into the right container." When the container is `Result<Vec<_>, _>`, it knows to short-circuit on `Err`. When it's `Option<Vec<_>>`, it short-circuits on `None`.

## How It Works in Rust

```rust
// sequence for Option: None anywhere → None
fn sequence_option<T>(xs: Vec<Option<T>>) -> Option<Vec<T>> {
    xs.into_iter().collect()
}

// sequence for Result: Err anywhere → first Err
fn sequence_result<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().collect()
}
```

Real-world example — parse a list of strings, fail if any is invalid:

```rust
let parsed: Option<Vec<i32>> = vec!["1", "2", "3"]
    .iter()
    .map(|s| s.parse::<i32>().ok())  // each parse gives Option<i32>
    .collect();                        // collect() turns Vec<Option<i32>> into Option<Vec<i32>>

// parsed == Some(vec![1, 2, 3])
// If any string was "bad", parsed == None
```

## What This Unlocks

- **Batch lookups** — query a database for multiple IDs; if any is missing, return `None`
- **Multi-field parsing** — parse every field in a struct from strings; get `Result<Struct, Error>` at the end
- **Chaining optional steps** — combine multiple `Option`-returning operations into one

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| sequence for Option | `fold_right` with pattern match | `.collect::<Option<Vec<_>>>()` |
| sequence for Result | `fold_right` with pattern match | `.collect::<Result<Vec<_>,_>>()` |
| Generic sequence | Higher-order `bind`/`return_` | `FromIterator` trait (built-in) |
| Short-circuit | Returns first `None`/`Error e` | Same semantics, built into `collect()` |
