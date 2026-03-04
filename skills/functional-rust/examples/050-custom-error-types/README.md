# 050: Custom Error Types

**Difficulty:** 1  **Level:** Foundations

Design rich, self-documenting error types that carry context, compose cleanly, and integrate with the Rust ecosystem.

## The Problem This Solves

String errors are tempting: `Err("value too large".to_string())`. They're quick to write. But they're terrible to handle programmatically. How does a caller distinguish "too large" from "too small"? They'd have to parse the string. Catching specific errors becomes impossible. Logging loses structure. Testing becomes fragile.

In Python, the solution is subclassing: `class TooLargeError(ValueError): pass`. That works, but there's a global exception class hierarchy, inheritance is deep, and adding fields to your exception means overriding `__init__` and `__str__`. Java's checked exceptions are similar — verbose, and often abandoned in favor of unchecked `RuntimeException` because the ceremony is too high.

Rust encourages you to make error types that are *exactly* as rich as they need to be. An enum with named variants, each carrying structured data. Human-readable via `Display`. Machine-readable via `Debug`. Composable via `From`. Chainable via `std::error::Error::source()`. You get all of this with three trait implementations.

## The Intuition

Think of Rust error types as structured data, not strings. Instead of `Err("99 is not even")`, you return `Err(ValidationError::NotEven(99))`. The caller can match on `ValidationError::NotEven(v)` and decide what to do — retry with an even number, log the specific value, or surface a user-friendly message.

The key trio:
- `Debug` (derivable): machine-readable, for logs and tests
- `Display`: human-readable, for user messages
- `std::error::Error`: ecosystem integration — lets your error work with logging frameworks, `anyhow`, `thiserror`, and anything else that accepts `dyn Error`

`source()` in `std::error::Error` is Rust's equivalent of Python's `raise X from Y` — it chains errors so you can trace root causes.

## How It Works in Rust

```rust
// Rich validation error — structured, not stringly-typed
#[derive(Debug, PartialEq)]
enum ValidationError {
    TooSmall { value: i64, min: i64 },  // fields carry context
    TooLarge { value: i64, max: i64 },
    NotEven(i64),
}

// Human-readable messages
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::TooSmall { value, min } =>
                write!(f, "{} is too small (min: {})", value, min),
            ValidationError::TooLarge { value, max } =>
                write!(f, "{} is too large (max: {})", value, max),
            ValidationError::NotEven(v) =>
                write!(f, "{} is not even", v),
        }
    }
}

impl std::error::Error for ValidationError {}  // no methods needed for leaf errors

// Top-level error wraps sub-errors
#[derive(Debug)]
enum CalculatorError {
    InvalidInput(ParseIntError),
    DivisionByZero,
    Validation(ValidationError),
}

// source() enables error chaining — trace root cause
impl std::error::Error for CalculatorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CalculatorError::InvalidInput(e) => Some(e),
            CalculatorError::Validation(e)   => Some(e),
            _ => None,
        }
    }
}

// From impls let ? auto-convert sub-errors
impl From<ParseIntError> for CalculatorError {
    fn from(e: ParseIntError) -> Self { CalculatorError::InvalidInput(e) }
}
impl From<ValidationError> for CalculatorError {
    fn from(e: ValidationError) -> Self { CalculatorError::Validation(e) }
}

// Clean usage — both ? conversions happen automatically
fn parse_and_validate(s: &str) -> Result<i64, CalculatorError> {
    let x: i64 = s.trim().parse()?;  // ParseIntError → CalculatorError
    let v = validate(x)?;            // ValidationError → CalculatorError
    Ok(v)
}
```

## What This Unlocks

- **Structured logging:** Error variants with fields give log aggregators something to index. `NotEven(99)` is searchable; `"99 is not even"` is not.
- **Programmatic recovery:** Callers can match on specific variants and take different actions — retry, use a default, return a user-friendly message, or re-raise.
- **Ecosystem compatibility:** Implementing `std::error::Error` makes your type work with `anyhow::Error`, `Box<dyn Error>`, logging crates, and any framework that accepts standard errors.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error type definition | `type error = TooSmall of int * int \| ...` | `enum ValidationError { TooSmall { value, min }, ... }` |
| Human-readable | `let pp_error = ...` (manual) | `impl Display for MyError` |
| Machine-readable | `Printexc` / `show_error` | `#[derive(Debug)]` |
| Error chaining | No built-in, manual | `fn source() -> Option<&dyn Error>` |
| Ecosystem trait | No single standard | `std::error::Error` — universal |
| Wrapping sub-errors | Manual variant: `CalcError of parse_error` | `CalculatorError::InvalidInput(ParseIntError)` + `From` |
