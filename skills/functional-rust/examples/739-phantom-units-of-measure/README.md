# 739: Phantom Units of Measure

**Difficulty:** 4  **Level:** Expert

`Quantity<Meters>` and `Quantity<Feet>` are the same `f64` at runtime but incompatible types at compile time — adding them is a type error, converting is explicit, and the unit arithmetic is zero-cost.

## The Problem This Solves

Unit confusion is a real category of engineering disaster. The Mars Climate Orbiter was lost in 1999 because one module used imperial units and another used metric — a `f64` passed from one to the other with no type-level indication of what it measured. The values looked plausible; the crash was a complete surprise.

In everyday code the stakes are lower but the bugs are common: a function that takes a duration in milliseconds called with a value in seconds, a distance in feet added to a distance in metres. The compiler sees two `f64` values and is happy. The test suite misses it. The customer notices.

Phantom types solve this with zero runtime cost. `Quantity<Meters>` and `Quantity<Feet>` hold the same `f64` but are different types. The `Add` trait is only defined for matching units — `Quantity<U> + Quantity<U>`. Different units don't add. Conversion functions are explicit and named. F# has built-in units of measure; Rust achieves the same guarantee with phantom types.

## The Intuition

The unit marker (`Meters`, `Feet`, `Seconds`) is a zero-sized type used as a type parameter. At runtime, `Quantity<Meters>` is just an `f64`. At compile time, it's a distinct type from `Quantity<Feet>`.

`impl Add for Quantity<U>` is generic over `U` — adding two `Quantity<Meters>` values gives a `Quantity<Meters>`, and adding two `Quantity<Feet>` gives `Quantity<Feet>`. But there's no `impl Add<Quantity<Feet>> for Quantity<Meters>` — that's the intentional gap. Trying to add them fails to find an `Add` implementation and the compiler tells you clearly.

Derived quantities (velocity = distance / time) use specific `Div` implementations: `impl Div<Quantity<Seconds>> for Quantity<Meters>` produces `Quantity<MetersPerSecond>`. The type algebra mirrors dimensional analysis.

## How It Works in Rust

```rust
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

// Unit markers — zero bytes each
pub struct Meters;
pub struct Feet;
pub struct Seconds;
pub struct MetersPerSecond;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,  // zero bytes; carries unit in the type
}

impl<U> Quantity<U> {
    pub fn new(v: f64) -> Self { Quantity { value: v, _unit: PhantomData } }
    pub fn value(self) -> f64 { self.value }
}

// Same-unit addition — U matches U
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output { Quantity::new(self.value + rhs.value) }
}

// Dimensional analysis: Meters / Seconds = MetersPerSecond
impl Div<Quantity<Seconds>> for Quantity<Meters> {
    type Output = Quantity<MetersPerSecond>;
    fn div(self, rhs: Quantity<Seconds>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

// Convenience constructors
pub fn meters(v: f64)  -> Quantity<Meters>  { Quantity::new(v) }
pub fn feet(v: f64)    -> Quantity<Feet>    { Quantity::new(v) }
pub fn seconds(v: f64) -> Quantity<Seconds> { Quantity::new(v) }

// Explicit conversion — never implicit
pub fn feet_to_meters(f: Quantity<Feet>) -> Quantity<Meters> {
    Quantity::new(f.value() * 0.3048)
}

// ── Valid operations ───────────────────────────────────────────────────────────
let a = meters(100.0) + meters(50.0);   // Quantity<Meters> — fine
let speed = meters(1000.0) / seconds(10.0);  // Quantity<MetersPerSecond>

let f = feet(328.084);
let m = feet_to_meters(f);  // explicit conversion required

// ── Compile errors — unit mismatch ────────────────────────────────────────────
// meters(1.0) + feet(1.0);    // ERROR: Add<Quantity<Feet>> not implemented for Quantity<Meters>
// meters(1.0) + seconds(1.0); // ERROR: same
// let wrong: Quantity<Feet> = meters(5.0);  // ERROR: mismatched types

// Size: just f64 — zero overhead
assert_eq!(std::mem::size_of::<Quantity<Meters>>(), std::mem::size_of::<f64>());
```

## What This Unlocks

- **Physical quantity safety** — distances, durations, velocities, forces — each unit family is a distinct type; the compiler rejects nonsensical combinations.
- **Dimensional analysis in the type system** — define `Div` implementations that express d = v × t, v = d / t as trait impls; the type checker validates units automatically.
- **Zero runtime cost** — `PhantomData` is erased; `Quantity<Meters>` compiles to exactly the same machine code as a bare `f64`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Units of measure | F# has native UoM; OCaml needs phantom types similarly to Rust | `Quantity<Unit>` with `PhantomData<Unit>` — same idiom |
| Addition restriction | `add_same` function with explicit unit type | `impl<U> Add for Quantity<U>` — only same-unit addition |
| Dimensional arithmetic | Manual phantom type propagation | `impl Div<Quantity<Seconds>> for Quantity<Meters>` — explicit type algebra |
| Conversion | Explicit function | `feet_to_meters(f: Quantity<Feet>) -> Quantity<Meters>` — explicit, never implicit |
| Runtime cost | Zero (phantom type erased) | Zero — `PhantomData` adds no bytes; compiles to bare `f64` ops |
