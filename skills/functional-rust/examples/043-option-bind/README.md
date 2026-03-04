# 043: Option Bind

**Difficulty:** 2  **Level:** Foundations

Chain multiple operations that each might fail — using `.and_then()` (also called "bind" or "flatMap").

## The Problem This Solves

You need to: parse a string as a number, divide that number, then check the result is positive. Each step can fail. With `.map()` alone, you'd end up with `Option<Option<i64>>` — an option wrapped inside another option. That's wrong.

`.and_then()` (known as "bind" in functional programming, "flatMap" in JavaScript/Kotlin) solves this. It takes a function that *itself* returns an `Option` and flattens the double-wrapping. If any step returns `None`, the whole chain short-circuits immediately. No nested `if let`, no pyramid of checks.

This is the key to clean error propagation in Rust: each step either produces a value or signals failure, and `and_then` threads those signals through without manual checking.

## The Intuition

In Python:
```python
def parse_div_positive(s, divisor):
    x = int(s) if s.isnumeric() else None
    if x is None: return None
    y = x // divisor if divisor != 0 else None
    if y is None: return None
    return y if y > 0 else None
```

In Rust with `and_then`:
```rust
fn parse_div_positive(s: &str, divisor: i64) -> Option<i64> {
    parse_int(s)
        .and_then(|x| safe_div(x, divisor))
        .and_then(|x| if x > 0 { Some(x) } else { None })
}
```

Same logic, no if-chains. Each `.and_then(|x| ...)` says: "if we have a value `x`, try this next step. If that step fails, return `None`."

**Why not just `.map()`?** If `safe_div` returns `Option<i64>`, then `.map(safe_div)` gives `Option<Option<i64>>`. That's wrong — you'd have to unwrap twice. `.and_then()` flattens it to `Option<i64>`.

## How It Works in Rust

```rust
fn parse_div_positive(s: &str, divisor: i64) -> Option<i64> {
    parse_int(s)                                   // Option<i64>
        .and_then(|x| safe_div(x, divisor))        // Option<i64> (flattened)
        .and_then(|x| if x > 0 { Some(x) } else { None })  // Option<i64>
}
```

**The `?` operator does the same thing**, with cleaner syntax in a function body:

```rust
fn parse_div_positive(s: &str, divisor: i64) -> Option<i64> {
    let x = parse_int(s)?;           // if None, return None from this function
    let y = safe_div(x, divisor)?;   // if None, return None
    if y > 0 { Some(y) } else { None }
}
```

`?` = "if this is `None`, immediately return `None` from the enclosing function." It's syntactic sugar for `and_then` chains, and it reads like normal sequential code.

**`or_else` — the fallback counterpart:**
```rust
None::<i64>.or_else(|| Some(42))  // → Some(42)
Some(1i64).or_else(|| Some(99))   // → Some(1)  (or_else is skipped)
```

`or_else` provides an alternative computation when the value is `None`. It's the "try the backup plan" operator.

## What This Unlocks

- **Validation pipelines** — chain parse → validate → transform, stopping at the first failure.
- **Nested lookups** — look up a key in one map, use the value to look up in another.
- **The `?` pattern** — once you understand `and_then`, `?` becomes intuitive in any `Option`-returning function.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind / flatMap | `Option.bind opt f` | `opt.and_then(\|x\| f(x))` |
| Short-circuit on None | Automatic with `bind` | Automatic with `and_then` / `?` |
| `?` operator | `let*` (monadic let) in recent OCaml | `?` in functions returning `Option<T>` |
| Fallback | `Option.first_some` / `\|\|` | `.or_else(\|\| Some(fallback))` |
| Nested Options | `Option.join` to flatten | `and_then` flattens automatically |
