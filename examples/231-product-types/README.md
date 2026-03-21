📖 **[View on hightechmind.io →](https://hightechmind.io/rust/231-product-types)**

---

# Example 231: Product Types
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Demonstrate product types (the categorical product): structs/records that bundle multiple fields, tuples as anonymous products, and the `curry`/`uncurry` isomorphism that shows tupled and curried functions are equivalent.

## Learning Outcomes

- How OCaml records map directly to Rust `struct` types with named fields
- Why Rust tuples are value types consumed on move, unlike OCaml's persistent pairs
- How `uncurry` and `curry` encode the categorical isomorphism `(A × B → C) ≅ (A → B → C)`
- Why Rust requires `Rc` for shared ownership in closures that return other closures

## OCaml Approach

OCaml records (`{ x: float; y: float }`) are immutable by default and structurally typed within a module scope. Tuples are first-class values, pattern-matched directly. `curry`/`uncurry` are straightforward because OCaml functions are automatically curried — applying `f a b` is identical to `f (a, b)` after `uncurry`.

## Rust Approach

Rust structs are nominal (not structural) types. Tuples are moved on access, so `fst` and `snd` consume their argument. Implementing `curry` requires `Rc` to share the inner function across multiple calls, since Rust closures take ownership. The method syntax (`impl Point2d`) lets behaviour live alongside data, which is more idiomatic than free functions for domain types.

## Key Differences

1. **Mutation default:** OCaml records are immutable by default; Rust structs require `mut` binding to mutate fields.
2. **Tuple consumption:** OCaml pairs can be projected freely; Rust tuple fields are moved on `fst`/`snd` unless the type is `Copy`.
3. **Currying:** OCaml functions are automatically curried; Rust requires explicit closure wrapping and `Rc` for shared state.
4. **Method vs free fn:** Rust encourages `impl Type { fn method(&self) }` for type-associated behaviour; OCaml uses modules.

## Exercises

1. Implement `bimap` for a product type `Pair<A, B>` that applies one function to the first component and another to the second.
2. Define a generic `swap` function for product types and implement `curry` and `uncurry` as morphisms in the product category.
3. Implement a heterogeneous record type using Rust tuples as a product type and write a lens for each field that allows reading and updating individual components.
