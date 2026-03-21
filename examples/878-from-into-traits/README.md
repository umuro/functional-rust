📖 **[View on hightechmind.io →](https://hightechmind.io/rust/878-from-into-traits)**

---

# 878-from-into-traits — From/Into Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Type conversions are ubiquitous in systems programming: parsing strings, converting between unit systems, adapting error types. Rust's `From<T>` and `Into<T>` traits standardize these conversions. Implementing `From<A> for B` automatically provides `Into<B> for A` via a blanket implementation. The `?` operator uses `From` to convert error types in fallible functions. `TryFrom`/`TryInto` handle conversions that can fail. This design replaces the error-prone cast operators of C/C++ with explicit, nameable, testable conversion functions. OCaml handles conversions through explicit functions in module interfaces, with no universal conversion trait.

## Learning Outcomes

- Implement `From<T>` and understand how `Into<T>` comes for free
- Use `TryFrom<T>` for fallible conversions that return `Result`
- Recognize how the `?` operator leverages `From` for error type conversion
- Implement bidirectional conversions between temperature units
- Compare Rust's trait-based conversions with OCaml's explicit conversion functions

## Rust Application

The code implements `From<Celsius> for Fahrenheit` and `From<Fahrenheit> for Celsius`, enabling `Fahrenheit::from(Celsius(100.0))` and the equivalent `let f: Fahrenheit = Celsius(100.0).into()`. `TryFrom<&str> for Point` parses a string like `"(3, 4)"` into a `Point`, returning `Err(String)` on malformed input. The `?` operator in practice: a function returning `Result<T, AppError>` can use `?` on any operation whose error implements `From<OriginalError> for AppError`.

## OCaml Approach

OCaml uses explicit named functions for conversions: `celsius_to_fahrenheit: float -> float`, `fahrenheit_to_celsius: float -> float`. There is no equivalent to Rust's blanket `Into` implementation. Error conversion in OCaml uses `Result.map_error` or explicit `match` on the inner error and rewrapping. The `ppx_deriving.conv` library can auto-derive conversion functions for record types. OCaml's type system does not enforce a canonical conversion interface.

## Key Differences

1. **Bidirectional for free**: Implementing `From<A> for B` gives `Into<A>` on `B` automatically; OCaml requires both functions to be written explicitly.
2. **Error handling integration**: Rust's `?` uses `From` to convert errors; OCaml uses `Result.map_error` or explicit rebinding.
3. **Fallible conversions**: `TryFrom`/`TryInto` formalize fallible conversions; OCaml uses `option`/`result`-returning functions by convention.
4. **Coherence rules**: Rust's orphan rules restrict where `From` can be implemented; OCaml has no such restriction on conversion functions.

## Exercises

1. Implement `From<(f64, f64)>` for a `Vector2D` struct, and `From<Vector2D>` for `(f64, f64)` for round-trip conversion.
2. Create a `Color` enum with RGB and HSL variants, and implement `From<RgbColor>` for `HslColor` using the standard conversion formula.
3. Implement `TryFrom<&str>` for an `IpAddr` enum with `V4([u8; 4])` and `V6([u8; 16])` variants, parsing both formats.
