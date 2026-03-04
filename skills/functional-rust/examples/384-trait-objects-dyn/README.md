# 384: dyn Trait and Fat Pointers

**Difficulty:** 3  **Level:** Advanced

Runtime polymorphism via vtable dispatch — heterogeneous collections and plugin architectures.

## The Problem This Solves

Generics give you zero-cost polymorphism via monomorphization: the compiler generates one copy of the function per concrete type. That's fast, but it means all types must be known at compile time, and you can't mix different types in the same collection.

Sometimes you need runtime polymorphism: a `Vec` of different shapes that all implement `Draw`. A plugin system where plugins are loaded at runtime. A callback system where handlers can be any type implementing a trait. That's what `dyn Trait` is for — it erases the concrete type and dispatches through a vtable, exactly like virtual functions in C++ or interface values in Go.

The cost: one extra pointer indirection per method call (vtable lookup), and the concrete type is lost (no monomorphization). The benefit: heterogeneous collections, object-safe trait dispatch, and no code bloat from monomorphization.

## The Intuition

`Box<dyn Draw>` is a *fat pointer*: two machine words. The first points to the data. The second points to the vtable — a struct of function pointers for every method in the `Draw` trait. Calling `shape.draw()` loads the function pointer from the vtable and calls it. This is exactly how C++ virtual dispatch works.

Not all traits are *object-safe* — `dyn Trait` requires that no method returns `Self`, takes `Self` by value, or has generic type parameters. If a trait isn't object-safe, the compiler tells you. Common fixes: add `where Self: Sized` to non-object-safe methods, or use `Box<dyn Any>` for type-erased values.

## How It Works in Rust

```rust
trait Draw {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

struct Circle { x: f64, y: f64, r: f64 }
struct Rect   { x: f64, y: f64, w: f64, h: f64 }

impl Draw for Circle {
    fn draw(&self) { println!("Circle at ({}, {})", self.x, self.y); }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x - self.r, self.y - self.r, self.r*2.0, self.r*2.0)
    }
}

impl Draw for Rect {
    fn draw(&self) { println!("Rect at ({}, {})", self.x, self.y); }
    fn bounding_box(&self) -> (f64, f64, f64, f64) { (self.x, self.y, self.w, self.h) }
}

// Heterogeneous collection — only possible with dyn
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { x: 0.0, y: 0.0, r: 5.0 }),
    Box::new(Rect   { x: 1.0, y: 1.0, w: 3.0, h: 4.0 }),
];

for shape in &shapes {
    shape.draw(); // vtable dispatch
}
```

## What This Unlocks

- **Heterogeneous collections** — `Vec<Box<dyn Trait>>` holds any type implementing the trait.
- **Plugin systems** — load types at runtime (from config, from dynamic libraries) behind a trait object.
- **Callbacks and handlers** — store closures or structs implementing `Fn` traits in a uniform way.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Runtime polymorphism | Variants + pattern match, or first-class modules | `dyn Trait` with vtable dispatch |
| Object file | Module value (first-class) | `Box<dyn Trait>` fat pointer |
| Virtual dispatch | Automatic via GADT / records of functions | Explicit `dyn` keyword, compiler checks object safety |
| Performance | Record-of-functions has same overhead | One indirection per call via vtable |
