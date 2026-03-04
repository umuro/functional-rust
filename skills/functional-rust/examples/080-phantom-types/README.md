# 080: Phantom Types

**Difficulty:** 3  **Level:** Advanced

Attach compile-time type information to values without any runtime cost using `PhantomData`.

## The Problem This Solves

You have a `Quantity` that holds a `f64`. Is it metres? Seconds? Kilograms? At runtime it's just a number — you can accidentally divide a distance by another distance and get a "speed" that's dimensionally wrong, with no error.

You have a `Door`. Can you walk through it? Depends on whether it's locked. Without types, "check if locked" is a runtime check that you might forget.

You have an `Email` string. Has it been validated? Without types, you might call `send(email)` on an unvalidated address.

Phantom types solve all three: they attach extra type-level information (the *unit*, the *state*, the *validation status*) to a value. That information exists only at compile time — zero runtime bytes — and the compiler uses it to allow or forbid certain operations.

## The Intuition

A phantom type is a type parameter that appears in the struct's generic signature but not in any actual field. It's a label the compiler tracks without storing anything.

```rust
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,  // zero bytes, pure compile-time marker
}
```

`Quantity<Meters>` and `Quantity<Seconds>` are different types. You can add metres to metres (same `Unit`), but not metres to seconds — the types don't match, the compiler refuses.

## How It Works in Rust

```rust
use std::marker::PhantomData;

struct Meters; struct Seconds; struct MetersPerSecond;

#[derive(Clone, Copy)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,  // PhantomData<U> tells the compiler about U
}

// Same-unit addition — only works when both sides have the same Unit:
impl<U> std::ops::Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Quantity::new(self.value + rhs.value) }
}

// Type-checked unit conversion:
fn speed(d: Quantity<Meters>, t: Quantity<Seconds>) -> Quantity<MetersPerSecond> {
    Quantity::new(d.value / t.value)
}

// State machine with phantom types:
struct Door<State> { name: String, _state: PhantomData<State> }

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> { Door { name: self.name, _state: PhantomData } }
    fn walk_through(&self) -> String { format!("Walked through {}", self.name) }
    // walk_through only exists on Door<Unlocked>
}
impl Door<Locked> {
    fn unlock(self) -> Door<Unlocked> { Door { name: self.name, _state: PhantomData } }
    // No walk_through here — compiler prevents walking through locked doors
}
```

`PhantomData<T>` is required because Rust's borrow checker needs to know about type parameters that aren't used in fields. Without it, the compiler would reject `struct Quantity<Unit>` with "type parameter `Unit` is never used."

## What This Unlocks

- **Unit tracking** — prevent mixing incompatible physical units at compile time; used in aerospace, scientific, and financial code.
- **Type-safe state machines** — model login flows, connection lifecycles, workflow stages where invalid transitions are compile errors.
- **Validated data wrappers** — ensure only validated/sanitised values reach sensitive functions; `Email<Validated>` can call `send()`, `Email<Unvalidated>` cannot.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom marker | Implicit type parameter | `PhantomData<T>` field required |
| Runtime cost | Zero | Zero |
| Method restriction | Module abstraction / abstract types | Selective `impl` blocks per phantom state |
| State transitions | Return different phantom type | Consume `self`, return new type |
| Error message | Type mismatch at call site | "method not found" or type mismatch |
| Enforcement mechanism | Abstract module signature | `impl<State>` specificity |
