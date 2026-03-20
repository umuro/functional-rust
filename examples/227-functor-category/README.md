📖 **[View on hightechmind.io →](https://hightechmind.io/rust/227-functor-category)**

---

# Example 227: Functor Category — Functors as Objects, Natural Transformations as Morphisms

**Difficulty:** ⭐⭐⭐
**Category:** Category Theory
**OCaml Source:** Real World OCaml / Category Theory for Programmers

## Problem Statement

In a functor category, objects are functors (type constructors with `map`) and morphisms are natural transformations (polymorphic functions between two functors that commute with `map`). This example models two natural transformations — `Vec → Option` and `Option → Vec` — and verifies the naturality condition.

## Learning Outcomes

- How Rust models functors through generic types (`Vec<T>`, `Option<T>`) without higher-kinded types
- How natural transformations become polymorphic generic functions in Rust
- The naturality condition expressed as a predicate over Rust functions
- Why slice borrows (`&[T]`) are idiomatic — no allocation, works on any contiguous sequence

## OCaml Approach

OCaml uses module signatures (`FUNCTOR`) to formally encode the functor interface, and polymorphic functions like `'a list -> 'a option` naturally express natural transformations. The naturality condition is verified at runtime with `assert`. OCaml's first-class module system makes functor categories almost directly expressible.

## Rust Approach

Rust lacks higher-kinded types, so `Functor` cannot be expressed as a trait parametrized over a type constructor. Instead, we work directly with concrete types (`&[T]`, `Option<T>`, `Vec<T>`) and write generic functions that serve as natural transformations. The naturality condition becomes a generic predicate function that takes a list and a function and verifies commutativity.

## Key Differences

1. **Higher-kinded types:** OCaml can define `module type FUNCTOR` with `type 'a t` (an HKT); Rust cannot — there is no equivalent of `F<T>` as a type-level variable in stable Rust.
2. **Natural transformations:** OCaml writes `'a list -> 'a option` and the polymorphism is implicit; Rust writes `fn<T>(list: &[T]) -> Option<&T>` with explicit generic parameters.
3. **Ownership:** `list_to_option` returns `Option<&T>` — a borrow into the slice — rather than cloning the first element, reflecting Rust's zero-copy preference.
4. **Naturality verification:** OCaml uses `assert` inline; Rust encodes it as a typed generic function `naturality_holds<T, U, F>` with `T: Clone, U: PartialEq` bounds, making the contract explicit.

## Exercises

1. Implement a `Compose<F, G>` type that applies functor `G` inside functor `F` and implement `fmap` for it, demonstrating that the composition of two functors is a functor.
2. Write a natural transformation from `Option<T>` to `Vec<T>` (empty vec for `None`, singleton vec for `Some`) and verify it satisfies the naturality condition: `fmap(f) . nat_transform == nat_transform . fmap(f)`.
3. Implement `const_functor` — a functor that ignores the `fmap` function and always returns the same wrapped value — and use it to count the number of elements in any functor context.
