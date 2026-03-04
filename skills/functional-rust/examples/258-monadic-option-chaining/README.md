# 258: Monadic Option Chaining

**Difficulty:** 2  **Level:** Intermediate

Chain fallible operations so the first `None` short-circuits the entire computation.

## The Problem This Solves

Many operations can fail to produce a value: looking up a key in a map, parsing a string, dividing by a number that might be zero, reading a field that might be absent. When you chain several such operations, a `None` at any step should propagate to the end without executing the remaining steps.

The naive approach is nested `match` expressions: match on step 1, and inside the `Some` branch, match on step 2, and inside that `Some` branch... Three steps deep and you have three levels of indentation. The structure obscures the logic.

The Option monad flattens this: each step either produces a value that flows to the next step, or produces `None` which silently skips everything remaining. This is the same short-circuit logic as `&&` for booleans, lifted to computations that produce values.

## The Intuition

Imagine a pipeline of transforms. Each transform says: "If I received a real value, I'll process it and pass along my result. If I received nothing, I'll pass nothing along immediately." The pipeline is written left-to-right; the first failure stops propagation without any if/else at each step.

`and_then` is this operation. It's OCaml's `>>=` (bind) for `Option`. `map` is `>>|` — transform the value if present, leave `None` alone. Together they're the two operations of the option monad.

Rust's `?` operator is syntactic sugar for the same thing: `expr?` means "unwrap `Some(x)` as `x`, or return `None` immediately". It makes monadic chaining look like sequential imperative code.

## How It Works in Rust

```rust
pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

pub fn safe_head(list: &[i32]) -> Option<i32> {
    list.first().copied()
}

// Style 1: and_then (bind) + map — mirrors OCaml's >>= and >>|
pub fn compute(lst: &[i32]) -> Option<i32> {
    safe_head(lst)
        .and_then(|x| safe_div(100, x))  // None if list empty or x == 0
        .map(|r| r * 2)                  // None propagates unchanged
}

// Style 2: ? operator — monadic chaining that reads like imperative code
pub fn compute_q(lst: &[i32]) -> Option<i32> {
    let x = safe_head(lst)?;      // returns None from function if absent
    let r = safe_div(100, x)?;    // returns None if division fails
    Some(r * 2)
}

// Style 3: explicit bind — shows what and_then desugars to
fn bind<T, U>(opt: Option<T>, f: impl FnOnce(T) -> Option<U>) -> Option<U> {
    match opt {
        None => None,
        Some(x) => f(x),  // only calls f if we have a value
    }
}
```

All three styles produce identical results. `and_then` is composable; `?` is readable; explicit `bind` is educational.

## What This Unlocks

- **Safe config lookups** — chain `get("host")?.parse::<IpAddr>()?` without nested unwrap panic risk.
- **Data pipeline validation** — `parse_date(s)?.validate()?.format()` — each step returns `Option` and the chain handles `None` automatically.
- **Replacing guard clauses** — replace four levels of `if let Some(x) = ...` with four `and_then` calls on one line.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | Custom `>>=` infix | `.and_then()` method |
| Map operator | Custom `>>|` infix | `.map()` method |
| Sugar | `let*` in `Option` monad context | `?` operator |
| `?` equivalent | None — no direct equivalent | `?` desugars to early `return None` |
| Stdlib bind | `Option.bind` (OCaml 4.08+) | `and_then` — available from the start |
