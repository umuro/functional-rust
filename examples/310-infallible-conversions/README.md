📖 **[View on hightechmind.io →](https://hightechmind.io/rust/310-infallible-conversions)**

---

# 310: Infallible Conversions

**Difficulty:** 3  **Level:** Advanced

`std::convert::Infallible` — the error type for conversions that cannot fail, and how `From` and `TryFrom` relate.

## The Problem This Solves

Rust's conversion traits split into two families: `From`/`Into` for conversions that always succeed, and `TryFrom`/`TryInto` for conversions that might fail. This is clean in principle, but breaks down when you need to write generic code that works with both. A generic function that accepts `TryFrom<T>` cannot accept `From<T>` implementations without a separate constraint — even though `From` implies infallibility.

The solution is `Infallible`. Every `From<T> for U` implementation automatically gives you `TryFrom<T, Error = Infallible> for U` via a blanket impl in the standard library. This unifies both families: `TryFrom` is the general form, `From` is the special case where the error type is `Infallible`. Generic code can now accept `TryFrom<T>` and work for both fallible and infallible conversions.

The practical benefit extends to API design. When you're implementing a trait that requires `TryFrom` (because it's part of a generic interface), but your specific type's conversion truly cannot fail, you can implement `From` and get `TryFrom` for free — with `Infallible` as the documented proof that failure is impossible.

## The Intuition

`Infallible` is an enum with zero variants. You cannot construct a value of type `Infallible`. So `Result<T, Infallible>` is really just `T` in disguise — the `Err` variant can never exist, and `.unwrap()` can never panic.

Think of it as a type-level proof: "I claim this conversion cannot fail, and here's the evidence — an error type with no values."

## How It Works in Rust

```rust
use std::convert::{Infallible, TryFrom};

// From<u8> for u32 is infallible — u8 always fits in u32
let n: u32 = u32::from(255u8); // never fails

// The blanket impl gives us TryFrom for free:
let r: Result<u32, Infallible> = u32::try_from(255u8);
// r is always Ok — Infallible proves it

// Your own type: From gives you TryFrom automatically
struct Meters(f64);
impl From<f64> for Meters {
    fn from(v: f64) -> Self { Meters(v) }
}
// Now Meters::try_from(1.5f64) works with Error = Infallible

// Generic code works with both:
fn convert<T, U: TryFrom<T, Error = Infallible>>(val: T) -> U {
    U::try_from(val).unwrap() // safe — Infallible proves no panic
}
```

## What This Unlocks

- **Unified generic APIs** — write functions accepting `TryFrom<T>` and handle both infallible and fallible converters without code duplication
- **Self-documenting contracts** — `Error = Infallible` in a trait impl is a machine-checked promise that the conversion cannot fail
- **Free `TryFrom` impls** — implement `From` and get `TryFrom` for free, satisfying trait bounds that require the fallible API

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Infallible conversion | `int_of_char` (total function) | `From<T>` / `Into<T>` |
| Fallible conversion | `int_of_string` raises exception | `TryFrom<T, Error=E>` |
| Infallible marker | N/A (type system doesn't track) | `std::convert::Infallible` |
| Generic over both | Manual dispatch | `TryFrom<T, Error=Infallible>` |
| Zero-variant type | `type void = \|` (empty variant) | `enum Infallible {}` |
