# 064: Traverse with Option

**Difficulty:** 2  **Level:** Intermediate

Apply a fallible function to every element of a list — succeed only if ALL elements succeed.

## The Problem This Solves

You have a list of strings and want to parse each one as an integer. But parsing can fail. If any element is unparseable, you want the whole operation to fail — not a partial list, not a list with holes, not a list of `Option<i32>`. Either every element parses successfully, or you get `None`.

This is the "all-or-nothing" pattern. It shows up constantly: validate a batch of form inputs (fail if any is invalid), fetch a list of resources (fail if any is unavailable), parse a config file's array (fail if any value is malformed). You need something stronger than `map` — you need `traverse`.

In JavaScript, `Promise.all([p1, p2, p3])` does exactly this for async operations: if any promise rejects, the whole thing rejects. Traverse is `Promise.all` for synchronous, functional code. And Rust's standard library has it built in.

## The Intuition

`traverse` is "map, then sequence." First apply a function to each element (like `map`), then "flip" the container: turn `Vec<Option<T>>` into `Option<Vec<T>>`. If any element is `None`, the whole result is `None`. If all are `Some`, collect them into a `Vec`.

The Rust punchline: **`iterator.map(f).collect::<Option<Vec<_>>>()`** is traverse, built into the standard library. The `collect()` method knows how to short-circuit on `None` when you're collecting into `Option<Vec<_>>`. No manual implementation needed.

The relationship between `traverse` and `sequence`:
- `sequence` takes `Vec<Option<T>>` and flips it to `Option<Vec<T>>` — traverse with identity function
- `traverse(f)` is `map(f)` followed by `sequence`
- In Rust: `xs.iter().map(f).collect()` does both at once

## How It Works in Rust

```rust
// The idiomatic Rust way: collect() IS traverse
fn traverse_option<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().map(f).collect()
    // When collecting into Option<Vec<U>>, Rust short-circuits on the first None
}

// Inline usage — most common form
let strs = ["1", "2", "3"];
let parsed: Option<Vec<i32>> = strs.iter().map(|s| s.parse().ok()).collect();
// Some([1, 2, 3]) — all succeeded

let mixed = ["1", "bad", "3"];
let parsed: Option<Vec<i32>> = mixed.iter().map(|s| s.parse().ok()).collect();
// None — "bad" returned None, short-circuited

// Sequence: you already have Vec<Option<T>>, just flip it
fn sequence_option<T: Clone>(xs: &[Option<T>]) -> Option<Vec<T>> {
    xs.iter().cloned().collect()
}
sequence_option(&[Some(1), Some(2), Some(3)]);  // Some([1, 2, 3])
sequence_option(&[Some(1), None,    Some(3)]);  // None

// Manual implementation with try_fold (shows what collect does internally)
fn traverse_fold<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().try_fold(Vec::new(), |mut acc, x| {
        acc.push(f(x)?);  // ? short-circuits try_fold on None
        Some(acc)
    })
}
```

## What This Unlocks

- **Batch validation:** Validate all fields in a form at once — `fields.iter().map(validate).collect::<Result<Vec<_>, _>>()` fails on the first invalid field.
- **Config parsing:** Parse an entire array from config — all values must be valid, or the config is rejected.
- **Result version:** The same pattern works with `Result` — `collect::<Result<Vec<_>, _>>()` — giving you the first error if any element fails.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Traverse implementation | Manual recursive `let rec traverse f = function \| [] -> Some [] \| x::xs -> ...` | `iter.map(f).collect::<Option<Vec<_>>>()` — built-in |
| Sequence | `traverse Fun.id xs` | `xs.iter().cloned().collect::<Option<Vec<_>>>()` |
| Short-circuit | Recursive: first `None` unwinds recursion | `collect()` stops iterating at first `None` |
| Works for Result | Separate implementation or functor | Same `collect()` pattern: `collect::<Result<Vec<_>, E>>()` |
| Manual fold | `List.fold_right` with pattern match | `try_fold` with `?` |
| Type annotation | Inferred from usage | Often needs turbofish: `.collect::<Option<Vec<_>>>()` |
