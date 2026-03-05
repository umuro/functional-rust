📖 **[View on hightechmind.io →](https://hightechmind.io/rust/226-category-basics)**

---

# 226: Category Theory Basics

**Difficulty:** Expert  **Level:** 4

A category is just: objects + arrows + associative composition. That's the complete foundation for reasoning about programs algebraically.

## The Problem This Solves

Programming is full of patterns that look superficially different but are structurally identical. Function composition. Monad chaining. Parser combination. Option/Result chaining. All of these share the same shape: you connect things together, the connection is associative, and there's a neutral "do nothing" element. But without a common vocabulary, we describe each one separately and miss the deeper unity.

Category theory provides that vocabulary. Once you see that Rust's type system, function composition, and monad operations are all instances of the same categorical structure, you can reason about them uniformly. You know what laws they must satisfy, what transformations are safe, and why `map` over `Option`, `Vec`, and `Result` all feel the same.

This example introduces the core machinery: categories as `compose` + `identity`, and the Kleisli category as the categorical model of monadic computation.

## The Intuition

A **category** has three things:
1. **Objects** — in Rust, these are *types* (`i32`, `String`, `Vec<u8>`, your structs)
2. **Arrows (morphisms)** — in Rust, these are *functions* (`fn(A) -> B`)
3. **Composition** — chaining arrows: `f: A→B` and `g: B→C` give `g∘f: A→C`

Two laws must hold:
- **Associativity:** `(h∘g)∘f = h∘(g∘f)` — grouping doesn't matter when composing
- **Identity:** there's an arrow `id: A→A` where `f∘id = f` and `id∘f = f` — "do nothing" composes neutrally

In Rust: `identity<A>(a: A) -> A { a }` is the identity arrow. `compose(f, g)` chains them.

The **Kleisli category** is what you get when all arrows have the shape `A -> M<B>` (for some wrapper `M`). In Rust: `A -> Option<B>`, `A -> Result<B, E>`, `A -> Vec<B>`. Kleisli composition is exactly monadic `and_then`. This is why every monad is secretly a category.

Think of it like plumbing: objects are pipe sizes, morphisms are pipe segments, composition is connecting segments end-to-end, and identity is a straight-through segment.

## How It Works in Rust

```rust
// Identity arrow: every type A has one
fn identity<A>(a: A) -> A { a }

// Composition: f after g
fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |a| f(g(a))
}

// Category as a trait (explicit abstraction)
trait Category {
    type Obj;
    fn id() -> Box<dyn Fn(Self::Obj) -> Self::Obj>;
    fn compose(
        f: Box<dyn Fn(Self::Obj) -> Self::Obj>,
        g: Box<dyn Fn(Self::Obj) -> Self::Obj>,
    ) -> Box<dyn Fn(Self::Obj) -> Self::Obj>;
}

// Kleisli category: arrows are A -> Option<B>
// Composition is exactly monadic and_then
fn kleisli_compose<A, B, C>(
    f: impl Fn(B) -> Option<C>,
    g: impl Fn(A) -> Option<B>,
) -> impl Fn(A) -> Option<C> {
    move |a| g(a).and_then(|b| f(b))  // this IS categorical composition in Kleisli
}

fn kleisli_id<A>(a: A) -> Option<A> { Some(a) }  // identity in Kleisli = Some
```

Verify the laws hold:
```rust
// Associativity: compose(h, compose(g, f)) == compose(compose(h, g), f)
// Identity: compose(f, identity) == f  and  compose(identity, f) == f
// These are checked in the tests — violating them means it's not a category
```

## What This Unlocks

- **Principled composition** — any type that forms a category gets `compose` and `id` for free, with laws that guarantee safe refactoring.
- **Kleisli as monad model** — understanding that `and_then`/`flat_map` is categorical composition explains *why* monad laws look the way they do.
- **Abstract reasoning** — once you identify a categorical structure in your code, you immediately know what holds (associativity, identity) without proof.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Category abstraction | Module signature `CATEGORY` | `trait Category` |
| Composition | `let compose f g x = f (g x)` | `fn compose<A,B,C>` with move closure |
| Kleisli arrows | `a -> b option` | `impl Fn(A) -> Option<B>` |
| Kleisli composition | `Option.bind` / `>>=` | `.and_then()` |
| Objects | Types in a module | Types as `type Obj` in trait |
