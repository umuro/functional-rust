📖 **[View on hightechmind.io →](https://hightechmind.io/rust/076-trait-objects)**

---

# 076 — Trait Objects (Dynamic Dispatch)

## Problem Statement

Trait objects (`dyn Trait`) enable runtime polymorphism in Rust — the ability to work with different types through a common interface without knowing the concrete type at compile time. They are Rust's answer to OOP inheritance and interface polymorphism: `Vec<Box<dyn Shape>>` can hold circles, rectangles, and triangles in one collection.

Dynamic dispatch via `dyn Trait` is used in plugin systems, event handlers, GUI widget trees, game entity systems, and any architecture requiring heterogeneous collections. The trade-off: dynamic dispatch has a small vtable lookup overhead but enables flexibility that static generics cannot provide.

## Learning Outcomes

- Define traits with methods and implement them for multiple types
- Use `&dyn Trait` and `Box<dyn Trait>` for dynamic dispatch
- Understand the vtable: a pointer to the trait implementation for the concrete type
- Compare `dyn Trait` (runtime polymorphism) vs generics `<T: Trait>` (compile-time monomorphization)
- Recognize that `dyn Trait` cannot be used with non-object-safe traits

## Rust Application

`trait Shape` with `area(&self) -> f64` and `name(&self) -> &str`. `Circle` and `Rectangle` implement `Shape`. `describe(shape: &dyn Shape)` accepts any shape through a trait object. `Vec<Box<dyn Shape>>` stores heterogeneous shapes. The vtable overhead is one pointer dereference per method call — negligible for most workloads.

## OCaml Approach

OCaml's equivalent is the module system or object system. Module-based: `type shape = { area: unit -> float; name: unit -> string }`. `let circle r = { area = (fun () -> Float.pi *. r *. r); name = (fun () -> "circle") }`. Record-of-functions is OCaml's idiomatic "dynamic dispatch" — a manually built vtable.

## Key Differences

1. **`dyn Trait` vs records**: Rust's vtable is automatic — define the trait, implement it, use `dyn Trait`. OCaml requires manually building record-of-functions vtables, or using the OO subset (`#name`).
2. **Object safety**: Rust's `dyn Trait` requires "object safety": no methods with `Self` return type, no generic methods. OCaml's record-of-functions approach has no such restriction.
3. **`Box` for ownership**: `Box<dyn Trait>` owns the object. `&dyn Trait` borrows it. OCaml's record-of-functions is always heap-allocated (via GC) — no explicit boxing.
4. **Monomorphization vs vtable**: `fn area<T: Shape>(s: &T)` monomorphizes (separate code per type, fast). `fn area(s: &dyn Shape)` uses vtable (one code path, flexible). OCaml's `records` are always vtable-style.

## Exercises

1. **Plugin system**: Define a `Plugin` trait with `name(&self) -> &str` and `execute(&self, input: &str) -> String`. Build a `PluginRegistry` that stores `Vec<Box<dyn Plugin>>` and dispatches by name.
2. **Object safety**: Attempt to use `Clone` as a trait object: `&dyn Clone`. Observe the compiler error. Explain why `Clone` is not object-safe and how to work around it with a `CloneBoxed` trait.
3. **Benchmark**: Measure the performance difference between calling `area` via `&dyn Shape` (dynamic dispatch) vs `fn area<T: Shape>(s: &T)` (static dispatch) on 10M calls. Quantify the vtable overhead.
