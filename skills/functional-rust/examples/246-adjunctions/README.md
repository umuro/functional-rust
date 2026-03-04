# 246: Adjunctions

**Difficulty:** 5  **Level:** Master

Two operations are "adjoint" when converting in one direction is always equivalent to converting in the other — the most elegant way to explain why currying, monads, and duality work the way they do.

## The Problem This Solves

Why does `curry` undo `uncurry` and vice versa? Why does the `Vec` monad's `flat_map` feel like a "natural" generalization of `map`? Why do `State` and `Reader` monads feel like mirror images? Why does the list monad arise from "free" and "forgetful" functors?

These feel like unrelated coincidences until you see the pattern underneath: **adjunctions**. Every monad arises from an adjunction. Currying is an adjunction. Free structures (like `Vec`) are always adjoint to forgetful functors (like "just the set of elements"). Understanding adjunctions replaces a dozen separate explanations with one.

Without this concept, you end up with a zoo of "why does this particular pattern work?" questions answered case-by-case. With adjunctions, you see they're all the same thing viewed from different angles. The pattern is: two operations `F` and `G` are adjoint when there's a perfect, invertible correspondence between "inputs to F's output" and "inputs to G's output." This exists to solve exactly that pain.

## The Intuition

Two functions `F` and `G` are **adjoint** (written F ⊣ G) when there's a natural correspondence:

> A function from `F(A)` to `B`  ≅  A function from `A` to `G(B)`

Read that again: *any* way to get from `F(A)` to `B` corresponds to *exactly one* way to get from `A` to `G(B)`, and vice versa. This is an **isomorphism** — both sides have the same information, just reorganized.

**The most concrete example you already know: currying.**

```
curry   :: (A, C) -> B    ≅    A -> (C -> B)
uncurry :: A -> (C -> B)  ≅    (A, C) -> B
```

Here `F(A) = (A, C)` (pairing with `C`) and `G(B) = C -> B` (functions from `C`). A function that takes a pair and returns `B` is equivalent to a function that takes `A` and returns a function `C -> B`. They carry identical information. `curry` and `uncurry` are the isomorphism.

This is the **Product ⊣ Exponential adjunction**. The "pairing with C" functor is adjoint to the "functions from C" functor. That's all currying is.

**The unit and counit:** Every adjunction has two natural transformations:
- **Unit** (`η: A -> G(F(A))`): inject `A` into the round-trip `G(F(A))`. For currying: `a -> (c -> (a, c))`.
- **Counit** (`ε: F(G(B)) -> B`): collapse the round-trip `F(G(B))` down to `B`. For currying: function application `((A -> B), A) -> B`.

They satisfy triangle identities: applying unit then counit gets you back. This is what makes `curry . uncurry = id` and `uncurry . curry = id`.

**How monads arise:** The composition `G ∘ F` is always a monad! For currying: `G(F(A)) = C -> (A, C)` — that's the State monad. Every monad comes from some adjunction. This is why `Vec` (the free monoid) is a monad: Free ⊣ Forgetful gives you the list monad.

## How It Works in Rust

```rust
// The Product ⊣ Exponential adjunction = curry/uncurry
// curry: Hom(A × C, B) -> Hom(A, C -> B)
pub fn curry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl Fn(A, C) -> B + 'static,
) -> impl Fn(A) -> Box<dyn Fn(C) -> B> {
    let f = Rc::new(f);
    move |a: A| {
        let a2 = a.clone();
        let f2 = f.clone();
        // Return a closure that captures 'a' — this IS the adjunction
        Box::new(move |c: C| f2(a2.clone(), c))
    }
}

// uncurry: Hom(A, C -> B) -> Hom(A × C, B)  (the inverse)
pub fn uncurry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl Fn(A) -> Box<dyn Fn(C) -> B> + 'static,
) -> impl Fn(A, C) -> B {
    move |a, c| (f(a))(c)  // apply then apply — the counit
}

// Verify the adjunction isomorphism holds:
let add = |a: i32, b: i32| a + b;
let add_rt = uncurry(curry(add));
assert_eq!(add_rt(3, 4), 7);  // uncurry(curry(f)) = f ✓

// The Free ⊣ Forgetful adjunction = list monad
// Unit of the adjunction: a -> [a]  (inject into free structure)
pub fn free_unit<A>(a: A) -> Vec<A> { vec![a] }

// Counit: Vec<Vec<A>> -> Vec<A>  (flatten = fold the free structure)
pub fn free_counit<A>(vv: Vec<Vec<A>>) -> Vec<A> {
    vv.into_iter().flatten().collect()
}

// The monad arising from Free ⊣ Forgetful:
// return = free_unit, bind = flat_map
pub fn list_bind<A: Clone, B>(xs: Vec<A>, f: impl Fn(A) -> Vec<B>) -> Vec<B> {
    xs.into_iter().flat_map(f).collect()
}

// Option monad — also arises from an adjunction
// return = Some, join = flatten
pub fn option_join<A>(oa: Option<Option<A>>) -> Option<A> { oa.flatten() }
```

The key insight in `curry`: the closure `move |c| f(a, c)` is exactly the adjunction at work — it "stores" `a` and waits for `c`. The adjoint "reorganizes" where the arguments live.

## What This Unlocks

- **Monad derivation:** Every monad you use (`Vec`, `Option`, `Result`, `State`, `Reader`) arises from an adjunction. Understanding the underlying adjunction explains why `return`/`bind` have the shapes they do.
- **Duality:** Monad and comonad are dual by adjunction. State monad ↔ Store comonad is an adjunction. This explains why they're mirror images structurally.
- **Free structures:** Whenever you see a "free" construction (free monad, free monoid = `Vec`, free group), it's the left adjoint to a forgetful functor. This is why `flat_map` on `Vec` has the structure it does.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `curry` type | `('a * 'c -> 'b) -> 'a -> 'c -> 'b` | `impl Fn(A, C) -> B` → `impl Fn(A) -> Box<dyn Fn(C) -> B>` |
| Returning closures | First-class, no boxing needed | Must box (`Box<dyn Fn(C) -> B>`) due to size requirements |
| `Rc` for sharing | Not needed (GC handles it) | `Rc::new(f)` to share `f` across multiple closures |
| Unit/counit | Natural transformations between polymorphic types | Concrete functions; no higher-kinded types |
| Triangle identities | Equational proofs | Tested via `assert_eq!(uncurry(curry(f))(a, b), f(a, b))` |
| Monad derivation | `module StateMonad = Compose(G)(F)` | Demonstrated separately (see example 249 for State monad) |
