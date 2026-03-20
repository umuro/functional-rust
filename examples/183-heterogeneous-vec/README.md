📖 **[View on hightechmind.io →](https://hightechmind.io/rust/183-heterogeneous-vec)**

---

# Heterogeneous Vector with Safe Downcast
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Sometimes you need a single collection to hold values of different types — a property bag, an event queue with mixed event types, a dynamic configuration store. `Box<dyn Any>` erases type information completely, but `downcast_ref::<T>()` recovers it safely: the downcast checks the stored `TypeId` against the requested type and returns `Option<&T>`. This is the runtime equivalent of type-safe storage — safer than `transmute` but with runtime dispatch.

## Learning Outcomes

- Use `Box<dyn Any>` for storing heterogeneous values with type erasure
- Learn `downcast_ref::<T>()` for safe type recovery from `Any`
- Implement a `HeteroVec` with typed push and typed get operations
- Understand the `'static` bound on `Any` and why non-`'static` types cannot be stored

## Rust Application

`HeteroVec` stores `Vec<Box<dyn Any>>`. `push<T: 'static>(&mut self, val: T)` boxes the value as `Box<dyn Any>`. `get<T: 'static>(&self, index: usize) -> Option<&T>` calls `self.items[index].downcast_ref::<T>()`. The `'static` bound is required because `Any` cannot hold references with shorter lifetimes — the stored value must be self-contained. Incorrect type in `get` returns `None` rather than panicking.

## OCaml Approach

OCaml's `Obj.magic` is the unsafe equivalent — it casts any value to any type without checking. Safe alternatives use GADTs:
```ocaml
type any = Any : 'a * ('a -> string) -> any
```
Or use polymorphic functions at the call site. OCaml's GC-managed values are all pointer-sized, making type erasure and "untyped" storage easy but unsafe without GADT wrappers.

## Key Differences

1. **Safety**: Rust's `downcast_ref` checks `TypeId` at runtime — safe by construction; OCaml's `Obj.magic` has no check — inherently unsafe.
2. **`'static` bound**: Rust's `Any` requires `'static` to ensure stored values outlive any reference they might contain; OCaml has no lifetime concept.
3. **Typed maps**: Rust's `TypeMap` pattern uses `TypeId` as keys for typed storage of one value per type; OCaml uses module types or phantom-typed keys.
4. **Performance**: `downcast_ref` is O(1) — one `TypeId` comparison; no allocation, no iteration.

## Exercises

1. Build a `TypeMap` that stores at most one value of each type: `insert<T: Any>(val: T)` and `get<T: Any>() -> Option<&T>`.
2. Implement a safe downcasting iterator: given `Vec<Box<dyn Any>>`, collect all values of a specific type `T`.
3. Write a property bag for UI components: `PropertyBag::set("color", "#ff0000")` and `PropertyBag::get::<String>("color")`.
