# 601: Coproduct / Sum Types

**Difficulty:** 5  **Level:** Master

Model "one of these alternatives" using the categorical coproduct — Rust's `enum` is a coproduct with injection and elimination morphisms.

## The Problem This Solves

You often have data that can be one of several distinct alternatives: a result is either `Ok` or `Err`; an AST node is a number, variable, or operation; a network packet is a control frame or a data frame. Representing this with a struct + boolean flags or a nullable union type loses type safety — the type doesn't tell you which case you're in, so the compiler can't ensure you handle all cases.

Category theory gives this a precise formulation: the coproduct `A + B` is a type inhabited by either an `A` or a `B`, with injection functions `inl: A → A+B` and `inr: B → A+B`. The *universal property* says: for any type `C` and functions `f: A→C` and `g: B→C`, there is a *unique* function `either(f, g): A+B → C`. This unique function is `match`.

Understanding coproducts as a categorical structure means you can reason about them algebraically: `Either<A, Void>` is isomorphic to `A`; `Either<A, B>` is isomorphic to `Either<B, A>`; `Either<A, Either<B, C>>` is isomorphic to `Either<Either<A, B>, C>`. These are the same algebraic identities as for addition.

## The Intuition

The coproduct `A + B` means "exactly one of A or B, tagged so you know which" — Rust's `enum` is precisely this, with enum constructors as injections and `match` as the unique elimination morphism guaranteed by the universal property. The trade-off: coproducts make adding new interpreters easy but adding new variants hard (you must update all match arms).

## How It Works in Rust

```rust
// Coproduct A + B: inhabited by either A or B
enum Either<A, B> {
    Left(A),   // injection inl: A → Either<A,B>
    Right(B),  // injection inr: B → Either<A,B>
}

impl<A, B> Either<A, B> {
    // The universal property: unique morphism from two functions
    // For any C, f: A→C, g: B→C, there is a unique Either<A,B>→C
    fn either<C, F, G>(self, f: F, g: G) -> C
    where F: FnOnce(A) -> C, G: FnOnce(B) -> C {
        match self {               // match IS the unique morphism
            Either::Left(a)  => f(a),
            Either::Right(b) => g(b),
        }
    }

    // Functor map over both sides
    fn map_left<C>(self, f: impl FnOnce(A) -> C) -> Either<C, B> {
        match self {
            Either::Left(a)  => Either::Left(f(a)),
            Either::Right(b) => Either::Right(b),
        }
    }
}

// Algebraic identity: Either<A, Void> ≅ A
enum Void {}  // uninhabited — no values of type Void exist

fn from_void_either<A>(e: Either<A, Void>) -> A {
    match e {
        Either::Left(a)  => a,
        Either::Right(v) => match v {},  // exhaustively handled — Void has no variants
    }
}
```

## What This Unlocks

- **Type-safe alternatives**: encode "success or error", "local or remote", "cached or fresh" with exhaustive pattern matching.
- **Algebraic reasoning**: use coproduct identities to refactor type representations without changing runtime behavior.
- **Free theorems**: the universal property guarantees that `either(f, g)` is the *only* function from `Either<A,B>` to `C` that factors through injections.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Coproduct A+B | `type t = L of A \| R of B` | `enum T { L(A), R(B) }` |
| Injection `inl` | `L : A -> t` constructor | `T::L(a)` |
| Injection `inr` | `R : B -> t` constructor | `T::R(b)` |
| Elimination | `match` | `match` |
| Universal morphism | Unique function from match | Same — `match` is the canonical eliminator |
| Uninhabited type | `type void = \|` (empty) | `enum Void {}` |
