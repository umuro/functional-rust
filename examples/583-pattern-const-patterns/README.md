📖 **[View on hightechmind.io →](https://hightechmind.io/rust/583-pattern-const-patterns)**

---

# 583: Const in Patterns

**Difficulty:** 2  **Level:** Beginner

Match against named constants and associated constants in `match` expressions, making arms self-documenting and refactor-safe.

## The Problem This Solves

Magic numbers in `match` arms are fragile and hard to read. When you write `match port { 80 => "HTTP", 443 => "HTTPS", ... }`, the meaning lives only in a comment, and if you later change `HTTP_PORT` from 80 to something else, you must hunt down every `match` that references it.

Named constants in `match` arms solve both problems: the name documents intent, and changing the constant's value automatically updates every pattern that references it. This is especially important for domain-specific codes, protocol ports, timeout values, and configuration constants that appear in multiple places.

## The Intuition

Rust allows `const` names in `match` patterns wherever a literal would work — for integers, booleans, chars, and other `PartialEq + Copy` types. The compiler replaces the constant with its value at compile time. Associated constants (`Type::CONST`) work the same way, letting you group related constants on a type rather than scattering them globally.

One important caveat: a bare identifier in a `match` arm is a *binding pattern*, not a constant. To use a constant, it must be either a `const` item, a path (`Module::CONST`), or an associated constant (`Struct::CONST`). Single-segment `const` names work directly.

## How It Works in Rust

**Top-level constants in patterns:**
```rust
const HTTP:  u16 = 80;
const HTTPS: u16 = 443;

fn describe_port(p: u16) -> &'static str {
    match p {
        HTTP  => "HTTP",
        HTTPS => "HTTPS",
        1..=1023 => "well-known",
        _    => "other",
    }
}
```
`HTTP` and `HTTPS` are treated as values to match against, not as variable bindings.

**Range patterns with constants:**
```rust
const MIN_AGE: u32 = 18;
const MAX_AGE: u32 = 65;

match age {
    0             => "newborn",
    1..=MIN_AGE   => "minor",
    MIN_AGE..=MAX_AGE => "adult",
    _             => "senior",
}
```
Constants can appear as range endpoints in `..=` patterns.

**Associated constants on a struct:**
```rust
struct Cfg;
impl Cfg {
    const TIMEOUT: u32 = 30;
}

match t {
    Cfg::TIMEOUT => "default",
    1..=10       => "fast",
    _            => "slow",
}
```
Path syntax (`Cfg::TIMEOUT`) distinguishes this from a binding pattern.

**Why not just use a variable?** A `let` binding in a match arm would *shadow* the outer variable, not compare against it. Only `const` items, paths, and literals act as value patterns.

## What This Unlocks

- **Refactor-safe match arms** — change the constant once, all patterns update automatically.
- **Self-documenting dispatch** — `HTTP => ...` reads like a specification, not a cryptic number.
- **Range patterns with semantic names** — `MIN_AGE..=MAX_AGE` expresses intent that `18..=65` doesn't.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Named constant in match | `let module M = struct let x = 5 end` then `M.x` in pattern | `const X: i32 = 5;` used directly in `match` |
| Bare identifier in match | Always a binding | Bare single-segment name: binding; `path::CONST` or `CONST`: value |
| Associated constants | Module-scoped values | `impl Type { const X: T = ...; }` |
| Range in pattern | `when` guard | `lo..=hi` range pattern directly in arm |
