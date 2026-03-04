# 141: Singleton Types

**Difficulty:** 4  **Level:** Advanced

Types inhabited by exactly one value — used as compile-time tags, proof tokens, and phantom labels that prevent mixing incompatible values.

## The Problem This Solves

Sometimes the goal isn't to carry runtime data — it's to carry *compile-time information* that guides the type checker. Consider physical units: a `Meters` value and a `Seconds` value are both `f64` at runtime, but adding them is a bug. You want the compiler to reject `meters + seconds` without any runtime check.

Singleton types — types with exactly one possible value — are the tool. The type itself carries the information; the value is irrelevant. `struct Kg;` and `struct Lb;` are both zero-sized, both have exactly one value each (the `Kg` and `Lb` constructors), and they're distinct types the compiler will never confuse. Attach them as phantom type parameters to a generic wrapper and you get distinct `Measure<Kg>` and `Measure<Lb>` types that can't be accidentally swapped.

The same technique applies to type-level natural numbers (`Zero`, `Succ<N>`), boolean flags (`TrueType`, `FalseType`), and capability tokens (see the `AdminToken` pattern in example 148). All zero bytes at runtime, real distinctions at compile time.

## The Intuition

A singleton type is a zero-sized struct used as a phantom type tag: its only purpose is to be distinct from other tags so the compiler treats `Wrapper<Kg>` and `Wrapper<Lb>` as different types.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Singleton unit tags — zero bytes each
pub struct Kg;
pub struct Lb;
pub struct Meter;

// Generic measure type — phantom type prevents mixing units
#[derive(Debug, Clone, Copy)]
pub struct Measure<Unit> {
    pub value: f64,
    _unit: PhantomData<Unit>,  // zero-sized — just carries the type
}

impl<U> Measure<U> {
    pub fn new(value: f64) -> Self {
        Measure { value, _unit: PhantomData }
    }
}

// Explicit conversion functions — the only way to change the tag
impl Measure<Kg> {
    pub fn to_lb(self) -> Measure<Lb> {
        Measure::new(self.value * 2.20462)
    }
}

let weight: Measure<Kg> = Measure::new(70.0);
let in_lb = weight.to_lb();
// weight + in_lb  // ERROR: Measure<Kg> vs Measure<Lb> — the compiler catches this

// Type-level Peano naturals — encoding counts in types
pub struct Zero;
pub struct Succ<N>(PhantomData<N>);

pub type One   = Succ<Zero>;
pub type Two   = Succ<One>;

// Nat<N> carries both the type-level count (N) and the runtime value
pub struct Nat<N> { value: usize, _phantom: PhantomData<N> }

impl Nat<Zero> {
    pub fn zero() -> Self { Nat { value: 0, _phantom: PhantomData } }
}
impl<N> Nat<Succ<N>> {
    pub fn succ(prev: Nat<N>) -> Self { Nat { value: prev.value + 1, _phantom: PhantomData } }
}

// These two are different types — compiler won't mix them:
let one: Nat<One> = Nat::<One>::succ(Nat::<Zero>::zero());
let two: Nat<Two> = Nat::<Two>::succ(one);
```

## What This Unlocks

- **Unit-safe arithmetic** — `Measure<Kg>`, `Measure<Lb>`, `Measure<Meter>`: mixing units is a compile error, conversion is explicit.
- **Type-level counters and indices** — length-indexed vectors, size-checked operations where the size is tracked in the type.
- **Proof tokens** — "has been authenticated", "has been validated", "lock is held": compile-time capabilities that can't be forged.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom type tags | `type 'a t` with unused `'a` | `PhantomData<Unit>` field in struct |
| Zero-sized types | Abstract types / phantom modules | Unit structs (`struct Kg;`) |
| Type-level naturals | GADT / type-level encoding | `struct Zero; struct Succ<N>;` |
| Unit safety | Via abstract types in separate modules | Via phantom generic parameter |
