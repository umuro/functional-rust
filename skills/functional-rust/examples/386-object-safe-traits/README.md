# 386: Object-Safe Traits

**Difficulty:** 3  **Level:** Advanced

Only object-safe traits can be used as `dyn Trait` — the rules determine which ones qualify.

## The Problem This Solves

Sometimes you don't know the concrete type at compile time. You want a `Vec<Box<dyn Shape>>` that holds circles, rectangles, and triangles together — heterogeneous collections with runtime dispatch. For this, Rust needs to build a **vtable**: a table of function pointers, one per trait method. The vtable is attached to a fat pointer (`dyn Trait = data ptr + vtable ptr`).

Not every trait can have a vtable. If a method returns `Self` (the concrete type), the vtable can't know what size to allocate — different types have different sizes. If a method is generic over `T`, there's no single function pointer to store; every `T` produces a different monomorphized version. These traits are **not object-safe** and cannot be used as `dyn Trait`.

Understanding the rules lets you design traits that are usable both statically (generics) and dynamically (trait objects), choosing the right tool for plugin systems, callbacks, and heterogeneous collections.

## The Intuition

Object safety is about whether the compiler can build a vtable for your trait. A vtable is a fixed-size array of function pointers. That means each method must have exactly one version and must work through a pointer without knowing the concrete type.

The two main violations: `-> Self` (can't know the return type size) and `fn method<T>` (would need infinite vtable entries, one per `T`). The workaround for `-> Self` is to return a `Box<dyn Trait>` instead, or gate the method with `where Self: Sized` to exclude it from the vtable.

## How It Works in Rust

```rust
// Object-SAFE: no Self returns, no generic methods
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;

    // This method is excluded from the vtable via where Self: Sized
    // It's available on concrete types but not through dyn Drawable
    fn clone_box(&self) -> Box<dyn Drawable> where Self: Sized + Clone {
        Box::new(self.clone())
    }
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("Circle(r={})", self.radius); }
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

impl Drawable for Rectangle {
    fn draw(&self) { println!("Rectangle({}x{})", self.width, self.height); }
    fn area(&self) -> f64 { self.width * self.height }
}

// Heterogeneous collection — works because Drawable is object-safe
fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
    ];
    println!("Total area: {:.2}", total_area(&shapes));
}
```

**What makes a trait NOT object-safe:**
```rust
trait NotObjectSafe {
    fn clone_self(&self) -> Self;       // returns Self — size unknown
    fn map<T>(&self, f: fn(f64) -> T); // generic method — infinite vtable entries
}
// let _: &dyn NotObjectSafe; // COMPILE ERROR
```

## What This Unlocks

- **Plugin systems** — define a `dyn Plugin` interface; load and call plugins without knowing their concrete types.
- **Heterogeneous collections** — `Vec<Box<dyn Drawable>>`, `Vec<Box<dyn Handler>>` — mix types that share a behavior.
- **Callback registries** — store `Vec<Box<dyn Fn(Event)>>` where callers register different closures.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Heterogeneous dispatch | Class types (`drawable list`) — runtime via object method table | `Vec<Box<dyn Drawable>>` — fat pointer with vtable |
| Object safety rules | Implicit — OCaml OO always dispatches dynamically | Explicit — trait must satisfy rules; compiler rejects violations |
| Opt-out for unsafe methods | No direct mechanism | `where Self: Sized` excludes a method from the vtable |
| Cost | Heap allocation + vtable (same) | Heap allocation + vtable (explicit via `Box`) |
