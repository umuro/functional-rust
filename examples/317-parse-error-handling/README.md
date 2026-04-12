📖 **[View on hightechmind.io →](https://hightechmind.io/rust/317-parse-error-handling)**

---

# 317: Parse Error Handling
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Parsing user-provided strings is the entry point for most validation errors. The standard library's `str::parse::<T>()` returns `Result<T, <T as FromStr>::Error>`, but the default error messages are often too vague. Implementing `FromStr` for custom types with detailed error enums provides precise, informative error messages and integrates with the standard `parse()` interface. This is the "type-safe parsing" pattern used throughout production Rust code.

## Learning Outcomes

- Implement `FromStr` for a custom type to enable `.parse::<MyType>()` syntax
- Define detailed parse error enums with `Empty`, `InvalidFormat`, and `OutOfRange` variants
- Return specific error variants with context rather than generic string errors
- Use `FromStr` to integrate with the standard library's parsing infrastructure

## Rust Application

`impl FromStr` provides the `parse()` interface for custom types:

```rust
use std::str::FromStr;

impl FromStr for PositiveNumber {
    type Err = ParsePositiveError;

    fn from_str(s: &str) -> Result<Self, ParsePositiveError> {
        if s.is_empty() { return Err(ParsePositiveError::Empty); }
        let n: i64 = s.parse().map_err(|_| ParsePositiveError::InvalidNumber(s.to_string()))?;
        if n <= 0 { return Err(ParsePositiveError::NotPositive(n)); }
        Ok(PositiveNumber(n as u64))
    }
}

// Now: "42".parse::<PositiveNumber>() -> Ok(PositiveNumber(42))
// "0".parse::<PositiveNumber>()  -> Err(NotPositive(0))
```

## OCaml Approach

OCaml uses manual parsing functions rather than a standard `parse()` interface. The idiomatic approach is a `of_string` function returning `option` or `result`:

```ocaml
let positive_of_string s =
  if String.length s = 0 then Error `Empty
  else match int_of_string_opt s with
  | None -> Error (`InvalidNumber s)
  | Some n when n <= 0 -> Error (`NotPositive n)
  | Some n -> Ok n
```

## Key Differences

1. **Standard interface**: Rust's `FromStr` is a standard trait — implementing it gives `str::parse()` syntax for free; OCaml requires custom `of_string` functions.
2. **Error type**: `type Err = ParsePositiveError` makes the error type explicit in the trait; OCaml's return type carries it but without a standard name.
3. **Ecosystem integration**: `FromStr` integrates with `structopt`/`clap` for CLI argument parsing, `serde` for deserialization, and `reqwest` for header value parsing.
4. **Blanket impls**: All primitive types implement `FromStr` in Rust's standard library; OCaml provides `*_of_string` functions for primitives.

## Exercises

1. Implement `FromStr` for an `IpAddress` type that parses `"x.x.x.x"` notation, with specific error variants for wrong format, invalid octets, and out-of-range values.
2. Add `Display` and `std::error::Error` implementations to your `ParsePositiveError` type.
3. Use the `#[derive(Debug)]` and implemented `FromStr` together with `clap` to parse a custom CLI argument type.
