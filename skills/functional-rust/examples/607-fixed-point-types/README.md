# 607: Fixed-Point Types and Recursion Schemes

**Difficulty:** 5  **Level:** Master

Express recursive types as `Fix<F>` — the fixed point of a functor — to get catamorphism, anamorphism, and hylomorphism for free.

## The Problem This Solves

Every time you define a recursive type (`List`, `Tree`, `Expr`), you end up writing the same fold and unfold patterns from scratch. A tree fold and a list fold look structurally identical — but there's no abstraction that captures both without duplicating the recursion logic.

The root cause is that recursive types bake recursion into their definition: `enum List<A> { Nil, Cons(A, Box<List<A>>) }`. What if you separated the "shape" of the type from the "recursion" itself? Define a non-recursive base functor `ListF<A, R>` where `R` is a placeholder for the recursive position. Then `Fix<ListF<A>>` is the recursive type.

This separation is the key insight of recursion schemes: write your algebra once (`T → A`), apply `cata` to get a fold over any `Fix<F>` type. The recursive plumbing (traverse the structure, apply the algebra) is shared across all types. `Fix` is not academic — it's the formal basis for `serde`, AST transformations, and any library that needs generic traversal.

## The Intuition

`Fix<F>` is the type where `Fix<F> = F<Fix<F>>` — it's the type that equals its own functor application, which is exactly what a recursive type is. The payoff: define your type as a non-recursive functor `F`, and you get `cata` (fold) and `ana` (unfold) for free, without re-implementing the recursion each time.

## How It Works in Rust

```rust
// The fixed-point wrapper — wraps one layer of F
struct Fix<F: Functor>(Box<F::Applied<Fix<F>>>);

// A trait for types that can map their recursive position
trait Functor {
    type Applied<T>;
    fn fmap<A, B>(fa: Self::Applied<A>, f: impl Fn(A) -> B) -> Self::Applied<B>;
}

// Base functor for List<A> — R is the recursive position
enum ListF<A, R> {
    Nil,
    Cons(A, R),
}

// Manual Fixed-point for List — simplified (Rust GATs make this easier in nightly)
enum ListFix<A> {
    Nil,
    Cons(A, Box<ListFix<A>>),  // in practice: Fix<ListF<A>>
}

impl<A> ListFix<A> {
    // Catamorphism: fold using an algebra (ListF<A, B> → B)
    fn cata<B>(self, alg: &impl Fn(Option<(A, B)>) -> B) -> B {
        match self {
            ListFix::Nil        => alg(None),
            ListFix::Cons(a, t) => {
                let tb = t.cata(alg);   // recurse first — bottom-up
                alg(Some((a, tb)))       // then apply algebra
            }
        }
    }
}

// Using cata — algebra defines the computation, recursion is handled by cata
let list = ListFix::Cons(1, Box::new(ListFix::Cons(2, Box::new(ListFix::Nil))));
let sum = list.cata(&|node| match node {
    None        => 0,
    Some((a, b)) => a + b,
});
```

## What This Unlocks

- **Generic traversals**: write `cata` once per type; all folds share the structure.
- **Anamorphisms** (`ana`): the dual — unfold a seed value into a recursive structure.
- **Hylomorphisms** (`hylo = cata . ana`): build and immediately fold — fuses two traversals into one without intermediate allocation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fixed point | `type 'f fix = Fix of 'f fix 'f` | `struct Fix<F>(Box<F::Applied<Fix<F>>>)` |
| Base functor | Type parameter in functor | `trait Functor` with `type Applied<T>` |
| Recursive position | Type variable | GAT `Applied<T>` (stable since 1.65) |
| `cata` | `let rec cata alg = function ...` | Method on `Fix<F>` or free function |
| Boxing overhead | GC-managed | `Box<T>` — one heap alloc per node |
| Practical use | `ppx_deriving` can generate folds | Manual or via `recursion` crate |
