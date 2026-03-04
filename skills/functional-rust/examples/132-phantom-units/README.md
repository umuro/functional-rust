# 132: Phantom Units of Measure

**Difficulty:** ⭐⭐  **Level:** Intermediate

Tag numeric values with their unit of measure so you can't accidentally add metres to feet or pass a duration where a distance is expected.

## The Problem This Solves

The Mars Climate Orbiter was lost in 1999 because one module sent thruster data in pound-force-seconds while another expected newton-seconds. Both were just floating-point numbers. The compiler had no idea they were incompatible. Total cost: $327 million.

This class of bug is embarrassingly common in everyday code too: passing a price in cents where a price in dollars is expected, mixing up pixels and points in a UI, confusing seconds and milliseconds in a deadline computation. All of these are just `f64` or `i64` at the type level. The names are only in variable names and doc comments — the compiler can't check them.

Phantom types give each quantity a unit tag. `Quantity<Meters>` and `Quantity<Feet>` are different types wrapping the same `f64`. You can add two `Quantity<Meters>` values together (same unit), but you cannot add `Quantity<Meters>` to `Quantity<Feet>` — the compiler rejects it. Physics relationships like "distance / time = speed" are expressed as specific `impl` blocks that produce the right output unit.

## The Intuition

A phantom type is a type parameter that exists in the type signature but isn't stored anywhere in memory. `struct Quantity<Unit> { value: f64, _unit: PhantomData<Unit> }` stores exactly one `f64`. The `Unit` parameter — `Meters`, `Seconds`, `Kilograms` — is just a label carried in the type. At runtime, a `Quantity<Meters>` and a `Quantity<Seconds>` are identical in memory. But to the compiler, they're completely different types.

The unit-aware arithmetic is implemented as specific trait impls. You write `impl Add for Quantity<Meters>` (same-unit addition is fine), and `impl Div<Quantity<Seconds>> for Quantity<Meters>` to say "distance divided by time gives speed." The compiler checks units at every operation. If you try to add metres and seconds, there's no `impl Add<Quantity<Seconds>> for Quantity<Meters>` — compile error.

## How It Works in Rust

```rust
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul};

// Unit markers — empty structs, just type labels
struct Meters;
struct Seconds;
struct Kilograms;
struct MetersPerSecond;  // a derived unit

// The core wrapper — value stored, unit is just a phantom tag
#[derive(Debug, Clone, Copy)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,  // zero bytes — pure type-level information
}

impl<U> Quantity<U> {
    fn new(value: f64) -> Self { Quantity { value, _unit: PhantomData } }
    fn value(&self) -> f64 { self.value }
}

// Same-unit addition: only compiles when both operands have the same unit U
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output { Quantity::new(self.value + rhs.value) }
}

// Physics: metres ÷ seconds = metres/second
// This impl only exists for this specific combination of units
impl Div<Quantity<Seconds>> for Quantity<Meters> {
    type Output = Quantity<MetersPerSecond>;
    fn div(self, rhs: Quantity<Seconds>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

// Physics: metres/second × seconds = metres
impl Mul<Quantity<Seconds>> for Quantity<MetersPerSecond> {
    type Output = Quantity<Meters>;
    fn mul(self, rhs: Quantity<Seconds>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}
```

Usage:
```rust
let d1: Quantity<Meters>  = Quantity::new(100.0);
let d2: Quantity<Meters>  = Quantity::new(50.0);
let total = d1 + d2;  // Quantity<Meters> ✓

let t: Quantity<Seconds> = Quantity::new(15.0);
let speed = total / t;  // Quantity<MetersPerSecond> — unit derived automatically

// let bad = d1 + t;  // compile error: no impl Add<Quantity<Seconds>> for Quantity<Meters>
```

Unit conversion:
```rust
trait ConvertTo<Target> { fn convert(self) -> Quantity<Target>; }

impl ConvertTo<Kilometers> for Quantity<Meters> {
    fn convert(self) -> Quantity<Kilometers> { Quantity::new(self.value / 1000.0) }
}
```

## What This Unlocks

- **Financial arithmetic** — `Money<USD>` and `Money<EUR>` can't be accidentally added; conversions must be explicit and typed.
- **Embedded sensor data** — `Reading<Celsius>` and `Reading<Fahrenheit>` prevent silently wrong temperature comparisons across sensor types.
- **Graphics/UI coordinates** — `Point<ScreenPixels>` vs `Point<LogicalPoints>` catches HiDPI coordinate bugs that are notoriously hard to debug at runtime.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Unit phantom | `type 'unit quantity = { value: float }` — parameter is phantom | `struct Quantity<Unit> { value: f64, _unit: PhantomData<Unit> }` |
| Same-unit add | Type annotation `(a : 'u quantity) -> (b : 'u quantity) -> 'u quantity` | `impl<U> Add for Quantity<U>` — any same-unit pair |
| Unit arithmetic | Explicit type annotation at call site | Specific `impl Div<Quantity<Seconds>> for Quantity<Meters>` |
| Conversion | Explicit function with new phantom | Trait `ConvertTo<Target>` — explicit, type-safe |
