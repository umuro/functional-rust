📖 **[View on hightechmind.io →](https://hightechmind.io/rust/621-profunctor-optics)**

---

# 621: Profunctor Encoding of Optics

**Difficulty:** 5  **Level:** Master

Encode Lens and Prism as higher-order functions over any profunctor — so that composition is just plain function composition, and all optic types collapse into one abstraction.

## The Problem This Solves

You have Lenses, Prisms, Traversals — all defined with different struct types and different composition rules. Composing a Lens with a Prism requires a special combinator. Composing a Lens with a Traversal requires another. Each combination has its own machinery.

When you build optics libraries, this combinatorial explosion becomes painful:

```rust
// Traditional encoding — different types, different composition logic
fn compose_lens_lens<S, A, B>(l1: Lens<S, A>, l2: Lens<A, B>) -> Lens<S, B> { ... }
fn compose_lens_prism<S, A, B>(l: Lens<S, A>, p: Prism<A, B>) -> AffineTraversal<S, B> { ... }
fn compose_prism_traversal<S, A, B>(p: Prism<S, A>, t: Traversal<A, B>) -> Traversal<S, B> { ... }
// ... and so on for every pair
```

The profunctor encoding solves this by making *all* optic types the same kind of thing: a higher-order function that transforms one profunctor into another. Composition of optics becomes plain function composition (`f ∘ g`). No special combinators, no type zoo. This exists to solve exactly that pain.

## The Intuition

In the profunctor encoding, an optic is just a function of this shape:

```
Optic<S, T, A, B> = forall P. P<A, B> -> P<S, T>
```

Read this as: "give me any profunctor `P` that knows how to handle `A → B`, and I'll give you a `P` that handles `S → T`."

Different optic types are different *constraints* on which profunctors they work with:

| Optic | Profunctor constraint | Meaning |
|-------|----------------------|---------|
| Iso | Any `Profunctor` | Works with all profunctors |
| Lens | `Strong` profunctor | Works with profunctors that can handle product types (pairs) |
| Prism | `Choice` profunctor | Works with profunctors that can handle sum types (Either/Result) |
| Traversal | `Traversing` profunctor | Works with profunctors over collections |

**Why does this unify composition?** Because function composition is associative:

```
(optic1 ∘ optic2)(P) = optic1(optic2(P))
```

If `optic1 : P<A,B> → P<S,T>` and `optic2 : P<B,C> → P<A,B>`, their composition `optic1 ∘ optic2 : P<B,C> → P<S,T>` is just `|p| optic1(optic2(p))`. No special combinator needed.

**Concrete intuition:** In the practical (van Laarhoven) encoding, a Lens is a higher-order function:

```
lens : (A → B) → (S → T)
```

It says: "give me a function that transforms the focused field `A → B`, and I'll give you a function that transforms the whole structure `S → T`."

Similarly, a Prism:

```
prism : (A → B) → (S → T)
```

Same shape — but it handles the "no match" case internally.

**Composition is just `|f| lens1(lens2(f))`** — no special `compose_lens_lens` function needed.

## How It Works in Rust

```rust
// Rust can't fully express "forall P. P<A,B> -> P<S,T>" due to HKT limits.
// We use the concrete / van Laarhoven encoding instead:
// a Lens is a function that takes a "modifier" fn(A)->B and returns fn(S)->T

// Step 1: Lens as a higher-order function
// Given: how to transform the focused field (A → B)
// Produce: how to transform the whole structure (S → T)
fn lens_via_fn<S, T, A, B>(
    get: impl Fn(&S) -> A + 'static,        // how to extract the focus
    set: impl Fn(B, &S) -> T + 'static,     // how to put it back
) -> impl Fn(impl Fn(A) -> B + 'static) -> impl Fn(S) -> T {
    move |f| {                               // f: A -> B  (the "modifier")
        move |s| set(f(get(&s)), &s)         // apply f to focus, rebuild S
    }
}

#[derive(Debug, Clone)]
struct Point { x: f64, y: f64 }

// A Lens targeting Point.x
let x_lens = lens_via_fn::<Point, Point, f64, f64>(
    |p| p.x,
    |x, p| Point { x, y: p.y },
);

let p = Point { x: 1.0, y: 2.0 };

// Use the lens: pass a modifier, get back a function over the whole struct
let double_x = x_lens(|x| x * 2.0);  // fn(Point) -> Point
let p2 = double_x(p);                  // applies to the whole Point
println!("{:?}", p2);  // Point { x: 2.0, y: 2.0 }

// Step 2: Prism as a higher-order function — same shape, different internals
fn prism_via_fn<S, T, A, B>(
    preview: impl Fn(&S) -> Option<A> + Clone,  // extract if present
    review:  impl Fn(B) -> T + Clone,            // inject into S
    inject:  impl Fn(S) -> T + Clone,            // pass-through when no match
) -> impl Fn(impl Fn(A) -> B) -> impl Fn(S) -> T {
    move |f| {
        let preview = preview.clone();
        let review  = review.clone();
        let inject  = inject.clone();
        move |s| match preview(&s) {
            Some(a) => review(f(a)),    // match: apply f, wrap result
            None    => inject(s),       // no match: pass through unchanged
        }
    }
}

// A Prism targeting Some(i32) inside Option<i32>
let some_prism = prism_via_fn::<Option<i32>, Option<i32>, i32, i32>(
    |o| *o,          // preview: Option<i32> -> Option<i32> (unwrap one layer)
    |b| Some(b),     // review: i32 -> Option<i32>
    |_| None,        // no match: produce None
);

some_prism(|x| x * 2)(Some(5));  // Some(10)
some_prism(|x| x * 2)(None);     // None  ← pass-through

// Step 3: Composition is just function composition
// No special lens-compose-lens combinator needed.
// lens1(lens2(f)) = apply lens2 first, then lens1.
// (In a full library with named composition, this is just (lens1 . lens2)(f))

// Example: compose two lenses
// outer_lens ∘ inner_lens (focused on inner inside outer)
fn compose<A, B, C, S, T, U>(
    outer: impl Fn(Box<dyn Fn(B) -> C>) -> Box<dyn Fn(S) -> T>,
    inner: impl Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(U) -> S>,
) -> impl Fn(Box<dyn Fn(A) -> C>) -> Box<dyn Fn(U) -> T> {
    move |f| outer(Box::new(move |b| {
        // This is just outer(inner(f)):
        // f: A -> C, inner turns it into B -> C... not quite right here.
        // Full version requires GATs — see the rs file for the concrete approximation.
        todo!()
    }))
}

// The point: in a language with full HKT, composition is (.)
// In Rust, each lens/prism works as a function, and nesting them works naturally:
let double_x_of_p = x_lens(|x| x * 10.0)(Point { x: 3.0, y: 4.0 });
// Output: Point { x: 30.0, y: 4.0 }
```

## What This Unlocks

- **Single composition operator** — all optics (Lens, Prism, Traversal, Iso) compose the same way: function composition. No `compose_lens_prism` combinators; just nest the calls.
- **Extensibility** — a new optic type is just a new constraint on which profunctors it accepts. Adding it to the system doesn't break existing optics.
- **Understanding optic libraries** — libraries like Haskell's `lens`, or Rust's `lens` crates, use variants of this encoding. Understanding profunctor optics makes reading their source and documentation tractable.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Profunctor optic type | `type ('s,'t,'a,'b) optic = { unOptic: 'p. ('a,'b) 'p -> ('s,'t) 'p }` | Not directly expressible — GATs don't support `forall P` universally |
| Composition | `(.)` — plain function composition | Manual nesting: `outer_fn(inner_fn(f))` |
| Lens encoding | van Laarhoven: `forall f. Functor f => (a -> f b) -> s -> f t` | `impl Fn(impl Fn(A)->B) -> impl Fn(S)->T` (monomorphised, not universal) |
| Prism encoding | `forall p. Choice p => p a b -> p s t` | Concrete function with `preview`/`review`/`inject` captures |
| Full HKT | First-class: `'p` ranges over all profunctors | Not supported; must specialise per concrete profunctor (`Mapper`, `Forget`, `Star`) |
