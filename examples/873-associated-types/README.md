📖 **[View on hightechmind.io →](https://hightechmind.io/rust/873-associated-types)**

---

# 873-associated-types — Associated Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When a trait operation involves a type that varies per implementation — like the element type of a container, or the output type of an addition — you need a way to express that relationship without making the trait itself generic. Rust's associated types solve this: a trait `Container` declares `type Item`, and each implementation specifies what `Item` is. This avoids the ambiguity of generic traits (where multiple `Container<i32>` and `Container<String>` implementations could coexist on the same struct) and makes trait bounds more readable. OCaml's module types use the same idea with `type t` and `type item` declarations inside module signatures.

## Learning Outcomes

- Declare and use associated types in Rust traits
- Understand why associated types are preferred over generic trait parameters in many cases
- Implement the same trait for multiple concrete types with different `Item` types
- Compare Rust associated types with OCaml module type `type` declarations
- Recognize the pattern in standard library traits like `Iterator::Item` and `Add::Output`

## Rust Application

The code defines a `Container` trait with `type Item`, implementing it for `Stack<T>` where `type Item = T`. A second trait `Combinable` declares `type Output` (the result type of combining two values), implemented for `Point` where combining returns the distance as `f64`. This mirrors the standard `Add` trait's `type Output`. The trait bounds at call sites become simpler: `impl<C: Container<Item = i32>>` is cleaner than `impl<C: Container<i32>>` for single-implementation cases.

## OCaml Approach

OCaml's `module type Container = sig type t; type item; val push: item -> t -> t; ... end` directly parallels Rust's associated type design. A stack module `Stack: Container with type item = int` fixes the item type at instantiation. The `with type` refinement in OCaml is the equivalent of specifying `type Item = i32` in a Rust `impl`. OCaml module types also support abstract types that become concrete in implementations, just like Rust's associated types.

## Key Differences

1. **Disambiguation**: Rust associated types prevent multiple implementations of the same trait on one type (unlike generic trait parameters); OCaml module types achieve the same through module identity.
2. **Constraint syntax**: Rust uses `where C::Item: Display`; OCaml uses `with type item = ...` or functor application.
3. **Type inference**: Rust infers associated types from the `impl` block; OCaml infers them from module structure.
4. **Standard library integration**: Rust's `Iterator::Item`, `Add::Output`, `Index::Output` all use associated types — this pattern is pervasive; OCaml uses it in `Map.S`, `Set.S`, etc.

## Exercises

1. Define a `Parseable` trait with `type Error` as an associated type, and implement it for `i32` and `f64` with different error types.
2. Add a `map` method to the `Container` trait using an associated type `MappedOutput<U>` (hint: this requires generic associated types in Rust 1.65+).
3. Implement a `Graph` trait with `type Node` and `type Edge` associated types, and a simple adjacency-list graph that satisfies it.
