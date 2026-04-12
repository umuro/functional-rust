📖 **[View on hightechmind.io →](https://hightechmind.io/rust/212-van-laarhoven)**

---

# Van Laarhoven Lenses
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The Van Laarhoven encoding represents all lens operations as a single function type: `type Lens s a = forall f. Functor f => (a -> f a) -> s -> f s`. Choosing different functors selects different operations: `Identity` functor gives `over`, `Const r` functor gives `view`. The profound payoff: **lens composition is plain function composition** (`f . g`). No special composition operator is needed. This is why Haskell's `lens` library can compose optics with `(.)`.

## Learning Outcomes

- Understand the Van Laarhoven encoding: one function type for all lens operations
- Learn how `Identity` and `Const` functors select different lens operations
- See why composition is plain function composition in this encoding
- Appreciate the mathematical elegance and understand the limitation in Rust (no rank-2 types)

## Rust Application

Rust lacks rank-2 types, so the full Van Laarhoven encoding is impossible. The simulation specializes: `VLLens<S, A>` bundles an `over_fn` (using `Identity`) and a `view_fn` (using `Const`). Composition `compose(outer, inner)` produces a lens where `view` = `outer.view_fn(inner.view_fn(s))` and `over` applies `outer.over_fn` inside `inner.over_fn`. The composition is genuinely function composition — no recursion or manual threading.

## OCaml Approach

OCaml simulates Van Laarhoven more faithfully using rank-2 record polymorphism:
```ocaml
type ('s, 'a) lens = {
  runLens : 'f. (module FUNCTOR with type 'a t = 'f) -> ('a -> 'f) -> 's -> 'f
}
```
This requires first-class modules for the functor parameter. Haskell's `lens` library uses type class constraints for zero overhead. Neither OCaml nor Rust achieves Haskell's full ergonomics for Van Laarhoven lenses.

## Key Differences

1. **Rank-2 requirement**: Full Van Laarhoven requires rank-2 polymorphism; Rust and OCaml approximate it; Haskell supports it natively.
2. **Composition**: The central benefit — composition as function composition — is preserved in both the Rust and OCaml simulations.
3. **Functor selection**: Haskell uses type class resolution to select `Identity` vs. `Const`; Rust and OCaml bundle both explicitly in the lens struct.
4. **Practical use**: Rust uses the simple `get`/`set` lens (examples 202-205) in production; Van Laarhoven is studied for its mathematical elegance.

## Exercises

1. Implement composition for `VLLens` and verify it produces correct `view` and `over` results for a three-level nested structure.
2. Show that `view(compose(l1, l2), s) == l1.view(l2.view(s))` for concrete lens examples.
3. Implement `identity_lens: VLLens<A, A>` where `view` returns the value itself and `over` applies the function directly.
