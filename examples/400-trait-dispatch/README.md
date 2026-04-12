📖 **[View on hightechmind.io →](https://hightechmind.io/rust/400-trait-dispatch)**

---

# 400: Static vs. Dynamic Trait Dispatch
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Polymorphism has two implementation strategies with different performance characteristics. Static dispatch (monomorphization) generates a separate copy of the function for each concrete type at compile time — maximum performance, larger binary. Dynamic dispatch (vtable) uses a single function that calls through a pointer table at runtime — smaller binary, supports heterogeneous collections, with the cost of one pointer indirection per virtual call. Rust gives you explicit control over this choice: `impl Trait` / generics for static, `dyn Trait` for dynamic.

Understanding this trade-off is essential for writing correct and performant Rust — it comes up in every API design decision involving traits, closures, and async code.

## Learning Outcomes

- Understand monomorphization: the compiler generates specialized code per type
- Learn how vtables enable runtime polymorphism via fat pointers
- See the concrete performance difference: vtable call = pointer dereference + indirect jump
- Understand when to prefer static dispatch (tight loops, known types) vs. dynamic (plugin systems, heterogeneous collections)
- Learn how `total_area_static` and `total_area_dynamic` express the same logic with different dispatch

## Rust Application

In `src/lib.rs`, `total_area_static<T: Shape>` is monomorphized: calling it with `[Circle]` and `[Square]` produces two separate compiled functions. `total_area_dynamic(&[Box<dyn Shape>])` uses one compiled function that dispatches through vtables. The dynamic version accepts mixed `[Circle, Square]` slices — impossible with the static version without an enum. The comment in the source captures the trade-off: static = faster + larger binary; dynamic = smaller binary + heterogeneous.

## OCaml Approach

OCaml uses uniform representation for most values and dynamically dispatches object methods always. Native code OCaml performs some inlining and specialization for known types, but the programmer rarely controls the static/dynamic dispatch boundary explicitly. OCaml's first-class modules can achieve static dispatch via functor monomorphization when performance requires it.

## Key Differences

1. **Explicit choice**: Rust makes static/dynamic dispatch an explicit API choice (`T: Shape` vs. `dyn Shape`); OCaml makes this implicit based on whether objects or modules are used.
2. **Binary size**: Rust's monomorphization can significantly inflate binary size for generic code; OCaml's uniform representation keeps binaries smaller.
3. **Heterogeneous collections**: Dynamic dispatch is required for mixed-type slices in Rust; OCaml can mix objects of the same class hierarchy without explicit trait objects.
4. **Inlining**: Static dispatch enables inlining across trait method calls; dynamic dispatch cannot be inlined since the target is unknown at compile time.

## Exercises

1. **Enum dispatch**: Implement a third variant using an enum `Shape { Circle(Circle), Square(Square) }` and `fn total_area_enum(shapes: &[Shape]) -> f64`. Benchmark all three approaches (static, dynamic, enum) for 1 million shapes.
2. **Indirect overhead measurement**: Write a benchmark calling a trivial method (`fn area()` returning a constant) 100 million times via static dispatch vs. vtable dispatch. Measure the overhead of the indirect call.
3. **Plugin system**: Build a shape registry that loads `Box<dyn Shape>` instances by name string. Show why dynamic dispatch is required here: the concrete type is unknown until the string is parsed at runtime.
