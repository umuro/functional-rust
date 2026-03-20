[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 084 — From and Into Traits

## Problem Statement

Implement Rust's `From` and `TryFrom` traits for type conversions: bidirectional temperature conversion between `Celsius` and `Fahrenheit`, parsing a `Color` from `&str` with `TryFrom`, and validating a raw user record into a typed `User`. Compare with OCaml's explicit named conversion functions.

## Learning Outcomes

- Implement `From<A> for B` and gain `Into<B> for A` automatically
- Use `TryFrom<&str>` when conversion can fail, returning `Result<Self, Self::Error>`
- Chain `map_err` and `?` in `TryFrom` implementations
- Understand the blanket impl: `impl<T, U: From<T>> Into<T> for U`
- Use `.into()` at call sites for ergonomic type conversion
- Map Rust's trait-based conversion system to OCaml's explicit function naming

## Rust Application

`impl From<Celsius> for Fahrenheit` provides the formula once; callers write `let f: Fahrenheit = celsius_val.into()`. `TryFrom<&str> for Color` returns `Result<Color, String>` — the `type Error = String` associated type must be declared. `TryFrom<RawUser> for User` parses the `age` field with `raw.age.parse().map_err(|_| "Invalid age".to_string())?` and validates the email. The blanket impl means implementing `From` automatically provides `Into` for free — you only need to write `From`.

## OCaml Approach

OCaml uses plain functions: `fahrenheit_of_celsius`, `celsius_of_fahrenheit`, `color_of_string`, `user_of_raw`. There is no trait system to unify these under a single interface. Code that needs to be generic over conversions must take the conversion function as a parameter. The result type mirrors Rust: `Ok` and `Error` are standard OCaml result constructors, making `Result.bind` chains natural.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Infallible | `impl From<A> for B` | `b_of_a : a -> b` function |
| Fallible | `impl TryFrom<A> for B` | `b_of_a : a -> ('b, err) result` |
| Ergonomics | `.into()` call | Explicit function call |
| Generic over conversion | Trait bound `From<A>` | Higher-order function parameter |
| Auto-blanket | `Into` from `From` | Manual |
| Code reuse | One `From` impl for all call sites | One function, explicit at each call |

The `From`/`Into` system is one of Rust's most pervasive patterns. Standard library types extensively use it: `String::from("hello")`, `Vec::from([1, 2, 3])`, error propagation with `?`. Implementing `From` for your types integrates them into this ecosystem.

## Exercises

1. Add `impl From<i32> for Color` that maps `0 → Red`, `1 → Green`, `2 → Blue` and panics otherwise. Then add `TryFrom<i32>` that returns `Err` instead of panicking.
2. Implement `From<Vec<(String, i32)>>` for a `HashMap<String, i32>`.
3. Write a `Validated<T>` newtype that wraps `T` and implement `TryFrom<String> for Validated<Email>` where `Email` is another newtype.
4. Create a conversion chain: `RawConfig` → `ParsedConfig` → `ValidatedConfig`, each step using `TryFrom`.
5. In OCaml, write a `convert` functor `Convert(S : sig type t end)(D : sig type t val of_s : S.t -> D.t end)` and show how it compares to Rust's `impl From<S> for D`.
