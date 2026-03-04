# 124: dyn Trait — Dynamic Dispatch

**Difficulty:** 3  **Level:** Intermediate

Dispatch method calls at runtime via a vtable — the right tool when you need heterogeneous collections or can't know the concrete type at compile time.

## The Problem This Solves

`impl Trait` and generics are great for performance, but they have a fundamental limitation: the compiler must know the concrete type at compile time. You can't put a `Circle`, a `Rectangle`, and a `Triangle` in the same `Vec<impl Shape>` — they're different types, they have different sizes, and `impl Trait` produces a different monomorphized copy of the code for each.

If you need a collection that holds mixed types, or if you're building a plugin system where the types aren't known at compile time, you need runtime dispatch. `dyn Trait` is Rust's answer. A `Box<dyn Shape>` is a fat pointer: one pointer to the data, one pointer to a vtable. The vtable tells the runtime which concrete `area()` method to call. The cost is an extra pointer indirection per call — usually negligible, always explicit.

There's a third option: enum dispatch. If the set of types is closed (you control them all), put them in an enum and `match`. No vtable, no heap allocation, exhaustive matching enforced by the compiler. The shape example in this module shows all three strategies side by side so you can compare the trade-offs directly.

## The Intuition

`dyn Trait` = one function call goes through a vtable at runtime — use it when you need heterogeneous collections or open extensibility, and accept the indirection cost.

## How It Works in Rust

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

// Static dispatch: compiler generates a separate copy for Circle, Rect, etc.
// Fast — direct call, LLVM can inline. But can't mix in one Vec.
fn print_area(s: &impl Shape) {
    println!("{}: {:.2}", s.name(), s.area());
}

// Dynamic dispatch: one function, vtable lookup per call.
// Slightly slower. Can hold Circle, Rect, Triangle in the same Vec.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()  // each .area() goes through vtable
}

let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rect { width: 3.0, height: 4.0 }),
    Box::new(Triangle { base: 6.0, height: 3.0 }),
];
println!("Total: {:.2}", total_area(&shapes));

// Borrowed trait objects — no allocation needed
let c = Circle { radius: 1.0 };
let s: &dyn Shape = &c;   // fat pointer: &data + &vtable
println!("{}", s.name());

// Enum dispatch — closed set, zero overhead, exhaustive matching
enum ShapeEnum { Circle(f64), Rect(f64, f64), Triangle(f64, f64) }
impl ShapeEnum {
    fn area(&self) -> f64 {
        match self {
            ShapeEnum::Circle(r)     => PI * r * r,
            ShapeEnum::Rect(w, h)    => w * h,
            ShapeEnum::Triangle(b,h) => 0.5 * b * h,
        }
    }
}
```

## What This Unlocks

- **Heterogeneous collections** — `Vec<Box<dyn Handler>>`, `Vec<Box<dyn Plugin>>`: hold any type that implements the trait.
- **Dynamic plugin systems** — load and call implementations you don't know at compile time.
- **Enum dispatch** — when the type set is fixed, skip the vtable entirely and match exhaustively.

## Key Differences

| Strategy | Extensible? | Overhead | Heterogeneous? |
|----------|-------------|----------|----------------|
| `impl Trait` (static) | Yes | Zero — inlined | No — one concrete type per call site |
| `dyn Trait` (dynamic) | Yes | vtable indirection | Yes — mixed types in one collection |
| Enum dispatch | No — closed set | Zero — `match` | Yes — all variants in one type |

| Concept | OCaml | Rust |
|---------|-------|------|
| Object polymorphism | Objects (structural) | `dyn Trait` (nominal, vtable) |
| Closed-set polymorphism | Variants / GADTs | Enums + `match` |
| Open polymorphism | First-class modules | `dyn Trait` or generics |
