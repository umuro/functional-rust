# 317: Parse Error Handling

**Difficulty:** 2  **Level:** Intermediate

`.parse::<T>()` returns `Result` — learn the standard parse error types and how to give better error messages.

## The Problem This Solves

Parsing strings into typed values is one of the most common operations in real programs: reading config files, handling CLI arguments, deserializing CSV, processing user input. In languages with exceptions, parse failures surface at runtime as surprises. In Rust, every parse returns a `Result` — failure is part of the function's signature.

The standard library's `.parse::<T>()` method (implemented via the `FromStr` trait) gives you `ParseIntError`, `ParseFloatError`, and `ParseBoolError` out of the box, but their error messages are sometimes terse. The real skill is knowing how to chain parse errors into richer application errors that tell users what went wrong and why.

Implementing `FromStr` on your own types extends this model: your custom parser gets the same ergonomics as built-in parsing — callers use `.parse::<YourType>()` and handle failure in the same way they handle `parse::<i32>()`.

## The Intuition

Every call to `.parse()` is a bet that the string has the right format. `FromStr` is the trait that codifies that bet. The `Err` type tells you specifically how the bet was lost — not a generic "it failed" but a typed error you can pattern match on, display to users, or wrap in your own error type.

## How It Works in Rust

```rust
// Built-in parsing — returns Result
let n: Result<i32, _> = "42".parse();            // Ok(42)
let n: Result<i32, _> = "abc".parse();           // Err(ParseIntError)
let f: Result<f64, _> = "3.14".parse();          // Ok(3.14)

// Better error messages with map_err
let port: u16 = s.parse::<u16>()
    .map_err(|e| format!("invalid port '{}': {}", s, e))?;

// Implement FromStr for your own types
use std::str::FromStr;

struct PositiveInt(u64);

impl FromStr for PositiveInt {
    type Err = String;  // or your own error enum
    fn from_str(s: &str) -> Result<Self, String> {
        let n: i64 = s.parse().map_err(|_| format!("not a number: {s}"))?;
        if n <= 0 { return Err(format!("{n} is not positive")); }
        Ok(PositiveInt(n as u64))
    }
}

// Now callers use the same ergonomics:
let p: Result<PositiveInt, _> = "42".parse();
```

## What This Unlocks

- **Typed parse errors** — `ParseIntError` and friends carry kind information (empty, invalid digit, overflow) you can match on for specific handling
- **Custom parsers with standard ergonomics** — implement `FromStr` once and get `.parse()`, `str::parse()`, and `from_str()` for free
- **Chain-friendly error enrichment** — `map_err` wraps terse standard errors with application-level context before propagating with `?`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parse int | `int_of_string` (raises exception) | `s.parse::<i32>()` → `Result` |
| Parse float | `float_of_string` | `s.parse::<f64>()` → `Result` |
| Error type | Exception (dynamic) | `ParseIntError`, `ParseFloatError` (static) |
| Custom parsing | Manual recursive descent | `impl FromStr` |
| Error enrichment | `try ... with` + re-raise | `map_err(|e| format!("context: {}", e))` |
