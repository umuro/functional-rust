# 147: Type Erasure

**Difficulty:** 4  **Level:** Advanced

Hide a concrete type behind a trait object — keep the behavior, forget the type.

## The Problem This Solves

You want to store a `Circle`, a `Rectangle`, and a `Triangle` in the same `Vec`. But they're different types — Rust doesn't allow that. You need a way to say "I only care that each thing can compute its area" and forget what it actually is.

Type erasure does exactly that: you erase the concrete type and retain only a trait interface. A `Box<dyn Drawable>` can hold any type that implements `Drawable`. You can call `.area()` on it. You can't call `Circle`-specific methods — those are gone. The trade is flexibility for specificity.

The alternative — a `Vec<ShapeEnum>` — is a closed set. Type erasure is open: any type that implements the trait works, even ones defined after the fact.

## The Intuition

`Box<dyn Trait>` is a fat pointer: two words of memory. One word points to the data on the heap. The other points to a *vtable* — a table of function pointers for the trait's methods. When you call `.area()`, Rust looks up the right function in the vtable at runtime.

This is dynamic dispatch: the exact function called is determined at runtime, not compile time. You trade monomorphization (zero-cost, compile-time) for flexibility (runtime lookup, one allocation per value).

When to use: heterogeneous collections, plugin systems, callbacks, anything where the concrete type is not known until runtime.

## How It Works in Rust

```rust
pub trait Drawable: std::fmt::Debug {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
}

// Different concrete types — different sizes, different implementations
pub struct Circle { pub radius: f64 }
pub struct Rect { pub w: f64, pub h: f64 }

impl Drawable for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    fn name(&self) -> &'static str { "circle" }
}

// Heterogeneous collection — concrete types are erased to Box<dyn Drawable>
pub struct Scene {
    shapes: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub fn add(mut self, s: impl Drawable + 'static) -> Self {
        self.shapes.push(Box::new(s));  // erases the type here
        self
    }

    pub fn total_area(&self) -> f64 {
        self.shapes.iter().map(|s| s.area()).sum()  // dispatch via vtable
    }
}
```

Erasing a value with a closure (mirrors OCaml GADT packing):

```rust
pub struct AnyShow(Box<dyn Showable>);

impl AnyShow {
    pub fn new<T: Showable + 'static>(v: T) -> Self {
        AnyShow(Box::new(v))  // T is erased — only Showable remains
    }
}

// After construction, you can't recover T — only call .show()
let items: Vec<AnyShow> = vec![
    AnyShow::new(42_i32),
    AnyShow::new("hello".to_string()),
    AnyShow::new(3.14_f64),
];
```

Erased callbacks (function type erasure):

```rust
pub struct Handler {
    callback: Box<dyn Fn(&str) -> String>,  // the function type is erased
}

// Any closure or function with the right signature works
let handlers = vec![
    Handler::new(|s| s.to_uppercase()),
    Handler::new(|s| s.chars().rev().collect()),
];
```

## What This Unlocks

- **Plugin systems** — load plugins at runtime; each implements a known trait
- **Event handlers / callbacks** — store heterogeneous closures in a single queue
- **UI widget trees** — `Vec<Box<dyn Widget>>` where each widget type is different

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type erasure mechanism | First-class modules (existential): `(module S : SHOW)` | `Box<dyn Trait>` — trait object |
| Dispatch | Function pointer through module record | vtable lookup — fat pointer |
| Allocation | Module packs value on heap | `Box` heap-allocates the concrete value |
| Multi-capability | Module with multiple values | Super-trait or combined trait bound |
| Recover concrete type | Pattern match GADT constructor | Not directly — use `Any::downcast_ref` |
| Zero-cost alternative | N/A | Generics with `impl Trait` (monomorphized) |
