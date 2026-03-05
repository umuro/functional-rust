📖 **[View on hightechmind.io →](https://hightechmind.io/rust/306-ok-or-patterns)**

---

# 306: ok_or and ok_or_else

**Difficulty:** 1  **Level:** Beginner

Convert `Option` to `Result` — the bridge between "missing value" and "actionable error."

## The Problem This Solves

`Option<T>` says "value or nothing." `Result<T, E>` says "value or error with a reason." These two types live in different worlds, but real code constantly moves between them. A `HashMap` lookup returns `Option`, but your function signature expects `Result`. You need to cross that bridge without losing information.

The naive approach — `match opt { Some(v) => Ok(v), None => Err(MyError) }` — is verbose and gets repetitive fast. Every function that wraps an `Option` source into a `Result` return type would need this boilerplate.

`ok_or` and `ok_or_else` collapse this into a single method call. They name the reason for absence, turning "maybe nothing" into "error with context" — exactly what callers need to handle failures gracefully.

## The Intuition

Think of `ok_or` as adding a label to `None`. When a lookup fails, you don't just get silence — you get `Err("key 'port' not found")`. The `_else` variant is the lazy version: it only builds the error message *if* it's actually needed, which matters when error construction is expensive (formatting, allocating, syscalls).

## How It Works in Rust

```rust
// ok_or: eager — error value always evaluated
let port: Result<&str, &str> = config.get("port").ok_or("port not configured");

// ok_or_else: lazy — closure only called when None
let port: Result<&str, String> = config.get("port")
    .ok_or_else(|| format!("key 'port' missing from {:?}", config.keys()));

// Chain with ? for ergonomic propagation
fn get_port(config: &HashMap<&str, &str>) -> Result<u16, String> {
    let s = config.get("port").ok_or_else(|| "port not set".to_string())?;
    s.parse::<u16>().map_err(|e| format!("invalid port: {}", e))
}
```

The reverse direction — `Result` → `Option` — uses `.ok()` (keeps the value, discards error) or `.err()` (keeps the error, discards value).

## What This Unlocks

- **Seamless integration** between Option-returning APIs (HashMap, Vec, str) and Result-based error propagation with `?`
- **Descriptive errors** instead of `None` — callers get context, not just silence
- **Lazy error construction** with `ok_or_else` avoids wasted allocation when the happy path is common

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Option → Result | `Option.to_result ~error:e v` | `opt.ok_or(e)` |
| Lazy error | Manual `match` | `opt.ok_or_else(f)` |
| Result → Option | `Result.to_option` | `result.ok()` |
| Error → Option | N/A (use pattern match) | `result.err()` |
| Common use | Monadic bind with `let*` | Chaining with `?` operator |
