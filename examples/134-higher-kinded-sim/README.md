📖 **[View on hightechmind.io →](https://hightechmind.io/rust/134-higher-kinded-sim)**

---

# Higher-Kinded Types Simulation
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Higher-kinded types (HKTs) allow abstracting over type constructors like `Option<_>`, `Vec<_>`, or `Result<_, E>` — not just types. This enables writing a single `map` implementation that works over any container without duplicating code. Haskell's `Functor` and `Monad` typeclasses rely on HKTs. Rust lacks native HKTs, but Generic Associated Types (GATs, stable since 1.65) enable a defunctionalization-based simulation that achieves the same abstraction.

## Learning Outcomes

- Understand what higher-kinded types are and why they are useful for generic programming
- Learn the defunctionalization technique: marker types (`OptionHKT`) + GATs (`type Applied<T>`)
- See how `Functor`, `Monad`, and generic `sequence` work over any HKT container
- Understand the limitations compared to native HKTs in Haskell or OCaml

## Rust Application

`trait HKT { type Applied<T>; }` is the GAT that reconstructs concrete types. `OptionHKT::Applied<i32>` resolves to `Option<i32>`. `Functor` provides `fmap` over any `HKT`; `Monad` adds `pure` and `bind`. The generic `sequence` function works over any `Monad` — collecting a list of monadic values into a monadic list — without knowing whether it is operating on `Option`, `Vec`, or `Result`. This is not possible with Rust's type system without GATs.

## OCaml Approach

OCaml has native higher-kinded types through its module system — functors (module-level functions) take a module implementing `Map.S` and produce a specialized module. More directly, OCaml's polymorphic variants and first-class modules allow `Functor.map : ('a -> 'b) -> 'a t -> 'b t` where `t` is the container type. The `ppx_jane` library and `Base` provide `Monad` signatures that work exactly like Haskell's, without defunctionalization.

## Key Differences

1. **Native vs. simulated**: OCaml supports HKTs directly through module functors; Rust requires the defunctionalization workaround with GATs.
2. **Ergonomics**: OCaml's `List.map`, `Option.map` are instances of a unified `Map.S` signature; Rust's simulation requires explicit `OptionHKT`, `VecHKT` marker types.
3. **GAT stability**: Rust's GATs became stable in 1.65 (2022); prior Rust required more verbose workarounds with lifetime-parameterized associated types.
4. **Error messages**: Type errors in Rust's HKT simulation can be difficult to decipher; OCaml's functor errors tend to be clearer.

## Exercises

1. Implement `Functor` for `ResultHKT<E>` and verify that `fmap` over an `Ok` applies the function while `Err` passes through unchanged.
2. Add a `sequence` function: `fn sequence<M: Monad>(vec: Vec<M::Applied<A>>) -> M::Applied<Vec<A>>` and test it with `Option`.
3. Implement a `Traversable` trait that generalizes `sequence` to work over any `HKT` container, not just `Vec`.
