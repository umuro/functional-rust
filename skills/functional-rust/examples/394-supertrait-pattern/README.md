# 394: Supertrait Pattern

**Difficulty:** 2  **Level:** Intermediate

Declare that implementing one trait requires implementing others — build capability hierarchies.

## The Problem This Solves

You're designing a `LabeledShape` trait that needs to call `self.area()` and `self.perimeter()` in its default methods. But those methods live on `Shape`. If you don't declare the relationship, the compiler doesn't know `LabeledShape` types have `area()` — you can't use it in the default implementations, and callers can't rely on it.

Without supertraits, every function that needs both behaviors must duplicate the bound: `fn describe<T: Shape + Display + LabeledShape>(...)`. The bounds pile up, become verbose, and drift out of sync when requirements change.

Supertraits let you declare intent: "`LabeledShape` requires `Shape`." Implementors must implement both. Default methods in `LabeledShape` can freely call `Shape` methods. Callers only need one bound. The relationship is encoded in the type system, not scattered across function signatures.

## The Intuition

A supertrait bound on a trait is like an inheritance declaration in OOP — but more explicit and more composable. `trait LabeledShape: Shape + Display` means "you can't implement `LabeledShape` without also implementing `Shape` and `Display`." The compiler enforces this at the impl site.

Think of it as a contract stack: each layer adds requirements and capabilities. `Shape` gives geometry. `LabeledShape: Shape` adds naming while reusing geometry. `ColoredShape: LabeledShape` adds color while reusing everything below.

## How It Works in Rust

```rust
use std::fmt;

// Layer 1: basic geometry
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Layer 2: requires Shape and Display
// Default methods can call self.area() because Shape is a supertrait
trait LabeledShape: Shape + fmt::Display {
    fn label(&self) -> &str;

    fn describe(&self) -> String {
        // self.area() is available because Shape is required
        format!("[{}] area={:.2}, perimeter={:.2}",
            self.label(), self.area(), self.perimeter())
    }
}

// Layer 3: requires LabeledShape (which requires Shape + Display)
trait ColoredShape: LabeledShape {
    fn color(&self) -> &str;

    fn full_description(&self) -> String {
        format!("{} (color: {})", self.describe(), self.color())
    }
}

struct Circle { radius: f64 }

// Must implement ALL supertraits before implementing the child trait
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

impl LabeledShape for Circle {
    fn label(&self) -> &str { "Circle" }
    // describe() comes for free from the default method
}

impl ColoredShape for Circle {
    fn color(&self) -> &str { "red" }
    // full_description() comes for free
}

// Callers only need one bound — supertraits are implied
fn print_shape(s: &dyn LabeledShape) {
    println!("{}", s.describe()); // can call area() via Shape supertrait
}
```

## What This Unlocks

- **Clean capability hierarchies** — `trait Serializable: Clone + Debug` ensures you always have debug printing and cloning when serializing, no extra bounds needed.
- **Rich default methods** — default implementations can use all supertrait methods, letting you provide more behavior "for free" to implementors.
- **Single-bound APIs** — `fn render<T: Widget>(w: &T)` can call all `Drawable + Layoutable + Focusable` methods if `Widget` declares them all as supertraits.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Trait/signature inclusion | `module type LabeledShape = sig include Shape; ... end` | `trait LabeledShape: Shape + Display` — syntax is more concise |
| Enforcement | Compiler checks module includes the full signature | Compiler requires all supertrait impls before child trait |
| Default methods using super | Must re-expose super functions | Default methods call supertrait methods directly on `self` |
| Dynamic dispatch | `(module S : LabeledShape)` first-class modules | `&dyn LabeledShape` fat pointer with vtable |
