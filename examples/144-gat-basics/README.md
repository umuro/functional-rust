📖 **[View on hightechmind.io →](https://hightechmind.io/rust/144-gat-basics)**

---

# Generic Associated Types (GAT) Basics
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Before Generic Associated Types (GATs, stable in Rust 1.65), it was impossible to write a trait whose associated types were themselves generic over a lifetime or type parameter. This blocked implementing traits like `LendingIterator` (where each call to `next` borrows from the iterator itself) or a generic `Container` trait with a `map` method that changes the element type. GATs fill this gap, enabling patterns previously impossible in stable Rust.

## Learning Outcomes

- Understand what Generic Associated Types are and the problems they solve
- Learn the GAT syntax: `type Mapped<U>` and `type Iter<'a>` inside trait definitions
- See how GATs enable lifetime-generic associated types (the lending iterator pattern)
- Understand the current limitations and workarounds in stable Rust's GAT support

## Rust Application

The canonical example is `LendingIterator`:
```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```
Here `Item<'a>` is generic over the lifetime `'a` of the borrow — a regular associated type cannot express this. Another use is `Container::Mapped<U>`: a `Vec<T>` mapping to `Vec<U>` requires `type Mapped<U> = Vec<U>`. GATs make higher-kinded simulation (example 134) more ergonomic, as `type Applied<T>` is a GAT.

## OCaml Approach

OCaml's module system naturally handles GAT-like patterns through parameterized module types:
```ocaml
module type CONTAINER = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end
```
The type parameter `'a` on `t` provides what Rust calls a GAT. OCaml functors parameterized by such modules achieve full higher-kinded programming without the complexity of GATs — it is a native, well-established feature.

## Key Differences

1. **Stability**: Rust GATs became stable in 1.65 (2022); OCaml's parameterized module types have been stable since OCaml 1.0.
2. **Lifetime GATs**: Rust GATs can be parameterized by lifetimes (`type Item<'a>`), enabling the lending iterator; OCaml has no lifetime concept.
3. **Complexity**: Rust's GAT support has known limitations around lifetime bounds (`where Self: 'a` is often required); OCaml's parameterized types have no such requirement.
4. **Ergonomics**: OCaml's syntax for parameterized associated types is simpler (`'a t`); Rust requires the full `type Item<'a> where Self: 'a` declaration.

## Exercises

1. Implement `LendingIterator` for a struct `StrChunks<'s>` that borrows from its source string and yields `&'s str` slices.
2. Define a `Mappable` trait with `type Output<U>` and implement it for `Vec<T>`, `Option<T>`, and `Result<T, E>`.
3. Write a GAT-based `Stack` trait with `type Pushed<T>` that returns a new stack type with an element pushed, enabling typestate-like operations.
