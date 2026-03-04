# 248: Kan Extensions

**Difficulty:** 5  **Level:** Master

The most universal constructions in category theory — every concept is a Kan extension.

## The Problem This Solves

You have a list `[1, 2, 3]` and you apply `.map(f).map(g)`. That's two passes over the data. The Yoneda lemma tells you these can be fused into one pass: `.map(|x| g(f(x)))`. But more powerfully, Kan extensions generalize this fusion principle to *any functor*, not just lists.

The **codensity monad** (the right Kan extension of a functor along itself) gives you the same thing for monadic bind chains. A sequence of left-nested `flat_map` calls on a list is O(n²) because each intermediate list is built and then immediately consumed. Representing the computation as a codensity continuation makes it O(n) — the intermediate lists are never materialized.

This matters in practice whenever you build query engines, effect systems, or parser combinators where operations are composed before being run.

## The Intuition

**Kan extensions** answer the question: "I have a functor F from category C to E, and another functor K from C to D. I want to extend F along K to get a functor from D to E. What's the best approximation?"

There are two dual answers:

- **Right Kan extension (Ran K F)**: "For each object `d` in D, give me all ways to turn a morphism `d → K(c)` into `F(c)`." In types: `Ran K F (d) = forall c. (d -> K(c)) -> F(c)`. It's the "most efficient" extension — uses only what the morphism tells it.

- **Left Kan extension (Lan K F)**: "For each object `d`, give me an object `c` in C together with a morphism `K(c) → d` and a value `F(c)`." It's the "most general" extension — collapses all the witnesses together.

**The Yoneda lemma as a special case**: When `K = Id` (the identity functor), `Ran Id F ≅ F`. In types: `forall r. (a -> r) -> F(r)` is naturally isomorphic to `F(a)`. This means any functor `F` can be represented as a continuation — and mapping over the continuation fuses automatically.

**The codensity monad**: `Ran F F` is a monad whenever `F` is a functor. For `F = Vec`, the codensity monad represents list computations as continuations. Multiple `flat_map` calls compose as continuation composition — O(1) per bind — and the list is only materialized once at the end via `lower()`.

Think of it like `StringBuilder` vs string concatenation: naive string concatenation is O(n²) because each `+` copies. `StringBuilder` buffers all the operations and concatenates once. Codensity is the monadic version of this pattern.

## How It Works in Rust

The codensity monad for `Vec`:
```rust
pub struct Codensity<A: 'static> {
    // Represents: forall r. (A -> Vec<r>) -> Vec<r>
    // Stored as a one-shot closure (FnOnce) specialized to R = A
    run: Box<dyn FnOnce(&dyn Fn(A) -> Vec<A>) -> Vec<A>>,
}

impl<A: Clone + 'static> Codensity<A> {
    // Lift: wrap a Vec<A> as a lazy continuation
    pub fn lift(xs: Vec<A>) -> Self {
        Codensity { run: Box::new(move |k| xs.into_iter().flat_map(k).collect()) }
    }

    // Lower: run the continuation with identity to get the Vec back
    pub fn lower(self) -> Vec<A> {
        (self.run)(&|x| vec![x])
    }
}

// Bind composes continuations — NO intermediate lists
pub fn bind_codensity<A: Clone + 'static>(
    m: Codensity<A>,
    f: impl Fn(A) -> Codensity<A> + 'static,
) -> Codensity<A> {
    Codensity {
        run: Box::new(move |k| {
            (m.run)(&|a| {
                let inner = f(a);
                (inner.run)(k)  // compose: k is passed through, not applied twice
            })
        }),
    }
}
```

Yoneda round-trip (right Kan extension along identity):
```rust
// to_ran converts Vec<A> to its Yoneda representation
fn to_ran<A: Clone>(xs: Vec<A>) -> impl Fn(&dyn Fn(A) -> A) -> Vec<A> {
    move |k| xs.iter().cloned().map(k).collect()
}

// from_ran recovers Vec<A> by applying the identity function
fn from_ran<A: Clone>(ran: impl Fn(&dyn Fn(A) -> A) -> Vec<A>) -> Vec<A> {
    ran(&|x| x)
}

// Map fusion: applying two functions inside Ran = one pass, not two
let fused = to_ran(vec![1, 2, 3, 4, 5])(&|x| x * 2 + 1);
// Equivalent to .map(|x| x*2).map(|x| x+1) but fused
```

## What This Unlocks

- **Efficient bind chains**: Codensity monad turns O(n²) left-nested monadic chains into O(n) — critical for query engines, parser combinators, and effect systems.
- **Free monad optimization**: the "final encoding" of free monads uses Kan extensions to avoid the O(n²) performance cliff of naive free monad bind.
- **Functor fusion**: the Yoneda lemma lets you fuse multiple `.map()` calls into one without changing semantics — proof that the optimizer is doing the right thing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Codensity type | `type 'a codensity = { run: 'r. ('a -> 'r list) -> 'r list }` | `struct Codensity<A> { run: Box<dyn FnOnce(...)> }` — no rank-2 types, specialized |
| Rank-2 polymorphism | First-class via `type 'r.` syntax | Not stable; workaround: specialize R or use trait objects |
| Yoneda `forall r` | Handled by the type system natively | Must fix R or use `Box<dyn Fn>` with trait objects |
| Bind fusion | Automatic via continuation composition | Same structure; requires `FnOnce` + `Box` for single-use |
| Left Kan (Lan) | `type ('f, 'a) lan = Lan : ('f 'b -> 'a) * 'b -> ('f, 'a) lan` | Needs existential types; achievable with trait objects |
