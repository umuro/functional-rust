[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 081 — Newtype Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Use single-field tuple structs (`struct Meters(f64)`) to give distinct types to values that share the same underlying representation. Prevent units-of-measure confusion (`Meters` vs `Seconds`), enforce distinct ID types (`UserId` vs `OrderId`), and add type-safe conversions between `Celsius` and `Fahrenheit` — all with zero runtime overhead.

## Learning Outcomes

- Define newtypes as single-field tuple structs in Rust
- Understand that newtypes are zero-cost: no runtime overhead vs the wrapped type
- Prevent accidental mixing of same-representation values at compile time
- Implement `From<Celsius> for Fahrenheit` for ergonomic `.into()` conversions
- Map Rust newtypes to OCaml single-constructor variants (`type meters = Meters of float`)
- Recognise when newtypes add safety versus when they add friction

## Rust Application

`struct Meters(f64)` and `struct Seconds(f64)` are distinct types despite identical representation. The `speed` function signature `fn speed(distance: Meters, time: Seconds)` makes it a compile error to pass a `Seconds` where `Meters` is expected. Deriving `Copy` is possible because `f64` is `Copy`. `UserId(u64)` and `OrderId(u64)` prevent accidental comparison or substitution between IDs of different domains. `From<Celsius> for Fahrenheit` lets callers write `let f: Fahrenheit = celsius_value.into()` without remembering the conversion formula.

## OCaml Approach

OCaml achieves the same safety with single-constructor variants: `type meters = Meters of float`. Pattern matching in function arguments (`let speed (Meters d) (Seconds t)`) destructures automatically. OCaml variants are slightly heavier than Rust newtypes in that they may not be unboxed in all contexts, but the safety guarantee is equivalent. The `to_fahrenheit` and `to_celsius` functions are explicit conversions — OCaml has no `From`/`Into` trait system, so conversions require naming.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Syntax | `struct Meters(f64)` | `type meters = Meters of float` |
| Runtime cost | Zero (transparent layout) | Possible boxing for `float` in records |
| Unwrapping | `.0` field access | Pattern match `(Meters m)` |
| Conversion | `impl From<A> for B` | Named function `to_celsius` |
| Ergonomics | `.into()` / `From::from` | Explicit call |
| Compile-time safety | Full (different types) | Full (different constructors) |

Both languages provide the same semantic guarantee: you cannot pass a `Meters` value where a `Seconds` is expected. Rust adds the `From`/`Into` trait infrastructure for ergonomic conversions; OCaml relies on explicit function naming.

## Exercises

1. Add a `Kilometers(f64)` newtype and implement `From<Kilometers> for Meters` and vice versa.
2. Create a `NonEmptyString(String)` newtype with a constructor `fn new(s: String) -> Option<NonEmptyString>` that returns `None` for empty strings.
3. Add `std::ops::Add<Meters> for Meters` so that two distances can be summed while still being `Meters`.
4. Implement `std::fmt::Display` for `Celsius` to print values as `"100°C"` and `Fahrenheit` as `"212°F"`.
5. In OCaml, define a `validated_email` newtype and a smart constructor `val make : string -> validated_email option`. Compare the pattern with Rust's equivalent approach.
