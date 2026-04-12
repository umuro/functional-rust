📖 **[View on hightechmind.io →](https://hightechmind.io/rust/136-existential-types)**

---

# Existential Types
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

An existential type says: "there exists some concrete type implementing this interface, but I won't tell you which one." This enables hiding implementation details, building heterogeneous collections (a list of values with different concrete types all sharing a common interface), and returning values whose concrete type is unnameable (like closures). Existential types are the dual of universal (generic) types, and they appear in every major typed language under different names.

## Learning Outcomes

- Understand what an existential type is and how it differs from a generic (universal) type
- Learn Rust's two encodings: `impl Trait` (static, zero-cost) and `Box<dyn Trait>` (dynamic, heap-allocated)
- See when each encoding is appropriate: single opaque return vs. heterogeneous collection
- Connect Rust's existential types to OCaml's first-class modules and GADTs

## Rust Application

`impl Showable` in return position is a static existential — the caller knows "some type implements Showable" but cannot name it. The compiler monomorphizes each call site. `Box<dyn Showable>` is a dynamic existential — it erases the concrete type at runtime via a vtable, enabling `Vec<Box<dyn Showable>>` to hold `Counter`, `Label`, and `i32` values together. The `make_showable_from` function shows the dynamic version: callers pass different types, all boxed behind the trait object.

## OCaml Approach

OCaml encodes existential types via first-class modules:
```ocaml
module type SHOWABLE = sig type t val show : t -> string end
type showable = (module SHOWABLE with type t = _)
(* or via GADTs: *)
type showable = Show : 'a * ('a -> string) -> showable
```
The GADT encoding packs the value and its `show` function together, erasing the concrete type. This is more flexible than Rust's `dyn Trait` because the packed function is not restricted to a fixed vtable layout.

## Key Differences

1. **Two encodings**: Rust provides static existentials (`impl Trait`) and dynamic existentials (`Box<dyn Trait>`); OCaml uses first-class modules and GADTs for both.
2. **Vtable layout**: Rust's `dyn Trait` vtable is fixed at trait definition time; OCaml's GADT-based existentials pack arbitrary functions without a fixed interface.
3. **Collection use**: Only `Box<dyn Trait>` (dynamic) allows heterogeneous collections; OCaml's `showable list` works the same way with GADT encoding.
4. **Dyn compatibility**: Rust imposes dyn-compatibility rules (no generic methods); OCaml's approach has no such restriction.

## Exercises

1. Create a `Vec<Box<dyn Display>>` containing an `i32`, a `f64`, and a `String`, then print each element.
2. Implement `make_counter() -> impl FnMut() -> u32` using a closure as a static existential, hiding the closure type.
3. Write a plugin registry that accepts `Box<dyn Plugin>` values from external code and calls their `run` method polymorphically.
