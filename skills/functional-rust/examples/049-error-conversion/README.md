# 049: Error Conversion

**Difficulty:** 1  **Level:** Foundations

Combine functions with different error types using `From<E>` — so `?` can auto-convert them.

## The Problem This Solves

Real applications call functions from multiple sources: your own code, standard library functions, third-party crates. Each returns a different error type. A file function returns `std::io::Error`. A JSON parser returns `serde_json::Error`. Your own validation code returns `MyValidationError`.

When you want to use `?` to propagate all of these from a single function, there's a type mismatch — your function returns `Result<T, AppError>`, but you're calling functions that return `Result<T, ParseIntError>`, `Result<T, IoError>`, etc. You need conversions.

In Python, this isn't an issue — all exceptions share a common base class and propagate freely. But that also means you have no idea *which* exceptions a function might raise unless you read every line of code it calls. Rust's `From` trait is the explicit, opt-in version: you decide which error types can convert to your `AppError`, you implement the conversions, and then `?` uses them automatically.

## The Intuition

`From<E>` is a trait that says: "my type can be created from `E`." Once you implement `From<ParseError> for AppError`, the `?` operator knows how to convert a `ParseError` into an `AppError` automatically — no manual `.map_err()` needed at each call site.

Think of it like Python's exception hierarchy, but opt-in and explicit. In Python, `ValueError` is a `Exception` because of inheritance — implicit. In Rust, `AppError` can be created from `ParseError` because you explicitly wrote `From<ParseError> for AppError` — explicit, composable, documented.

The key insight: **`?` on a `Result<T, E>` in a function returning `Result<T, AppError>` calls `AppError::from(e)` automatically.** If you've implemented `From<E> for AppError`, it just works.

## How It Works in Rust

```rust
// Your application's unified error type
#[derive(Debug)]
enum AppError {
    Parse(ParseError),
    Math(MathError),
    Io(String),
}

// Tell Rust: ParseError can become AppError
impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self { AppError::Parse(e) }
}

// Tell Rust: MathError can become AppError
impl From<MathError> for AppError {
    fn from(e: MathError) -> Self { AppError::Math(e) }
}

// These return different error types:
fn parse_positive(s: &str) -> Result<i64, ParseError> { ... }
fn reciprocal(x: i64) -> Result<f64, MathError> { ... }

// Now you can use ? on both — conversions happen automatically!
fn pipeline(s: &str) -> Result<f64, AppError> {
    let n = parse_positive(s)?;   // ParseError → AppError::Parse via From
    let r = reciprocal(n)?;       // MathError → AppError::Math via From
    Ok(r * 100.0)
}

// map_err: manual conversion when you don't have From
let result: Result<i64, AppError> =
    "42".parse::<i64>()
        .map_err(|e| AppError::Io(e.to_string()));  // explicit conversion

// Into is the mirror of From — once you implement From, Into is free
let pe = ParseError("bad input".to_string());
let ae: AppError = pe.into();  // calls From<ParseError> for AppError
```

## What This Unlocks

- **Unified error handling:** A single `AppError` enum can wrap errors from all your dependencies. One match arm handles each source.
- **`?` across boundaries:** Call standard library I/O functions, parsing functions, and your own validation in the same function body, propagating with `?` throughout.
- **Error middleware:** The `From` implementation is where you add context, log, or transform errors before they join your application's error hierarchy.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error unification | Manual wrapping or polymorphic variants | `enum AppError` + `From<E>` impls |
| Auto-conversion | Not built-in, use explicit wrapping | `?` calls `From::from()` automatically |
| Manual conversion | `Result.map_error` | `.map_err(\|e\| ...)` |
| Symmetric trait | N/A | `Into<T>` is auto-derived from `From<T>` |
| Std error types | `Stdlib` exceptions are all one type | Each module has own error type; `From` bridges them |
| Conversion cost | None (OCaml GC manages) | Zero-cost (Rust move semantics, no allocation) |
