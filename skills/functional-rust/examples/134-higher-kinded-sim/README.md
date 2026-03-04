# 134: Higher-Kinded Types Simulation

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Write generic algorithms that work over any container — `Option`, `Vec`, `Result` — without duplicating code, by simulating the higher-kinded types Rust doesn't natively support.

## The Problem This Solves

In functional programming, a `Functor` is any container you can `map` over. `Option::map`, `Vec::iter().map()`, `Result::map` — all the same pattern, different containers. In OCaml or Haskell you can write a single `double_all` function that works for *any* functor, parameterized over the container type itself.

Rust doesn't support higher-kinded types natively. You can't write `fn double_all<F<_>>(xs: F<i32>) -> F<i32>` — Rust doesn't allow type-level functions (types that take types and return types). So you end up copy-pasting: one `double_all_option`, one `double_all_vec`, one `double_all_result`. That's not reuse.

The simulation uses a technique called *defunctionalization*: instead of parameterizing over `F<_>` directly, you introduce a marker type `OptionHKT` and a trait that maps `T` to `F<T>` via an associated type. It's indirect, but it works — you get genuinely generic code over container type.

## The Intuition

The core trick: instead of `F<T>`, write `F::Applied<T>`. Define a trait `HKT` with an associated type `Applied<T>`. For `OptionHKT`, `Applied<T>` is `Option<T>`. For `VecHKT`, `Applied<T>` is `Vec<T>`. Now your generic function takes `F: HKT` and works with `F::Applied<i32>` — which resolves to `Option<i32>` or `Vec<i32>` depending on the type argument.

Layer `Functor` on top of `HKT` to add the `map` operation. Layer `Monad` on top of `Functor` to add `flat_map` (the ability to chain operations that might fail or expand). Each new container type just needs an `impl` of these traits, and all generic algorithms work for free.

## How It Works in Rust

```rust
// Step 1: HKT trait — the "applied" trick
trait HKT {
    type Applied<T>;  // GAT (generic associated type): maps T → Container<T>
}

// Step 2: Marker types for each container
struct OptionHKT;
impl HKT for OptionHKT {
    type Applied<T> = Option<T>;  // OptionHKT::Applied<i32> = Option<i32>
}

struct VecHKT;
impl HKT for VecHKT {
    type Applied<T> = Vec<T>;     // VecHKT::Applied<i32> = Vec<i32>
}

// Step 3: Functor — generic map over any HKT
trait Functor: HKT {
    fn map<A, B>(fa: Self::Applied<A>, f: impl Fn(A) -> B) -> Self::Applied<B>;
}

impl Functor for OptionHKT {
    fn map<A, B>(fa: Option<A>, f: impl Fn(A) -> B) -> Option<B> { fa.map(f) }
}

impl Functor for VecHKT {
    fn map<A, B>(fa: Vec<A>, f: impl Fn(A) -> B) -> Vec<B> {
        fa.into_iter().map(f).collect()
    }
}

// Step 4: Write ONE function that works for any Functor
fn double_all<F: Functor>(xs: F::Applied<i32>) -> F::Applied<i32> {
    F::map(xs, |x| x * 2)
}

// Step 5: Monad adds flat_map (sequencing, chaining)
trait Monad: Functor {
    fn pure<A>(a: A) -> Self::Applied<A>;
    fn flat_map<A, B>(fa: Self::Applied<A>, f: impl Fn(A) -> Self::Applied<B>) -> Self::Applied<B>;
}

impl Monad for OptionHKT {
    fn pure<A>(a: A) -> Option<A> { Some(a) }
    fn flat_map<A, B>(fa: Option<A>, f: impl Fn(A) -> Option<B>) -> Option<B> { fa.and_then(f) }
}
```

Usage:
```rust
// Same function, different containers — no duplication
let doubled_vec = double_all::<VecHKT>(vec![1, 2, 3]);   // [2, 4, 6]
let doubled_opt = double_all::<OptionHKT>(Some(21));       // Some(42)

// Monadic chaining: safe division then safe sqrt
fn safe_div(a: i32, b: i32) -> Option<i32> { if b == 0 { None } else { Some(a / b) } }

let result = OptionHKT::flat_map(Some(10), |x|
    OptionHKT::flat_map(safe_div(x, 2), safe_sqrt)
);  // Some(2.236...)
```

## What This Unlocks

- **Generic combinators** — write `sequence`, `traverse`, `zip_with` once and use them with any container that implements `Functor`/`Monad`.
- **Testing abstractions** — implement a test container type (`IdentityHKT`) that runs without effects, letting you unit-test code that would normally use `Future` or `IO`.
- **Effect systems** — advanced libraries simulate effect polymorphism using this technique, letting the same business logic run synchronously or asynchronously.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Higher-kinded types | Native: `module type FUNCTOR = sig type 'a t val map : ... end` | Simulated: `trait HKT { type Applied<T>; }` via GATs |
| Functor definition | Module type with `type 'a t` | Trait `Functor: HKT` with `fn map` |
| Generic algorithms | Module functor `module DoubleAll(F: FUNCTOR)` | Generic function `fn double_all<F: Functor>` |
| Monad | `MONAD` module type extending `FUNCTOR` | Trait `Monad: Functor` with `pure` and `flat_map` |
