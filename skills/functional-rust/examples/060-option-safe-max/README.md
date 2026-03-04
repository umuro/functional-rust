# 060: Option — Safe List Maximum

**Difficulty:** 1  **Level:** Beginner

Find the maximum of a list without panicking on empty input, using `Option` to represent the absence of a result.

## The Problem This Solves

"What is the maximum of an empty list?" is not a sensible question. An empty list has no maximum. Code that crashes or returns a garbage sentinel (`i32::MIN`, `-1`) is hiding a logic error behind a magic value.

`Option<T>` makes the absence explicit in the type. A function returning `Option<i32>` is advertising: "I might not have an answer — you must handle both cases." The compiler enforces that you check. No null pointer dereferences. No forgotten sentinel comparisons.

This is how Rust eliminates an entire class of bugs: every function that might fail to produce a value says so in its signature, and the caller must handle both `Some` and `None`.

## The Intuition

`Option<T>` is an enum with two variants: `Some(value)` meaning "there is a value" and `None` meaning "there isn't one." It's like a box that either contains something or is explicitly empty — and the box type tells you it might be empty.

OCaml's `'a option` is the same idea, with the same `Some`/`None` constructors. The concept is nearly identical; the Rust version just has a richer API (40+ methods) built on top of it.

## How It Works in Rust

```rust
// Idiomatic: iterator's .max() already returns Option
pub fn list_max_idiomatic(xs: &[i32]) -> Option<i32> {
    xs.iter().copied().max()
    // .copied() converts Option<&i32> → Option<i32>
}

// Recursive: mirrors OCaml's pattern match on h :: t
pub fn list_max_recursive(xs: &[i32]) -> Option<i32> {
    match xs {
        [] => None,                   // empty: no answer
        [head, tail @ ..] => match list_max_recursive(tail) {
            None    => Some(*head),   // only element
            Some(m) => Some(if *head > m { *head } else { m }),
        },
    }
}

// Chain Option operations — map transforms the value inside:
pub fn double_max(xs: &[i32]) -> Option<i32> {
    list_max_idiomatic(xs).map(|x| x * 2)
    // None.map(f) == None — safe to chain without checking
}

// The ? operator propagates None early:
pub fn list_max_fold(xs: &[i32]) -> Option<i32> {
    let (&first, rest) = xs.split_first()?;  // ? returns None if empty
    Some(rest.iter().fold(first, |acc, &x| acc.max(x)))
}
```

`Option::map` transforms the inner value if `Some`, passes `None` through unchanged. This lets you chain operations without nested `match` expressions.

## What This Unlocks

- **Safe partial functions** — any function that might not have an answer: first element, lookup by key, parsing, conversion.
- **`Option` chaining** — `.map()`, `.and_then()`, `.unwrap_or()`, `?` operator build expressive pipelines without explicit null checks.
- **The `?` operator** — propagate `None` early from within a function returning `Option`, replacing nested `match` with a single `?`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type | `'a option` | `Option<T>` |
| Constructors | `Some x` / `None` | `Some(x)` / `None` |
| Pattern match | `match opt with Some x -> ... \| None -> ...` | `match opt { Some(x) => ..., None => ... }` |
| Chaining | `Option.map`, `Option.bind` | `.map()`, `.and_then()`, `?` operator |
| Iterator integration | `List.hd` raises `Failure` on empty | `iter().max()` returns `Option` natively |
| `.copied()` | Not needed | Converts `Option<&T>` → `Option<T>` |
