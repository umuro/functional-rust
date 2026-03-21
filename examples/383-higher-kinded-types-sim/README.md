📖 **[View on hightechmind.io →](https://hightechmind.io/rust/383-higher-kinded-types-sim)**

---

# 383: Simulating Higher-Kinded Types with GATs
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Higher-Kinded Types (HKTs) let you abstract over type constructors like `Option`, `Vec`, and `Result` — not just concrete types. A `Functor` typeclass in Haskell captures "any container that can be mapped over" without specifying whether it's a list, maybe, or tree. Rust lacks native HKTs, but Generic Associated Types (GATs, stabilized in Rust 1.65) enable simulation: `type Mapped<B>` as an associated type lets a trait express "map this container's inner type from A to B."

This pattern appears in functional library design (the `futures` crate's `Output` associated type, `async-trait` patterns) and is the foundation of monadic abstractions used in parser combinators and effect systems.

## Learning Outcomes

- Understand what higher-kinded types are and why they matter for generic functional programming
- Learn how Rust's Generic Associated Types (GATs) simulate HKT behavior
- See the `Functor` trait implemented for `Option`, `Vec`, and `Result`
- Understand the difference between type parameters (concrete types) and type constructors (types that take types)
- Learn why Rust chose GATs as the approach to HKT simulation

## Rust Application

The `Functor` trait in `src/lib.rs` uses two associated types: `Unwrapped` (the inner type A) and `Mapped<B>` (a GAT — the container with inner type B). The `fmap` method transforms `Self` into `Self::Mapped<B>` by applying `F: Fn(Self::Unwrapped) -> B`. Implementing for `Option<A>` delegates to `Option::map`; for `Vec<A>` it uses `into_iter().map(f).collect()`. The GAT `type Mapped<B>` is the key innovation — it's a type that takes another type as parameter, hence "higher-kinded."

## OCaml Approach

OCaml supports HKTs natively through its module system. A `Functor` module signature contains `type 'a t` and `val fmap : ('a -> 'b) -> 'a t -> 'b t`. Implementing for `Option` is `module OptionFunctor = struct type 'a t = 'a option; let fmap f = Option.map f end`. The type parameter `'a` in `'a t` directly expresses the kind `* -> *` that Rust approximates with GATs. OCaml's approach is simpler and more expressive.

## Key Differences

1. **Native vs. simulated**: OCaml supports `'a t` (type constructor abstraction) natively; Rust requires the GAT workaround with `type Mapped<B>`.
2. **Ergonomics**: OCaml's `fmap f opt` reads naturally; Rust's `opt.fmap(f)` works but GAT bounds can become verbose in generic contexts.
3. **Monad simulation**: OCaml can express `bind` as naturally as `fmap`; Rust's GAT approach struggles with `bind` (flatMap) since `Mapped<Mapped<B>>` creates problematic nesting.
4. **Stability**: Rust's GATs were unstable for years and stabilized in 1.65; OCaml has had native HKTs since its inception.

## Exercises

1. **Applicative functor**: Extend the trait to add `fn ap<B, F: Fn(Self::Unwrapped) -> B>(self, f: Self::Mapped<F>) -> Self::Mapped<B>` and implement it for `Option` (None when either is None).
2. **Custom container**: Define a `Tree<T>` type (leaf or node with two children) and implement `Functor` for it, applying `fmap` recursively to transform all leaf values.
3. **Foldable trait**: Define a `Foldable` trait with `fn fold<B, F: Fn(B, Self::Unwrapped) -> B>(self, init: B, f: F) -> B` and implement it for `Option` and `Vec`.
