# 210: Iso Basics

**Difficulty:** 3  **Level:** Advanced

A bidirectional, lossless transformation — convert between representations and compose conversions freely.

## The Problem This Solves

Every program has representations: a temperature as `f64` Celsius, or as `f64` Fahrenheit, or as a newtype `Celsius(f64)`. Converting between them is easy. Converting back should recover the original exactly. But nothing in the type system enforces that guarantee — you could write a broken "roundtrip" and only discover it in production.

An `Iso` (isomorphism) makes the contract explicit: it pairs a forward function `get` with an inverse `reverse_get`, and the two are required to be true inverses. Reversing an Iso swaps the functions. Composing two Isos chains both directions. The type system tracks which representation you're in.

This matters most for serialisation (struct ↔ JSON ↔ bytes), unit conversions, and newtype wrappers. Once you have an Iso, you can lift any operation on one type to work on the other — no manual conversion at every call site.

## The Intuition

An Iso is like a perfect translation dictionary that works both ways. If you translate "cat" to French and back, you get "cat" again — not "feline" or "kitten". Lossless means no information is lost in either direction.

`get` is the forward direction. `reverse_get` is the backward direction. They must satisfy: `get(reverse_get(a)) == a` and `reverse_get(get(s)) == s`. Violating this isn't a compiler error, but it breaks every downstream operation that assumes the invariant.

Composing two Isos: if `A ↔ B` and `B ↔ C`, you get `A ↔ C`. The composed forward function is `get_bc(get_ab(a))`, and the composed reverse is `reverse_get_ab(reverse_get_bc(c))`. Both directions chain automatically.

## How It Works in Rust

```rust
struct Iso<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    reverse_get: Box<dyn Fn(&A) -> S>,
}

impl<S: 'static, A: 'static> Iso<S, A> {
    fn new(
        get: impl Fn(&S) -> A + 'static,
        reverse_get: impl Fn(&A) -> S + 'static,
    ) -> Self {
        Iso { get: Box::new(get), reverse_get: Box::new(reverse_get) }
    }

    // Reversing swaps the two function fields — the type flips to Iso<A, S>
    fn reverse(self) -> Iso<A, S> {
        Iso { get: self.reverse_get, reverse_get: self.get }
    }
}

// Celsius ↔ Fahrenheit
fn celsius_fahrenheit() -> Iso<f64, f64> {
    Iso::new(
        |c| c * 9.0 / 5.0 + 32.0,   // forward: C → F
        |f| (f - 32.0) * 5.0 / 9.0, // reverse: F → C
    )
}

let iso = celsius_fahrenheit();
let f = (iso.get)(&100.0);           // → 212.0
let c = (iso.reverse_get)(&212.0);   // → 100.0

// Newtype wrapper — Meters ↔ f64
struct Meters(f64);
let meters_iso: Iso<f64, Meters> = Iso::new(
    |&m| Meters(m),
    |Meters(v)| *v,
);
```

## What This Unlocks

- **Safe unit conversions** — wrap domain types like `Meters`, `Euros`, `Celsius` and convert without raw casts.
- **Serialisation pipelines** — `Struct ↔ DTO ↔ JSON bytes` as composed Isos; reversibility is guaranteed by construction.
- **Optic composition** — every Iso is also a valid Lens and Prism; compose with other optics to drill into nested structures.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Newtype | `type meters = Meters of float` | `struct Meters(f64)` (tuple struct) |
| Iso struct | Record with `get` and `reverse_get` | Struct with `Box<dyn Fn>` fields |
| Reversing | Creates new record, swaps fields | Consumes `self`, returns `Iso<A, S>` |
| Char handling | `String.get` is byte-indexed | `.chars()` is Unicode-aware |
| Composition | Simple function chaining | `'static` bounds needed for closures |
