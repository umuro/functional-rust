# 212: Van Laarhoven Lenses

**Difficulty:** 3  **Level:** Advanced

Represent lenses as polymorphic functions over functors — compose them with plain function composition.

## The Problem This Solves

Standard lenses store `get` and `set` as two separate functions. When you compose them, you manually wire each level. The more lenses you compose, the more bookkeeping you do. And each operation (get, set, modify) requires branching to the right function.

The Van Laarhoven encoding collapses all lens operations into one function type: `(a -> f b) -> (s -> f t)`. You pick the functor `f` to select the operation: Identity functor gives you `modify`, Const functor gives you `get`. One lens type, every operation.

The deeper payoff: composition is free. Two Van Laarhoven lenses compose with ordinary function composition — no special `compose_lens` needed. This is why Haskell's `lens` library works the way it does, and why understanding it unlocks the entire optics ecosystem.

## The Intuition

A Van Laarhoven lens is a function that says: "I know how to reach inside an `s` and do something with the `a` inside it, whatever 'something' means — you tell me by giving me a function `a -> f b`."

The magic is the functor `f`. If `f` is `Identity` (just wraps a value), the lens returns a modified structure. If `f` is `Const` (ignores the second argument, just holds a read value), the lens returns the extracted value. Same lens code, different behavior depending on what you plug in.

Rust can't express this directly because it would require "for any functor f" — a rank-2 type. So the practical encoding stores two operations: an `over` function and a `get` function. Composition still chains the `over` functions.

## How It Works in Rust

```rust
// Identity functor: wraps a value, used for modify/over
struct Identity<A>(A);
impl<A> Functor for Identity<A> {
    type Inner = A;
    fn map<B>(self, f: impl FnOnce(A) -> B) -> Identity<B> { Identity(f(self.0)) }
}

// Const functor: ignores map, used for get (holds the read value)
struct Const<A, B>(A, PhantomData<B>);
impl<A: Clone, B> Functor for Const<A, B> {
    type Inner = B;
    fn map<C>(self, _f: impl FnOnce(B) -> C) -> Const<A, C> {
        Const(self.0, PhantomData) // unchanged — _f is never called
    }
}

// Practical encoding: store get and over separately
struct VLLens<S, A> {
    over_fn: Box<dyn Fn(&dyn Fn(&A) -> A, &S) -> S>, // modify
    get_fn:  Box<dyn Fn(&S) -> A>,                    // read
}

impl<S: 'static, A: 'static> VLLens<S, A> {
    fn get(&self, s: &S) -> A { (self.get_fn)(s) }
    fn over(&self, f: impl Fn(&A) -> A, s: &S) -> S { (self.over_fn)(&f, s) }
    fn set(&self, a: A, s: &S) -> S where A: Clone {
        self.over(move |_| a.clone(), s)
    }
}

// Composition: chain over functions — the key advantage
fn compose<S, A, B>(outer: VLLens<S, A>, inner: VLLens<A, B>) -> VLLens<S, B> {
    // ... wires outer.over with inner.over
}
```

## What This Unlocks

- **Free composition** — no custom `compose_lens` function; Van Laarhoven lenses compose via function composition, enabling deep nesting with zero overhead.
- **Optics family unification** — Traversals, Prisms, and Folds all share the same encoding shape; understanding VL lenses explains the whole optics hierarchy.
- **Library interop** — the `lens` crate and similar libraries use this encoding; reading it unlocks the source of widely-used Rust/Haskell optic libraries.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Rank-2 types | Not available — use module functors | Not available — use trait encoding |
| Functor | Module signature `FUNCTOR` | `Functor` trait with GATs |
| Composition | Function composition `(.)` | Closure chaining in `compose()` |
| Practical encoding | Record of `run_identity` + `run_const` | Struct with two `Box<dyn Fn>` |
| Identity functor | Defined as module | `struct Identity<A>(A)` with `Functor` impl |
