📖 **[View on hightechmind.io →](https://hightechmind.io/rust/384-trait-objects-dyn)**

---

# 384: Trait Objects and `dyn Trait`
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Static dispatch (generics with monomorphization) produces the fastest code but requires knowing all concrete types at compile time. Sometimes the set of types is open and determined at runtime — a plugin system, a heterogeneous collection, or a callback registered by user code. Dynamic dispatch via `dyn Trait` solves this: values are stored as fat pointers (data pointer + vtable pointer), enabling runtime polymorphism at the cost of an indirect function call per virtual method.

`dyn Trait` is used in GUI frameworks (event handlers), plugin architectures, `Box<dyn Error>` in error handling, the `std::io::Read`/`Write` traits, and anywhere a collection needs to hold mixed types.

## Learning Outcomes

- Understand the difference between static dispatch (`impl Trait` / generics) and dynamic dispatch (`dyn Trait`)
- Learn what a vtable is and how fat pointers enable runtime polymorphism
- See how `Box<dyn Animal>` enables heterogeneous collections in Rust
- Understand the object safety rules that restrict which traits can be used with `dyn`
- Learn the performance trade-off: monomorphization vs. vtable indirection

## Rust Application

In `src/lib.rs`, `Dog` and `Cat` both implement the `Animal` trait. The `make_noise` function takes `&[Box<dyn Animal>]` — a slice of heap-allocated trait objects. Each `Box<dyn Animal>` is a fat pointer: 16 bytes (data pointer + vtable pointer). When `a.speak()` is called, the vtable pointer dispatches to the correct implementation at runtime. This allows a single `Vec<Box<dyn Animal>>` to hold mixed types without generics.

## OCaml Approach

OCaml achieves the same effect through its object system. Classes define virtual methods, and any `Animal` object holds a method table pointer. Alternatively, OCaml uses first-class modules: `(module Animal : ANIMAL)`. The most idiomatic approach uses algebraic types with pattern matching, avoiding dynamic dispatch entirely when the type set is closed. For open type sets, OCaml's extensible variant types or object methods provide runtime dispatch.

## Key Differences

1. **Fat pointer size**: Rust's `Box<dyn Trait>` is 16 bytes (two pointers); OCaml objects carry a tag word and method table pointer — similar overhead.
2. **Null safety**: Rust's `Box<dyn Animal>` is never null; OCaml objects can be the `Obj.magic null` value in unsafe code.
3. **Object safety**: Rust requires traits used with `dyn` to be object-safe (no generic methods, no `Self` in return positions); OCaml's object methods have no equivalent restriction.
4. **Alternatives**: Rust offers enum dispatch as a closed-set alternative to `dyn`; OCaml uses algebraic types for the same purpose with exhaustiveness checking.

## Exercises

1. **Plugin registry**: Build a `PluginRegistry` that stores `Box<dyn Plugin>` values keyed by name string. Implement `register`, `run_all`, and `run_by_name` methods. Show how new plugins can be added at runtime without changing the registry code.
2. **Type erasure benchmark**: Write a benchmark comparing `Vec<Box<dyn Trait>>` with a generic function over a concrete type, measuring the overhead of vtable dispatch for a tight loop of 1 million calls.
3. **dyn Trait with state**: Implement a `Box<dyn Iterator<Item = i32>>` that wraps different iterator types, showing how `dyn` enables storing iterators of different concrete types in the same variable.
