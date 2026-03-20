[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 079 — Associated Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Define traits with associated types (`type Item`) to express relationships between a type and its element type without adding an extra type parameter. Implement a `Container` trait with `IntStack` and `StringQueue`, a generic `drain_all` utility, and a custom `RangeIter` — comparing with OCaml's module abstract types.

## Learning Outcomes

- Declare `type Item` inside a trait and use `Self::Item` in method signatures
- Understand why associated types often replace generic type parameters in traits
- Implement the standard `Iterator` trait with `type Item = i32`
- Write generic functions that refer to a container's item type as `C::Item`
- Map Rust associated types to OCaml `with type item = t` module refinements
- Recognise when to use associated types versus generic parameters

## Rust Application

The `Container` trait declares `type Item` as an associated type, so each implementor fixes what it holds. `IntStack` sets `type Item = i32`; `StringQueue` sets `type Item = String`. The generic function `drain_all<C: Container>` collects items as `Vec<C::Item>` — no extra type variable needed. `RangeIter` implements `Iterator` (from `std`) with `type Item = i32`, enabling the full iterator adapter chain (`map`, `filter`, `collect`) without extra parameters. Using associated types rather than `Container<T>` prevents callers from instantiating conflicting `T` values.

## OCaml Approach

OCaml module types achieve the same abstraction: `module type Container = sig type t; type item; val empty : t; val push : item -> t -> t; … end`. The `with type item = int` refinement pins the abstract type concretely. Functors parameterised over a `Container` module use `C.item` just as Rust uses `C::Item`. The OCaml approach is more explicit — module types and their refinements are named and composed; Rust traits hide the plumbing inside `impl` blocks.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Syntax | `type Item;` inside `trait` | `type item` inside `module type` |
| Pinning | `type Item = i32` in `impl` | `with type item = int` constraint |
| Access | `C::Item` | `C.item` |
| Standard iterator | `Iterator::Item` | Custom `Iterator` module type |
| Multiple impls | One `Item` per impl | One `item` per module |
| Generic alternative | `trait Container<T>` | `module type Container(T)` (functor) |

Associated types enforce a one-to-one relationship: a given type can only implement `Container` once, with one fixed `Item`. Generic parameters (`trait Container<T>`) allow multiple implementations. OCaml's module system makes this distinction through opaque vs refined types; Rust encodes it structurally through trait design.

## Exercises

1. Add a `peek` method to `Container` that returns `Option<&Self::Item>` without removing the item. Implement it for `IntStack`.
2. Create a `Transformer` trait with `type Input` and `type Output`, and implement it for a struct that doubles integers.
3. Write a generic `map_container<C, D>` function that drains `C` and pushes transformed items into `D`, where `D::Item = String` and `C::Item: Display`.
4. Implement a Fibonacci iterator with `type Item = u64` using the standard `Iterator` trait.
5. In OCaml, write a functor `Mapped(C : Container)(F : sig val f : C.item -> C.item end)` that applies `f` to every item during push. Compare the constraint surface with Rust's equivalent generic trait bound.
