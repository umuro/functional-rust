# 580: Option Matching Idioms

**Difficulty:** 2  **Level:** Beginner

Work with `Option<T>` fluently — combine direct matching with the combinator API.

## The Problem This Solves

Null-safety is the original motivation. Every other language has a "billion-dollar mistake" — the null reference — that compiles fine and crashes at runtime. Rust has no null. Optional values are represented as `Option<T>`, and you cannot use the inner value without explicitly handling the `None` case.

But that creates a new problem: verbose `match` blocks everywhere. `match opt { Some(v) => do_something(v), None => default }` is correct but noisy for simple transformations. And chaining operations on `Option` — parse, validate, transform — becomes a pyramid of `if let` blocks.

The `Option` combinator API (`map`, `and_then`, `filter`, `unwrap_or`, `zip`) is the answer: transform an `Option<T>` without unpacking it. You write the happy-path logic; the `None` propagates automatically.

## The Intuition

`Option<T>` is an enum with two variants: `Some(T)` (a value) and `None` (absent). Matching it exhaustively is the safe baseline. Combinators are the ergonomic layer on top.

Think of `map` as "apply this function inside the box, if there is a box." `and_then` (OCaml: `bind`) is "apply this function that might fail — returning another Option — and flatten the result." `filter` discards a `Some` if the value doesn't pass a test.

The combinator chain `opt.map(transform).and_then(validate).unwrap_or(default)` reads like a pipeline: transform if present, validate, fall back if anything fails. That's the Option monad. OCaml spells this with `let*` and custom bind operators. Rust puts it directly on the type.

`filter_map` is the hidden gem of iterator processing: it maps and discards `None` in one pass, avoiding a separate `.filter()` + `.map()` chain.

## How It Works in Rust

```rust
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn safe_sqrt(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}

// Combinator chain — None propagates through automatically
fn compute(a: i32, b: i32) -> Option<f64> {
    safe_div(a, b)
        .map(|q| q as f64)      // transform if Some
        .and_then(safe_sqrt)     // chain another fallible operation
        .map(|r| r * 2.0)       // transform the result
}

// filter_map — transform + filter in one pass
let names: Vec<Option<&str>> = vec![Some("alice"), None, Some("bob")];
let upper: Vec<_> = names.iter()
    .filter_map(|o| o.map(str::to_uppercase))
    .collect();  // ["ALICE", "BOB"] — None silently dropped

// Fallback values
let x: Option<i32> = None;
x.unwrap_or(0);              // 0 — cheap default
x.unwrap_or_else(|| 42);     // 42 — computed lazily
x.unwrap_or_default();       // 0 — T's Default::default()

// and_then for chained fallible ops
let parsed = Some("42")
    .and_then(|s| s.parse::<i32>().ok())  // parse may fail
    .filter(|&n| n > 0);                   // discard negatives

// flatten, zip
Some(Some(42)).flatten();           // Some(42) — remove one Option layer
Some(1).zip(Some("hello"));         // Some((1, "hello"))
Some(1).zip(None::<&str>);          // None
```

## What This Unlocks

- **Null-safe pipelines** — chain transforms without unwrapping; `None` propagates without an if-statement.
- **Iterator filtering** — `filter_map` is the idiomatic way to map and drop failures in one pass.
- **Forced explicitness** — you cannot accidentally use an absent value; the type system requires you to handle `None`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type | `'a option` | `Option<T>` |
| Variants | `Some x`, `None` | `Some(x)`, `None` |
| Map | `Option.map f opt` | `opt.map(\|x\| f(x))` |
| Chain/bind | `Option.bind opt f` or `let*` | `opt.and_then(f)` |
| Filter | `Option.filter f opt` | `opt.filter(pred)` |
| Default | `Option.value opt ~default` | `opt.unwrap_or(default)` |
