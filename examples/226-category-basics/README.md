📖 **[View on hightechmind.io →](https://hightechmind.io/rust/226-category-basics)**

---

# Category Basics
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Category theory provides the mathematical foundation for functional programming abstractions — functors, monads, and natural transformations are all categorical concepts. A category consists of objects (types), morphisms (functions), composition (`.`), and identity (`id`). The category of types and functions (called **Hask** in Haskell, **Typ** in type theory) is the mathematical model that explains why `Option`, `Result`, and `Vec` all have `map`, and why `and_then` (bind) has its signature.

## Learning Outcomes

- Understand the three components of a category: objects, morphisms, and composition
- Learn the identity and associativity laws for function composition
- See Rust's type system as the objects and functions as morphisms in a category
- Connect categorical composition to Rust's function composition combinator

## Rust Application

`identity<A>(a: A) -> A` is the identity morphism. `compose<A, B, C>(f: B -> C, g: A -> B) -> A -> C` is morphism composition. The category laws: identity (`compose(f, identity) == f`) and associativity (`compose(h, compose(g, f)) == compose(compose(h, g), f)`). In Rust, functions are values (`fn(A) -> B` or `impl Fn(A) -> B`), and composition is a higher-order function — Rust's type system is a category.

## OCaml Approach

OCaml's standard library provides `Fun.compose` (since OCaml 4.08):
```ocaml
let (>>) f g x = g (f x)
let id x = x
```
OCaml's pipeline operator `|>` is related: `x |> f` = `f x`. Function composition is idiomatic OCaml; category theory is explicitly taught in the OCaml community through libraries like `Base.Fn.compose`.

## Key Differences

1. **Infix composition**: Haskell uses `.` for `compose`; OCaml uses `>>` or `|>`; Rust uses `compose(f, g)` — named function, no infix operator.
2. **Category laws**: Both languages' `compose` and `identity` satisfy the laws by construction — they are mathematical truths, not runtime assertions.
3. **Type system as category**: The connection between the Rust type system and category theory enables reasoning about abstraction correctness.
4. **Kleisli category**: Functions `A -> Option<B>` form a different category (Kleisli category for `Option`) where composition is `and_then` — this is the mathematical model for monadic composition.

## Exercises

1. Verify the identity law: write a test that `compose(f, identity) == f` for a specific function `f`.
2. Verify associativity: `compose(h, compose(g, f)) == compose(compose(h, g), f)` for concrete `f`, `g`, `h`.
3. Implement Kleisli composition `kleisli_compose<A, B, C>(f: impl Fn(A) -> Option<B>, g: impl Fn(B) -> Option<C>) -> impl Fn(A) -> Option<C>` and verify its laws.
