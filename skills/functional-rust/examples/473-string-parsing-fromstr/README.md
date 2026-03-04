# 473: FromStr and parse()

**Difficulty:** 1  **Level:** Beginner

Turn any string into a typed value — and make your own types parseable with `FromStr`.

## The Problem This Solves

In Python you call `int("42")` or `float("3.14")`. These are built-in functions that can raise `ValueError`. In JavaScript you have `parseInt`, `parseFloat`, and `JSON.parse`. The problem with all of these: the error handling is loosely typed or easy to forget.

Rust's approach is different. Every type that can be parsed from a string implements the `FromStr` trait. You call `.parse::<T>()` on any `&str`, and you get back a `Result<T, T::Err>`. The type system forces you to handle the error. You can't accidentally ignore a parse failure.

More powerfully: you can implement `FromStr` for your own types. Once you do, `"255,0,0".parse::<Color>()` just works — the same ergonomic `.parse()` call, your own error type, fully integrated with `?` error propagation and iterator methods like `.ok().map(...)`.

## The Intuition

Think of `.parse::<T>()` as a universal conversion gateway. Any type that knows how to read itself from a string registers that knowledge with the `FromStr` trait. Then `.parse()` dispatches to the right implementation based on the type you ask for.

In Python, you'd write a `Color.from_string()` class method. In Rust, you implement `FromStr` — and now your type gets `.parse()` for free, with full `Result` error handling. The `?` operator makes errors flow up naturally.

The key mental model: `"text".parse::<YourType>()` returns `Result<YourType, YourType::Err>`. Handle with `match`, `unwrap()` (in tests/examples), or `?` in functions that return `Result`.

## How It Works in Rust

```rust
use std::str::FromStr;

// Built-in types already implement FromStr
let n: i32 = "42".parse().unwrap();          // type inferred
let n = "42".parse::<i32>().unwrap();        // explicit turbofish
let f = "3.14".parse::<f64>().unwrap();
let b = "true".parse::<bool>().unwrap();

// Error case — returns Err, not panic
match "-5".parse::<u32>() {
    Ok(v)  => println!("{}", v),
    Err(e) => println!("parse error: {}", e),
}

// Implement FromStr for your own type
#[derive(Debug, PartialEq)]
struct Color { r: u8, g: u8, b: u8 }

impl FromStr for Color {
    type Err = String;  // your error type
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("expected 3 parts, got {}", parts.len()));
        }
        let parse_u8 = |x: &str| x.trim().parse::<u8>()
            .map_err(|e| e.to_string());
        Ok(Color {
            r: parse_u8(parts[0])?,
            g: parse_u8(parts[1])?,
            b: parse_u8(parts[2])?,
        })
    }
}

let c: Color = "255,0,0".parse().unwrap();   // Color { r: 255, g: 0, b: 0 }

// Combine with iterator methods
let doubled = "21".parse::<i64>().ok().map(|n| n * 2); // Some(42)
```

## What This Unlocks

- **Config file parsing** — read `"8080".parse::<u16>()` for port numbers, `"true".parse::<bool>()` for flags.
- **CSV/DSV processing** — parse each field to its typed value in one chained expression.
- **CLI argument parsing** — libraries like `clap` use `FromStr` internally; implementing it for your types makes them first-class CLI arguments.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String to int | `int_of_string "42"` (raises on failure) | `"42".parse::<i32>()` → `Result` |
| String to float | `float_of_string "3.14"` | `"3.14".parse::<f64>()` → `Result` |
| Safe parse | `int_of_string_opt` → `option` | `.parse()` → `Result<T, Err>` |
| Custom type parsing | Manual function, no trait | Implement `FromStr` → `.parse()` works |
| Error type | Polymorphic exception | Associated type `Err` — user-defined |
| Chain on success | `Option.map` | `.ok().map(...)` or `?` operator |
