# 247: Limits and Colimits

**Difficulty:** ⭐⭐⭐  **Level:** Category Theory

Every data type you've ever written is either a limit or a colimit — universal constructions that define types by their relationship to all other types.

## The Problem This Solves

Why does `(A, B)` feel like the "natural" way to hold both an `A` and a `B`? Why is `Either<A, B>` the "natural" way to hold one or the other? Category theory answers this: tuples are *products* (binary limits) and Either-types are *coproducts* (binary colimits). The "natural" feeling comes from the *universal property* — a formal statement that `(A, B)` is the unique type (up to isomorphism) that has projections to both `A` and `B` and factors through every other type that does too.

Understanding limits and colimits unifies all of Rust's fundamental types under one framework. Products, tuples, the unit type `()`, structs, equalizers (filter-like operations), and pullbacks (joins) are all limits. Coproducts, enums, the Never type `!`, and pushouts are all colimits. They're duals of each other.

## The Intuition

A **limit** is the "most specific" type that maps *to* a collection of types. It combines information from multiple types in the most general way possible.

- **Product** `(A, B)`: the limit of two objects. Has projections `fst: (A,B)->A` and `snd: (A,B)->B`. Any other type `X` with functions to both `A` and `B` factors *uniquely* through `(A, B)`. This is the universal property.
- **Terminal object** `()`: the limit of the *empty* diagram. There's exactly one function `f: T -> ()` for any type `T`. "Everything maps to unit."
- **Equalizer**: the limit of two parallel arrows `f, g: A -> B`. It's the subset of `A` where `f(a) == g(a)`. Concretely: filter by a condition that equates two functions.
- **Pullback**: the limit of a span `A -> C <- B`. It's the set of pairs `(a, b)` where `f(a) == g(b)`. Concretely: a database join.

A **colimit** is the dual — the "most general" type that has maps *from* a collection:

- **Coproduct** `Either<A, B>`: has injections `inl: A -> Either<A,B>` and `inr: B -> Either<A,B>`. Any type `C` receiving functions from both `A` and `B` factors through `Either`. This is `match`.
- **Initial object** `!` (Never): the colimit of the empty diagram. There's exactly one function `f: ! -> T` for any `T`. "Nothing maps from Never."

**Duality**: every limit statement flips to a colimit statement by reversing all arrows. Product ↔ Coproduct. Terminal ↔ Initial. Equalizer ↔ Coequalizer. Pullback ↔ Pushout.

## How It Works in Rust

```rust
// Product (binary limit): (A, B) with projections
fn proj1<A: Clone, B>(p: &(A, B)) -> A { p.0.clone() }
fn proj2<A, B: Clone>(p: &(A, B)) -> B { p.1.clone() }

// Universal property: any X -> A and X -> B factors through (A, B)
fn pair_of<X: Clone, A, B>(f: impl Fn(X)->A, g: impl Fn(X)->B) -> impl Fn(X)->(A,B) {
    move |x| (f(x.clone()), g(x))  // the unique factoring morphism
}

// Coproduct (binary colimit): Either<A, B> with injections
enum Coprod<A, B> { Inl(A), Inr(B) }

fn inl<A, B>(a: A) -> Coprod<A, B> { Coprod::Inl(a) }
fn inr<A, B>(b: B) -> Coprod<A, B> { Coprod::Inr(b) }

// Universal property: any f:A->X and g:B->X factors through Either
fn copair<A, B, X>(f: impl Fn(A)->X, g: impl Fn(B)->X) -> impl Fn(Coprod<A,B>)->X {
    move |e| match e {
        Coprod::Inl(a) => f(a),  // this IS the match — it's the unique factoring morphism
        Coprod::Inr(b) => g(b),
    }
}

// Equalizer (limit of parallel arrows): filter where f(x) == g(x)
fn equaliser<A: Clone, B: PartialEq>(
    xs: &[A], f: impl Fn(&A)->B, g: impl Fn(&A)->B
) -> Vec<A> {
    xs.iter().filter(|x| f(x) == g(x)).cloned().collect()
}

// Pullback (limit of span A->C<-B): pairs where f(a) == g(b)
fn pullback<'a, A: Clone, B: Clone, C: PartialEq>(
    as_: &[A], bs: &'a [B], f: impl Fn(&A)->C, g: impl Fn(&B)->C
) -> Vec<(A, B)> {
    as_.iter().flat_map(|a|
        bs.iter().filter(|b| f(a) == g(b)).map(|b| (a.clone(), b.clone()))
    ).collect()
}
```

## What This Unlocks

- **Unified data modeling** — every struct is a product (limit); every enum is a coproduct (colimit). The type system is built from these two constructions and nothing else.
- **Database queries as colimits** — joins are pullbacks, unions are coproducts. SQL's relational algebra IS category theory applied to sets.
- **Duality for free** — whenever you understand a limit construction, you immediately have the colimit dual. Product ↔ coproduct gives you structs ↔ enums. Terminal ↔ initial gives you `()` ↔ `!`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Product | Tuple `(a * b)` or record | `(A, B)` or struct |
| Coproduct | `type t = A of a \| B of b` | `enum Coprod<A, B> { Inl(A), Inr(B) }` |
| Terminal | `unit` / `()` | `()` |
| Initial / Never | `'a` (empty type) | `!` / `std::convert::Infallible` |
| Universal property | Proven by construction | Encoded in generic function signature |
