📖 **[View on hightechmind.io →](https://hightechmind.io/rust/294-custom-error-type)**

---

# 294: Custom Error Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Generic error strings (`String`, `&str`) lose information — callers cannot match on the error kind to handle different failures differently. Custom error enums document every possible failure mode in the type system, enabling exhaustive handling, machine-readable error codes, and structured error data. This is the standard approach in production Rust libraries and mirrors OCaml's algebraic error types.

## Learning Outcomes

- Define error types as enums with variants carrying relevant context data
- Implement `Display` for user-facing error messages and `Debug` for developer diagnostics
- Use `impl std::error::Error` to integrate with the Rust error ecosystem
- Recognize when struct variants (with named fields) vs tuple variants are appropriate

## Rust Application

A custom error enum makes every failure mode visible in the function signature:

```rust
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidNumber(String),
    OutOfRange { value: i64, min: i64, max: i64 },
    EmptyInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(s) => write!(f, "invalid number: '{}'", s),
            ParseError::OutOfRange { value, min, max } =>
                write!(f, "value {} out of range [{}, {}]", value, min, max),
            ParseError::EmptyInput => write!(f, "empty input"),
        }
    }
}
```

## OCaml Approach

OCaml uses polymorphic variants or regular variant types for errors, commonly with a single `error` type defined per module:

```ocaml
type parse_error =
  | InvalidNumber of string
  | OutOfRange of { value: int; min: int; max: int }
  | EmptyInput

let display_error = function
  | InvalidNumber s -> Printf.sprintf "invalid number: '%s'" s
  | OutOfRange {value; min; max} ->
    Printf.sprintf "value %d out of range [%d, %d]" value min max
  | EmptyInput -> "empty input"
```

## Key Differences

1. **Trait obligation**: Rust error types must implement `Display` and `Debug`; OCaml has no such requirement — any type can be an error.
2. **Ecosystem integration**: Implementing `std::error::Error` makes Rust errors compatible with `Box<dyn Error>`, `anyhow`, and `thiserror`.
3. **Structured data**: Both languages support carrying context data in error variants with field-carrying structs or tuples.
4. **Exhaustive matching**: Both Rust and OCaml require exhaustive match on error variants — adding a variant is a compile-time breaking change.

## Exercises

1. Define a `NetworkError` enum with variants for connection refused, timeout, and authentication failure — each carrying relevant context.
2. Implement a `ValidationError` type with variants for each field constraint violation and aggregate multiple validation failures.
3. Add a `source() -> Option<&dyn Error>` implementation to wrap a lower-level `ParseError` inside a higher-level `ConfigError`.
