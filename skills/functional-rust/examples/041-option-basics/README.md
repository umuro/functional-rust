# 041: Option Basics

**Difficulty:** 1  **Level:** Foundations

Meet `Option<T>` — Rust's way of saying "this value might not exist", with no nulls allowed.

## The Problem This Solves

Every language has the "might be empty" problem: dividing by zero, getting the first element of an empty list, looking up a key that might not exist. Most languages solve this with `null` or `None` — and then silently crash at runtime when you forget to check.

Python's `None`, JavaScript's `null`/`undefined`, Java's `null` — all the same trap: the absence of a value looks like a normal value until it explodes. Tony Hoare called null "my billion-dollar mistake."

Rust has **no null**. Instead, `Option<T>` is a type that's either `Some(value)` or `None`. The compiler sees these as completely different cases. You *cannot* use an `Option<T>` as if it were a `T` — you must explicitly handle both cases. The error happens at compile time, not at 3am on production.

## The Intuition

In Python: `Optional[int]` from the `typing` module. But Python's type hints are advisory — the runtime doesn't enforce them. You can still pass `None` where an `int` was expected and only find out when the code runs.

In Rust, `Option<T>` is not a hint — it's the actual type. There is no `T` until you unwrap the `Option`. The two cases:

```
Some(42)  — there IS a value, and it's 42
None      — there is NO value
```

This is equivalent to a tagged union (discriminated union in TypeScript, `Maybe` in Haskell, `Optional<T>` in Java 8+). The difference: Rust enforces it at compile time.

## How It Works in Rust

```rust
// Safe division: instead of panicking on /0, return None
fn safe_div(a: i64, b: i64) -> Option<i64> {
    if b == 0 { None } else { Some(a / b) }
}

// Safe head: instead of panicking on empty slice, return None
fn head<T: Clone>(lst: &[T]) -> Option<T> {
    lst.first().cloned()
}
```

**Using the result — three common patterns:**

```rust
// 1. Pattern match (explicit, exhaustive)
match safe_div(10, 2) {
    Some(result) => println!("Got: {}", result),
    None => println!("Division by zero"),
}

// 2. unwrap_or (provide a default)
let x = safe_div(10, 0).unwrap_or(0);  // x = 0

// 3. The ? operator (propagate None upward)
fn compute() -> Option<i64> {
    let x = safe_div(100, 5)?;  // if None, return None from compute()
    let y = safe_div(x, 2)?;
    Some(y + 1)
}
```

**Never do this in production:** `option.unwrap()` — it panics on `None`. Use `unwrap_or`, `unwrap_or_else`, or `?` instead.

## What This Unlocks

- **Null-safe code** — any function that might "not find anything" returns `Option<T>` instead of a nullable type.
- **Forced error handling** — the compiler won't let you ignore a `None` case.
- **Composable chains** — `Option` values compose naturally with `map`, `and_then`, and `?` (covered in examples 042-044).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| The type | `'a option` | `Option<T>` |
| Present value | `Some x` | `Some(x)` |
| Absent value | `None` | `None` |
| Pattern match | `match opt with \| Some x -> ... \| None -> ...` | `match opt { Some(x) => ..., None => ... }` |
| Default value | `Option.value opt ~default:0` | `opt.unwrap_or(0)` |
| Propagate None | `Option.bind` or `let*` | `?` operator |
