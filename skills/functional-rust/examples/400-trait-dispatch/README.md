# 400: Static vs Dynamic Dispatch

**Difficulty:** 3  **Level:** Advanced

Two ways to call trait methods — one resolved at compile time, one at runtime — with different performance and flexibility trade-offs.

## The Problem This Solves

When you call a method through a trait, the compiler needs to know *which* implementation to call. With static dispatch, it figures this out at compile time by generating a separate copy of the function for each concrete type (monomorphization). With dynamic dispatch, it defers the decision to runtime via a vtable — a table of function pointers attached to the object.

Static dispatch is faster: the compiler can inline calls, there's no pointer indirection, and the optimizer sees the full picture. Dynamic dispatch is more flexible: you can store different types in the same collection, return trait objects from functions without knowing the concrete type, and load plugins at runtime.

The choice matters when designing APIs. A function that takes `impl Processor` gets monomorphized — zero overhead, but you can't mix types in a `Vec<impl Processor>`. A function that takes `&dyn Processor` has a vtable lookup on every call, but a `Vec<Box<dyn Processor>>` holds any mix of types.

## The Intuition

Static dispatch = the compiler stamps out one function per concrete type. Dynamic dispatch = the runtime looks up the right function in a table. Same trait, different machinery.

## How It Works in Rust

```rust
trait Processor {
    fn process(&self, x: i64) -> i64;
}

// STATIC dispatch — monomorphized, inlinable
fn apply_static<P: Processor>(p: &P, x: i64) -> i64 {
    p.process(x)  // compiler knows the exact impl at compile time
}

// DYNAMIC dispatch — vtable lookup at runtime
fn apply_dynamic(p: &dyn Processor, x: i64) -> i64 {
    p.process(x)  // fat pointer: (data ptr, vtable ptr)
}

// Static: homogeneous — can't mix Double and Square in one Vec
let d = Double;
apply_static(&d, 5);  // compiler generates apply_static::<Double>

// Dynamic: heterogeneous — mix any Processor implementations
let processors: Vec<Box<dyn Processor>> = vec![
    Box::new(Double),
    Box::new(Square),
    Box::new(AddN { n: 10 }),
];
for p in &processors { p.process(5); }  // vtable dispatch each time
```

1. `impl Trait` / `<T: Trait>` → static. Compiler generates code per type.
2. `dyn Trait` → dynamic. Fat pointer carries data + vtable. Each call = one indirection.
3. `dyn Trait` requires the trait to be *object-safe* (no `Self` returns, no generics on methods).

## What This Unlocks

- **Static dispatch**: Maximum performance, inlining, dead-code elimination — default choice when types are known.
- **Dynamic dispatch**: Plugin architectures, heterogeneous collections, erasing types across API boundaries.
- **Hybrid**: Generic functions with `impl Trait` internally can accept `Box<dyn Trait>` externally — combine both.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Polymorphism default | Dynamic (first-class modules, objects) | Static (monomorphization) |
| Runtime dispatch | Object types, first-class modules | `dyn Trait` with vtable |
| Compile-time dispatch | Functors, inline modules | `impl Trait`, `<T: Trait>` |
| Heterogeneous collections | `'a list` (uniform type) or GADT | `Vec<Box<dyn Trait>>` |
| Object safety | N/A | Required for `dyn Trait` (no generic methods) |
