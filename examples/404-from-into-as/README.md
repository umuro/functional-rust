📖 **[View on hightechmind.io →](https://hightechmind.io/rust/404-from-into-as)**

---

# 404: From, Into, TryFrom, TryInto
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Type conversions are pervasive: temperatures between Celsius and Fahrenheit, integers between types, domain values from raw data. Ad-hoc conversion functions (`celsius_to_fahrenheit`, `as_kelvin`) don't compose and require memorizing function names. The `From`/`Into` trait pair standardizes infallible conversions: implement `From<A> for B` and get `Into<A>` on `B` for free via blanket impl. `TryFrom`/`TryInto` handle fallible conversions returning `Result`. This unification means all conversions use `.into()`, `.from()`, or `.try_into()` consistently.

`From`/`Into` power the entire `?` operator error conversion, `Into<Vec<u8>>` in network APIs, `From<String>` for `PathBuf`, and virtually every constructor pattern in `std`.

## Learning Outcomes

- Understand the `From`/`Into` blanket impl relationship: implementing `From<A> for B` gives `Into<B> for A` automatically
- Learn `TryFrom`/`TryInto` for fallible conversions with `type Error`
- See how temperature unit conversions demonstrate clean `From` chains
- Understand why `.into()` sometimes requires type annotation to resolve ambiguity
- Learn how `From` powers the `?` operator's error type conversion

## Rust Application

In `src/lib.rs`, `From<Celsius> for Fahrenheit`, `From<Fahrenheit> for Celsius`, and the Kelvin conversions form a complete temperature conversion graph. Each `From` implementation gives a corresponding `Into` for free. `PositiveInt` uses `TryFrom<i32>` with `type Error = String` to validate at construction time. The `?` operator uses `From<E1> for E2` to convert error types automatically when returning `Result<T, E2>` in a function.

## OCaml Approach

OCaml uses explicit conversion functions in modules: `Celsius.of_fahrenheit`, `Temperature.to_kelvin`. There is no equivalent of the `From`/`Into` blanket relationship — each conversion function is independent. OCaml's `Result.bind` and `let*` syntax handle fallible conversions. The `ppx_conv_func` library provides some standardization but OCaml has no universal conversion trait.

## Key Differences

1. **Blanket impl**: Rust's `Into` is automatically derived from `From`; OCaml requires implementing each conversion direction independently.
2. **Error conversion**: Rust's `?` operator uses `From<E1> for E2` for automatic error coercion; OCaml uses `Result.map_error` explicitly.
3. **Fallible variants**: Rust has separate `TryFrom`/`TryInto` for fallible conversions with `type Error`; OCaml uses `option` or `result` as return types on any function.
4. **Type inference**: Rust sometimes needs type annotation with `.into()` (`let f: Fahrenheit = c.into()`); OCaml's explicit function names make the target type clear.

## Exercises

1. **Color space conversion**: Implement `From<RgbColor>` for `HslColor` and `From<HslColor>` for `RgbColor` using the standard formulas. Write a round-trip test verifying that converting RGB → HSL → RGB returns the original within floating-point tolerance.
2. **Error conversion chain**: Create three error types `ParseError`, `IoError`, `AppError`. Implement `From<ParseError> for AppError` and `From<IoError> for AppError`. Write a function using `?` that combines a parse and an IO operation into a single `Result<T, AppError>`.
3. **Integer hierarchy**: Implement `From<i32>` for `BigInt(Vec<i32>)` and `TryFrom<BigInt>` for `i32` (failing when the BigInt doesn't fit). Show that `let big: BigInt = 42i32.into()` works, and `i32::try_from(big).unwrap_or(-1)` handles overflow.
