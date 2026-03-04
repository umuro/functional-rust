# 404: From, Into, TryFrom, TryInto

**Difficulty:** 2  **Level:** Intermediate

The canonical way to convert between types in Rust — infallible and fallible, with `as` for low-level casts.

## The Problem This Solves

Type conversion is everywhere. Integers need to grow or shrink. Strings become error types. Domain objects convert to transport types. Without a standard approach, every library invents its own conversion methods (`to_i32()`, `parse_as_user()`, `from_raw_string()`) and composing them becomes an ecosystem of one-offs.

Rust standardizes this with `From<T>` and `Into<T>`. Implement `From<A> for B` and you automatically get `Into<B> for A` for free — the blanket impl in std handles it. This means APIs that accept `impl Into<T>` work with any type that has a `From` impl, without extra work from callers. The `?` operator uses `From` to convert errors automatically.

`as` exists for raw numeric casts. It's intentionally lossy: `300u16 as u8` silently truncates to `44`. Use it only when you understand and accept the truncation. For conversions that might fail, use `TryFrom`/`TryInto` — same ergonomics, but returns `Result`.

## The Intuition

`From<T>` is a declared, lossless conversion that can't fail; `as` is a raw cast that can truncate; implement `From` and `Into` comes for free.

## How It Works in Rust

```rust
// From<T> — infallible conversion
struct Wrapper(i32);
impl From<i32> for Wrapper {
    fn from(val: i32) -> Self { Wrapper(val) }
}

let w: Wrapper = Wrapper::from(42);
let w2: Wrapper = 42.into();  // Into is automatic from From impl

// String conversions — heavily uses From/Into
let s: String = String::from("hello");
let s2: String = "hello".into();

// Error conversion with ? — requires From<OrigError> for TargetError
fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>().map_err(|e| e.to_string())
}

// TryFrom — fallible conversion, returns Result
use std::convert::TryFrom;

let big: i32 = 1000;
let small = u8::try_from(big);  // Err: value too large
let ok = u8::try_from(200i32);  // Ok(200)

// as — raw numeric cast, potentially lossy
let x: u16 = 300;
let y: u8 = x as u8;   // 44 — silently truncates!
let f: f64 = 3.99;
let n: i32 = f as i32; // 3 — truncates, doesn't round
```

1. Implement `From<Source> for Target` — get `Into` for free.
2. Use `TryFrom`/`TryInto` when conversion can fail — returns `Result`.
3. Reserve `as` for numeric contexts where you understand and accept truncation.

## What This Unlocks

- **Ergonomic APIs**: Functions taking `impl Into<String>` accept `&str`, `String`, and anything else with a `From` impl.
- **Error propagation**: `?` uses `From` to convert errors automatically — `impl From<io::Error> for MyError` makes `?` work.
- **Zero boilerplate**: One `From` impl gives you `Into`, `TryInto` (via `TryFrom`), and compatibility with the entire ecosystem.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type conversion | Explicit coercion functions, no standard interface | `From`/`Into` traits, blanket impls |
| Numeric widening | Implicit in some cases | Always explicit: `x as i64` or `i64::from(x)` |
| Lossy cast | `Int32.of_int` (may truncate) | `as` keyword — intentionally lossy |
| Fallible conversion | Returns `option`/`result` by convention | `TryFrom`/`TryInto` return `Result` |
| Error conversion | Manual, per-library | `impl From<E1> for E2` + `?` automatic |
