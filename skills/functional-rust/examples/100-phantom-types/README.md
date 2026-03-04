# 100: Phantom Types — Type-Safe Units

**Difficulty:** 3  **Level:** Intermediate

Prevent mixing meters with seconds at compile time — zero runtime overhead, pure type system enforcement.

## The Problem This Solves

Unit errors cause real disasters. NASA lost the Mars Climate Orbiter because one team used metric units and another used imperial. In code, `add(distance_m, time_s)` looks fine to the compiler if both are `f64` — but it's physically nonsense.

Runtime validation catches this too late. You want the compiler to refuse `meters(100.0) + seconds(5.0)` the same way it refuses `"hello" + 42`. Both are type errors — but without phantom types, the compiler can't see it.

The solution: tag each value with a marker type that tracks *what it represents*. The marker carries no data — it's purely for the type checker.

## The Intuition

A `Quantity<Meters>` and a `Quantity<Seconds>` are different types even though they both contain a single `f64`. The `Meters` and `Seconds` types are never stored — they're just type-level labels.

`PhantomData<T>` is Rust's way of saying "this struct is parameterized by T, but T doesn't appear in any stored field." Without it, the compiler would complain about unused type parameters.

The size test confirms zero overhead: `size_of::<Quantity<Meters>>() == size_of::<f64>()`.

## How It Works in Rust

```rust
use std::marker::PhantomData;
use std::ops::Add;

// The unit types — zero-sized markers, never constructed
pub struct Meters;
pub struct Seconds;

// A quantity tagged with its unit
#[derive(Debug, Clone, Copy)]
pub struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,  // zero size, tells compiler about the type param
}

impl<U> Quantity<U> {
    pub fn new(value: f64) -> Self {
        Quantity { value, _unit: PhantomData }
    }
}

// Add is only defined when BOTH sides have the SAME unit
impl<U> Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Quantity::new(self.value + rhs.value)
    }
}

pub fn meters(v: f64) -> Quantity<Meters> { Quantity::new(v) }
pub fn seconds(v: f64) -> Quantity<Seconds> { Quantity::new(v) }
```

This code compiles:
```rust
let total = meters(100.0) + meters(50.0);   // OK: same unit
```

This doesn't:
```rust
let bad = meters(100.0) + seconds(5.0);     // Compile error: type mismatch
```

The compiler catches the bug — no runtime check, no overhead.

## What This Unlocks

- **Unit-safe math libraries** — distances, angles, durations, currencies
- **State machines in types** — `Door<Locked>` vs `Door<Unlocked>` with different available methods
- **Validated newtypes** — `Email<Validated>` can only be created by a function that checks format

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom marker | Abstract type in signature: `type meters` | `PhantomData<Meters>` struct field |
| Runtime cost | Zero | Zero |
| Adding same units | Module functor or direct `+` | `impl Add for Quantity<U>` |
| Cross-unit safety | Module abstraction | Type mismatch at compile time |
| Alternative | N/A | Newtype wrapper (`struct Meters(f64)`) — simpler but less flexible |
