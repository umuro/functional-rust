📖 **[View on hightechmind.io →](https://hightechmind.io/rust/602-product-types-advanced)**

---

# 602: Product Types (Advanced)

**Difficulty:** 5  **Level:** Master

Structs are categorical products: projections are field accessors, pairing morphisms are struct constructors, and lenses are the natural morphisms between products.

## The Problem This Solves

You write structs every day, but treating them as categorical products unlocks a precise vocabulary for reasoning about data transformations. When is it safe to refactor a struct? When is two representations equivalent? When can you derive one accessor from others? Category theory answers these questions algebraically.

The product type `A × B` is inhabited by pairs `(a, b)` where both an `A` and a `B` are present. Its projections `π₁: A×B → A` and `π₂: A×B → B` are field accessors. The *universal property* says: for any type `C` with functions `f: C→A` and `g: C→B`, there is a unique function `⟨f, g⟩: C → A×B`. This is exactly struct construction: `T { a: f(x), b: g(x) }`.

Lenses emerge naturally from products: a lens for field `a` in `T { a: A, b: B }` is the pair of `get: T→A` (projection) and `set: A→T→T` (pairing with the other fields unchanged). Combining products and coproducts gives you the full algebraic type system.

## The Intuition

A struct is a categorical product: `get` is a projection morphism, `set` is a pairing morphism that reconstructs the product, and Rust's struct update syntax `T { field: new, ..old }` is the categorical pairing — it applies the new projection for one field and the identity projection for all others. The trade-off: products give total access (all fields always present) but require updating all fields in transformations; coproducts give partial access but handle alternatives.

## How It Works in Rust

```rust
// Product type A × B — both A and B are always present
struct Pair<A, B> {
    first: A,   // projection π₁
    second: B,  // projection π₂
}

impl<A, B> Pair<A, B> {
    // Universal property: unique morphism from C given f: C→A and g: C→B
    fn pair<C>(c: C, f: impl Fn(&C) -> A, g: impl Fn(&C) -> B) -> Self {
        Pair { first: f(&c), second: g(&c) }
    }

    // Lens for `first`: get + set
    fn get_first(&self) -> &A { &self.first }

    fn set_first(self, new_first: A) -> Self {
        Pair { first: new_first, ..self }  // struct update = categorical pairing
    }

    // Bifunctor: map both components independently
    fn bimap<C, D>(self, f: impl FnOnce(A) -> C, g: impl FnOnce(B) -> D) -> Pair<C, D> {
        Pair { first: f(self.first), second: g(self.second) }
    }
}

// Algebraic identity: A × () ≅ A
fn from_unit_pair<A>(p: Pair<A, ()>) -> A { p.first }
fn to_unit_pair<A>(a: A) -> Pair<A, ()> { Pair { first: a, second: () } }

// Record update syntax IS the pairing morphism
#[derive(Clone)]
struct Config { host: String, port: u16, tls: bool }

let base = Config { host: "localhost".into(), port: 8080, tls: false };
let prod = Config { port: 443, tls: true, ..base };  // pairing: new π_port, π_tls; identity π_host
```

## What This Unlocks

- **Lens derivation**: any field in a struct has a canonical lens — `get` = projection, `set` = pairing. This is how optics libraries like `lens-rs` are derived.
- **Type algebra**: refactor `(A, B, C)` to `((A, B), C)` — the isomorphism is `((a, b), c) ↔ (a, b, c)`, useful for generic code that works on nested products.
- **Generic programming**: functions that work on "any product with a `name` field" are expressed as trait bounds — the typeclass approach to structural polymorphism.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Product A×B | `type t = { a: A; b: B }` | `struct T { a: A, b: B }` |
| Projection π₁ | `.a` field access | `.a` field access |
| Projection π₂ | `.b` field access | `.b` field access |
| Pairing morphism | `{ a = f x; b = g x }` | `T { a: f(x), b: g(x) }` |
| Record update | `{ r with a = v }` | `T { a: v, ..r }` |
| Lens | `{get; set}` record | `(fn get, fn set)` or lens struct |
| Unit product | `unit` / `()` | `()` |
