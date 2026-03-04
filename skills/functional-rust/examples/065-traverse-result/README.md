# 065: Traverse with Result

**Difficulty:** ⭐⭐  **Level:** Foundations

Apply a fallible function to every element in a list and collect all successes — or stop at the first failure.

## The Problem This Solves

You have a list of raw inputs — strings from a form, rows from a CSV, IDs from a request — and you want to parse or validate each one. If any input is invalid, you want the whole thing to fail with the first error. If all are valid, you want the collected results.

In JavaScript this is `Promise.all()`: give it an array of Promises, get back a Promise of an array. If any Promise rejects, the whole thing rejects. `traverse` is the synchronous, pure version of that pattern.

Without it, you'd write a loop, accumulate results, check for errors, propagate manually. It gets ugly fast and every version looks slightly different. Rust makes this a one-liner using the compiler's understanding of `Result`.

## The Intuition

The magic is in `.collect::<Result<Vec<_>, _>>()`. Rust's `collect()` is polymorphic — it knows how to collect an iterator of `Result<T, E>` values into a `Result<Vec<T>, E>`. It runs through the iterator, accumulates successes, and short-circuits on the first `Err`. You get exactly the `Promise.all()` semantics for free.

In OCaml, you write this yourself using `fold_right` or explicit recursion. In Rust, you just ask `collect()` to produce the right type and the compiler handles it.

## How It Works in Rust

```rust
// Apply a fallible function to each element, collect or fail
fn traverse_result<T, U, E, F: Fn(&T) -> Result<U, E>>(
    xs: &[T],
    f: F,
) -> Result<Vec<U>, E> {
    xs.iter().map(f).collect()  // that's it — collect knows what to do
}
```

The type annotation on `collect()` is the key — you're asking for `Result<Vec<_>, _>`:

```rust
// Explicit type annotation form:
let results: Result<Vec<i32>, String> = inputs
    .iter()
    .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
    .collect::<Result<Vec<_>, _>>();
```

Sequence (traverse with identity — "flip" a `Vec<Result>` into a `Result<Vec>`):

```rust
fn sequence_result<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().collect()  // same trick
}
```

## What This Unlocks

- **Batch validation** — parse or validate a list of inputs and get either all results or the first error
- **Database batch operations** — run a fallible transformation on each item before inserting
- **Config loading** — parse multiple required config values, fail early if any is missing or malformed

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Implementation | Recursive or `fold_right` | `.collect::<Result<Vec<_>,_>>()` |
| Error short-circuit | Pattern match on `Error e` | Built into `collect()` |
| `sequence` | `traverse Fun.id` | Same `.collect()` on `Vec<Result<T,E>>` |
| Explicit fold | Manual `fold_right` | `try_fold` on iterator |
