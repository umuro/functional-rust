📖 **[View on hightechmind.io →](https://hightechmind.io/rust/235-yoneda-lemma)**

---

# Yoneda Lemma
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The Yoneda lemma is one of the most important results in category theory: `Nat(Hom(A, -), F) ≅ F(A)`. In Rust terms: a natural transformation from `fn(A, -)` to any functor `F` is in one-to-one correspondence with a value of `F(A)`. The practical consequence: `Yoneda<F, A>` — a type holding a natural transformation — is isomorphic to `F(A>` but enables free `map` operations without actually running them, accumulating a composition chain that is applied all at once.

## Learning Outcomes

- Understand the Yoneda lemma as a correspondence between natural transformations and functor values
- Learn how `Yoneda<F, A>` defers and fuses `map` operations (Yoneda's performance benefit)
- See the proof: `to_yoneda(fa)(id) = fa` and `from_yoneda(y) = y(id)`
- Connect to free `Functor` instances: any type with a `Yoneda` wrapper gets `map` for free

## Rust Application

`struct Yoneda<F, A> { run: Box<dyn Fn(Box<dyn Fn(A) -> B>) -> F<B>> }` — but Rust lacks HKTs so this requires the defunctionalization trick from example 134. The practical insight: accumulating `n` `map(f1).map(f2).map(f3)` calls on `Yoneda` is `O(1)` in traversals — it builds a composed function. Converting back with `from_yoneda(y) = y(id)` applies all accumulated maps in one pass. This is the foundation of Haskell's stream fusion.

## OCaml Approach

OCaml's Yoneda:
```ocaml
type ('f, 'a) yoneda = { run : 'b. ('a -> 'b) -> 'b 'f }
let to_yoneda fa = { run = fun f -> map f fa }
let from_yoneda y = y.run Fun.id
let map_yoneda f y = { run = fun g -> y.run (g >> f) }
```
`map_yoneda` fuses the function `f` into the natural transformation without actually running it. OCaml's rank-2 type `'b.` enables the polymorphic `run` field.

## Key Differences

1. **Rank-2 types**: OCaml's `'b.` record polymorphism expresses `Yoneda` directly; Rust requires defunctionalization (GAT markers).
2. **Performance benefit**: Both languages benefit from Yoneda map fusion; Haskell's library uses it for stream fusion; Rust's iterator fusion is an analogous (compiler-level) optimization.
3. **Free Functor**: `Yoneda` provides a free `Functor` instance for any type constructor — even non-functors can be mapped over via `Yoneda` lifting.
4. **Practical use**: Yoneda is primarily educational; the performance benefit is achieved automatically by LLVM in Rust for iterator chains.

## Exercises

1. Implement `to_yoneda` and `from_yoneda` for `Option<A>` and verify the round-trip: `from_yoneda(to_yoneda(fa)) == fa`.
2. Demonstrate map fusion: apply 10 `map` operations via `Yoneda` and verify only one actual traversal occurs.
3. Prove the Yoneda isomorphism: `to_yoneda(from_yoneda(y)) == y` for a specific `y`.
