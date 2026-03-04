# 076: Trait Objects

**Difficulty:** 2  **Level:** Intermediate

Store different types in the same collection by dispatching through a shared trait at runtime.

## The Problem This Solves

You have circles, rectangles, and triangles. You want to put them all in a `Vec` and call `.area()` on each. The problem: they're different types. In Java you'd use an interface. In Python, duck typing just works. In Rust you need to be explicit about how polymorphism happens.

Without trait objects, you'd need a separate `Vec` for each shape type, or an enum that lists every possible variant up front. The enum approach is fine when the set of types is closed and you own them all — but it breaks down when you want to add new shapes from outside your crate.

`Box<dyn Shape>` gives you Java-style interface polymorphism: a heap-allocated pointer to any type that implements `Shape`, with method dispatch happening via a vtable at runtime. You give up a small amount of performance (one extra pointer indirection) and gain full open extensibility.

## The Intuition

Think of `dyn Shape` as a fat pointer: it carries both a pointer to the data *and* a pointer to a vtable (a table of function pointers for each trait method). When you call `.area()`, Rust looks up the right function in the vtable.

Compare:
- **Python**: `shape.area()` — always dynamic, no declaration needed
- **Java**: `Shape shape = new Circle(...)` — interface dispatch via vtable
- **Rust `dyn Trait`**: `Box<dyn Shape>` — explicit vtable dispatch, heap allocated
- **Rust `impl Trait` / generics**: monomorphized at compile time, no vtable, faster but can't mix types in a `Vec`

The key question: do you need to store *different* concrete types together? If yes → `dyn Trait`. If no → generics.

## How It Works in Rust

```rust
// Define the shared interface
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

// Each type implements it independently
struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

// Box<dyn Shape> lets you mix types in one Vec
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 3.0, height: 4.0 }),
];

// Runtime dispatch — the right `.area()` is called for each type
let total: f64 = shapes.iter().map(|s| s.area()).sum();
```

```rust
// When types are known at the call site, use generics (no vtable, zero cost)
fn describe<S: Shape>(shape: &S) -> String {
    format!("{}: {:.2}", shape.name(), shape.area())
}
```

The `&dyn Shape` (borrowed) vs `Box<dyn Shape>` (owned) distinction: use `&dyn` when you just need to call methods temporarily; use `Box<dyn>` when you need to store or own the value.

## What This Unlocks

- **Plugin systems**: accept `Box<dyn Renderer>` or `Box<dyn Logger>` from user code without knowing the concrete type.
- **Heterogeneous collections**: `Vec<Box<dyn Widget>>` for a GUI toolkit, `Vec<Box<dyn Middleware>>` for a web framework.
- **Test doubles**: swap real implementations with mocks by passing `Box<dyn Database>` instead of a concrete struct.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Polymorphism | Structural (modules/objects implicitly match) | Explicit trait `impl` required |
| Dynamic dispatch | Object types, first-class modules | `dyn Trait` (fat pointer + vtable) |
| Static dispatch | Polymorphic functions via type inference | Generics `<T: Trait>` (monomorphized) |
| Mixed-type collection | `Shape list` with object type | `Vec<Box<dyn Shape>>` |
| Closed set of types | Algebraic types + pattern match | `enum` + `match` (preferred when set is known) |
