📖 **[View on hightechmind.io →](https://hightechmind.io/rust/236-coyoneda)**

---

# Coyoneda
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

While the Yoneda lemma gives a free Functor for any type constructor from the "contravariant" side, Coyoneda gives a free Functor from the "covariant" side. `Coyoneda<F, A>` is an existential type that wraps a value `F<B>` and a function `B -> A`, deferring the `fmap` until it is needed. The practical use: you can make any type constructor a Functor for free using Coyoneda, even if it does not natively support `map`. This is the foundation of the free applicative and free monad.

## Learning Outcomes

- Understand Coyoneda as an existential wrapper that provides free `map` for any type constructor
- Learn the construction: `Coyoneda<F, A> = exists B. (F<B>, B -> A)`
- See how Coyoneda accumulates `map` calls without evaluating the underlying functor
- Connect Coyoneda to free applicatives and free monads as building blocks

## Rust Application

`struct Coyoneda<F, A> { inner: Box<dyn Any>, func: Box<dyn Fn(Box<dyn Any>) -> A> }` — the existential `B` is erased via `Box<dyn Any>`. `map(coyoneda, f)` extends the function chain: `coyoneda.func = f ∘ coyoneda.func`. Running: `coyoneda.func(coyoneda.inner)` applies the accumulated function to the inner value. For any type `F<B>`, `lift<B>(fb: F<B>) -> Coyoneda<F, B>` lifts it into Coyoneda with the identity function.

## OCaml Approach

OCaml's Coyoneda uses GADT existentials:
```ocaml
type 'a 'f coyoneda = Coyoneda : 'b 'f * ('b -> 'a) -> 'a 'f coyoneda
let lift fb = Coyoneda (fb, Fun.id)
let map f (Coyoneda (fb, g)) = Coyoneda (fb, f >> g)
let run (Coyoneda (fb, f)) = map f fb  (* requires F to be a Functor *)
```
The GADT hides `'b` — the existential type — while preserving the function `'b -> 'a`. Map fusion is compositional: each `map` just extends the function.

## Key Differences

1. **GADT vs. Any**: OCaml's GADT existential keeps the type information in the pattern match; Rust's `Box<dyn Any>` erases it and requires downcasting.
2. **Free Functor**: Coyoneda provides a free functor for any `F` regardless of whether `F` supports `fmap`; this is useful for uninspectable data sources.
3. **Map fusion**: Both accumulate `map` calls into a function composition — O(1) per `map` call; evaluation is deferred until `run`.
4. **Free monad connection**: Coyoneda is used as the base for free monads when the underlying functor does not have a natural `fmap`.

## Exercises

1. Implement `map` for `Coyoneda<F, A>` and verify that applying 5 maps results in a single composed function.
2. Write `run: Coyoneda<Option, A> -> Option<A>` that evaluates the deferred computation.
3. Demonstrate that `Coyoneda` provides a valid `Functor` instance: verify the identity and composition laws.
