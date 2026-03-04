# 239: Strong Profunctor

**Difficulty:** ⭐⭐⭐⭐  **Level:** Category Theory

A strong profunctor is a function-like thing that can "carry extra context" — the categorical foundation of lenses and the entire optics hierarchy.

## The Problem This Solves

You have a function `fn(Name) -> Name` (transform a name). You want to use it to transform the `name` field inside a `User` struct, leaving all other fields unchanged. Normally you'd write boilerplate: get the field, apply the function, set it back. With lenses, this is automatic and composable.

But what makes a lens work at the type level? The answer is Strong profunctors. A **profunctor** is like a function that can be mapped on both input (contravariantly) and output (covariantly). A **strong** profunctor additionally knows how to "pass along" extra context — the other fields it doesn't touch — through the transformation. This is exactly the capability lenses need: focus on part `A` inside whole `S`, transform `A`, and reconstruct `S` with everything else unchanged.

Understanding strong profunctors explains *why* the lens laws are the laws they are and why lenses compose — composition of profunctor morphisms.

## The Intuition

A **profunctor** `P<A, B>` is like a transformer from `A` to `B`, but it can be adapted at both ends:
- **Contramap input**: if you have `P<A, B>` and `f: C -> A`, you get `P<C, B>` — preprocess the input
- **Map output**: if you have `P<A, B>` and `g: B -> D`, you get `P<A, D>` — postprocess the output

The canonical profunctor is just `fn(A) -> B`.

A **strong** profunctor additionally has:
- `first`: given `P<A, B>`, get `P<(A, C), (B, C)>` — "I'll transform the `A` part, carry `C` unchanged"
- `second`: given `P<A, B>`, get `P<(C, A), (C, B)>` — "I'll transform the `A` part, carry `C` on the left"

Think of `first` as: "I'm a camera lens focused on the first element of a pair. I transform it; the second element passes through untouched."

**A lens** is then: `Lens<S, A>` = for any strong profunctor `P`, lift `P<A, A>` to `P<S, S>`. Concretely, it needs a `get: S -> A` (extract the field) and a `set: (S, A) -> S` (put back a new value). The `strong` capability provides the "carry the rest of S through" machinery.

## How It Works in Rust

```rust
/// The canonical strong profunctor: just a function
pub struct Mapper<A, B> {
    f: Box<dyn Fn(A) -> B>,
}

impl<A: 'static, B: 'static> Mapper<A, B> {
    /// first: lift P(A,B) to P((A,C),(B,C)) — C passes through untouched
    pub fn first<C: 'static>(self) -> Mapper<(A, C), (B, C)> {
        let f = self.f;
        Mapper::new(move |(a, c)| (f(a), c))  // transform a, carry c
    }

    /// second: lift P(A,B) to P((C,A),(C,B)) — C passes through on the left
    pub fn second<C: 'static>(self) -> Mapper<(C, A), (C, B)> {
        let f = self.f;
        Mapper::new(move |(c, a)| (c, f(a)))  // carry c, transform a
    }

    /// dimap: adapt both input and output
    pub fn dimap<C: 'static, D: 'static>(
        self,
        pre:  impl Fn(C) -> A + 'static,  // preprocess input
        post: impl Fn(B) -> D + 'static,  // postprocess output
    ) -> Mapper<C, D> {
        let f = self.f;
        Mapper::new(move |c| post(f(pre(c))))
    }
}

/// A lens: focus on field A inside whole S
pub struct Lens<S, A> {
    pub get: Box<dyn Fn(&S) -> A>,
    pub set: Box<dyn Fn(S, A) -> S>,
}

impl<S: Clone + 'static, A: Clone + 'static> Lens<S, A> {
    /// Apply a Mapper<A,A> through the lens to get Mapper<S,S>
    pub fn apply(&self, mapper: Mapper<A, A>) -> Mapper<S, S> {
        let get = self.get.clone_box();  // S -> A
        let set = self.set.clone_box();  // (S, A) -> S
        // Use first/dimap to: extract A, transform it, put it back
        mapper
            .dimap(
                move |s: S| (get(&s), s),     // S -> (A, S)  -- extract + carry whole
                move |(new_a, s)| set(s, new_a), // (A, S) -> S  -- reconstruct
            )
    }
}
```

## What This Unlocks

- **Composable lenses** — `lens_a.compose(lens_b)` focuses deeper into nested structures. Composition works because profunctor morphisms compose.
- **The optics hierarchy** — Lens (Strong), Prism (Choice profunctor), Traversal (both) — each optic is characterized by what profunctor capability it requires. Understanding Strong explains the Lens tier.
- **Generic transformations** — write one function `transform<P: Strong>(p: P<A, A>) -> P<S, S>` and it works for any strong profunctor — functions, state transformers, indexed operations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Profunctor | Module type `PROFUNCTOR` | `trait Profunctor` / concrete `Mapper<A,B>` |
| Strong extension | `first` / `second` in module sig | Methods on `Mapper` |
| Lens encoding | Van Laarhoven `(a -> f a) -> s -> f s` | `struct Lens { get, set }` + `apply` |
| Dimap | Higher-kinded function | Method on `Mapper` with owned `self` |
| Composition | Module functor | Method chaining / `compose` function |
