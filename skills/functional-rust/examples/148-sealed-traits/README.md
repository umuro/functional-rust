# 148: Sealed Traits

**Difficulty:** 3  **Level:** Intermediate

A public trait that downstream crates cannot implement — you control the closed set of types forever.

## The Problem This Solves

Sometimes you want a public trait that users of your library can *use* but cannot *implement*. This matters for stability guarantees: if you publish a trait and anyone can implement it, adding a new method to that trait in the next release is a breaking change — every external `impl` would fail to compile. If you seal the trait, you own all implementations and can add methods freely.

It also matters for correctness. Some traits carry invariants that must hold for all implementations. If an external crate can add a rogue implementation, those invariants can be violated. Sealing the trait means you've verified every implementation in your codebase.

The Rust language doesn't have a built-in `sealed` keyword, but the pattern is simple and idiomatic: define a private supertrait in a private module. External code can't name `private::Sealed`, so it can't write `impl private::Sealed for TheirType`. Since your public trait requires `private::Sealed` as a supertrait, external code can't implement your trait either. It's a privacy trick, but it works perfectly and the compiler enforces it.

## The Intuition

Put a private supertrait behind a private module — external code can't see it, so it can't implement it, so it can't implement your public trait.

## How It Works in Rust

```rust
// private module — not pub, not accessible from outside this crate
mod private {
    pub trait Sealed {}  // accessible inside this crate; not from outside
}

// Public trait — but requires the private Sealed supertrait
pub trait Shape: private::Sealed {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
}

// Only types in THIS crate can implement Shape,
// because only they can see and implement private::Sealed.

pub struct Circle { pub radius: f64 }
pub struct Rectangle { pub width: f64, pub height: f64 }

// Step 1: Seal the type (in this crate only)
impl private::Sealed for Circle {}
impl private::Sealed for Rectangle {}

// Step 2: Implement the public trait
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    fn name(&self) -> &'static str { "circle" }
}

// From an external crate, this would fail:
// struct MyShape;
// impl crate::private::Sealed for MyShape {}  // ERROR: module `private` is private
// impl crate::Shape for MyShape { ... }       // ERROR: missing Sealed bound

// Generic functions work fine — Shape is still public and usable
fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

## What This Unlocks

- **Stable library APIs** — add methods to a sealed trait without breaking downstream crates (since there are no external impls to break).
- **Invariant enforcement** — every implementation is in your codebase where you can audit it.
- **Capability tokens** — the `AdminToken` variant: a type only your crate can construct, passed as proof to gate privileged operations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sealed module interface | Abstract types / restricted module visibility | Private supertrait in private module |
| Adding methods later | Safe if module is abstract | Safe if trait is sealed |
| External implementors | Prevented by module system | Prevented by private supertrait |
| Capability tokens | Abstract tokens in restricted modules | Private-field unit struct, private constructor |
