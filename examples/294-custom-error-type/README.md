📖 **[View on hightechmind.io →](https://hightechmind.io/rust/294-custom-error-type)**

---

# 294: Custom Error Types

**Difficulty:** 2  **Level:** Intermediate

Define all the ways your function can fail as an enum — making errors part of your API, not an afterthought.

## The Problem This Solves

When a function can fail, beginners often use `String` as the error type: `Result<T, String>`. It works, but it has a serious drawback — the caller has no idea what errors to expect without reading your documentation. Did it fail because the input was empty? Because the number was out of range? Because of a network timeout? You can't tell from the type.

In Java and Python, exceptions work around this by having exception class hierarchies, but callers still have to read docs (or crash) to know what to catch. Rust's approach is different: you define a specific enum that lists every possible failure mode. Now the type signature itself documents what can go wrong. The compiler ensures you handle every case.

Custom error types also let you carry data with the error — not just "value out of range" but "value 200 is out of range [0, 100]". That's actionable information for the caller.

## The Intuition

You've seen Rust's built-in error types like `ParseIntError`. A custom error type is just your own version of that. In Python you might write:

```python
class ParseError(Exception): pass
class OutOfRangeError(Exception):
    def __init__(self, value, min, max): ...
```

In Rust, you use an enum instead of class hierarchies:

```rust
enum ParseError {
    InvalidNumber(String),
    OutOfRange { value: i64, min: i64, max: i64 },
    EmptyInput,
}
```

Each variant can carry different data. Pattern matching on the variant gives you the exact data for that failure case. And `impl Display` is what turns your error into a human-readable message — the equivalent of Python's `__str__`.

## How It Works in Rust

```rust
use std::fmt;

// The enum lists every way this operation can fail
#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidNumber(String),                          // carries the bad input
    OutOfRange { value: i64, min: i64, max: i64 }, // carries all three numbers
    EmptyInput,                                     // no data needed
}

// impl Display = human-readable message (for end users, log files)
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(s) =>
                write!(f, "invalid number: '{}'", s),
            ParseError::OutOfRange { value, min, max } =>
                write!(f, "value {} out of range [{}, {}]", value, min, max),
            ParseError::EmptyInput =>
                write!(f, "empty input"),
        }
    }
}

fn parse_bounded(s: &str, min: i64, max: i64) -> Result<i64, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);  // specific variant, no guessing
    }
    let n: i64 = s.parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;  // convert & propagate
    if n < min || n > max {
        return Err(ParseError::OutOfRange { value: n, min, max });
    }
    Ok(n)
}

// Callers can pattern-match on specific failure modes
match parse_bounded("999", 0, 100) {
    Ok(n) => println!("Got: {}", n),
    Err(ParseError::OutOfRange { value, min, max }) =>
        println!("{} is not in [{}, {}]", value, min, max),
    Err(ParseError::InvalidNumber(s)) =>
        println!("'{}' is not a number", s),
    Err(ParseError::EmptyInput) =>
        println!("please provide a value"),
}
```

`#[derive(Debug)]` gives you automatic `{:?}` formatting for debugging (just print the enum variant and data). `impl Display` is for the end-user-facing message. Both are useful, and both are separate concerns.

## What This Unlocks

- **API design** — your function signature becomes self-documenting: `Result<Config, ConfigError>` tells callers exactly what can go wrong
- **Error recovery** — callers can match on `OutOfRange` specifically to ask the user to try again, while treating `EmptyInput` differently
- **Composability with `?`** — combine with `impl From<OtherError> for YourError` to use `?` across your whole application

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error representation | Polymorphic variant or exception | Enum with named variants |
| Human-readable message | Ad-hoc `pp_error` functions | `impl Display` trait |
| Debug representation | Automatic with ppx | `#[derive(Debug)]` |
| Composing error types | Manual wrapping | `impl From<X> for Y` + `?` |
| Exhaustiveness | Compiler checks variant coverage | Compiler checks `match` arms |
