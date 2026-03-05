📖 **[View on hightechmind.io →](https://hightechmind.io/rust/619-iso-pattern)**

---

# 619: Isomorphism (Iso)

**Difficulty:** 5  **Level:** Master

A bidirectional, lossless conversion between two types — `to: A → B` and `from: B → A` — proving the types are equivalent representations.

## The Problem This Solves

You often have data in two equivalent forms: a newtype wrapper and its inner type, a domain model and a DTO, a compressed and decompressed representation. Converting between them is straightforward, but the relationship is informal — you have `into()` and `from()` implementations, but nothing in the type system says these conversions are inverses of each other, lossless, or composable.

An isomorphism makes the bidirectionality explicit and composable. `Iso<A, B>` holds `to: A→B` and `from: B→A` together as a unit, with the semantic guarantee that `from(to(a)) == a` and `to(from(b)) == b`. This isn't just documentation — it enables a small algebra: isos compose (chain two conversions), invert (flip direction), and lift through containers (if `A ≅ B` then `Vec<A> ≅ Vec<B>`).

In optics hierarchies, `Iso` sits at the top: it's the most powerful optic because it implies zero information loss. Every `Iso` is also a `Lens`, a `Prism`, and a `Traversal` — you can use it anywhere a weaker optic is expected.

## The Intuition

An isomorphism is a pair of inverse functions that prove two types are "the same thing with different clothes on" — `to` and `from` round-trip perfectly, so you can work in whichever representation is convenient and convert back losslessly. The trade-off: isos are only applicable when the conversion truly is lossless; use `Prism` (partial) or `Lens` (structural projection) when there might be information loss.

## How It Works in Rust

```rust
// An Iso: two types with a perfect bidirectional conversion
struct Iso<A, B> {
    to:   Box<dyn Fn(A) -> B>,
    from: Box<dyn Fn(B) -> A>,
}

impl<A: 'static, B: 'static> Iso<A, B> {
    fn new(to: impl Fn(A) -> B + 'static, from: impl Fn(B) -> A + 'static) -> Self {
        Iso { to: Box::new(to), from: Box::new(from) }
    }

    // Invert: flip the direction — B → A becomes the "forward" direction
    fn inverse(self) -> Iso<B, A> {
        Iso { to: self.from, from: self.to }
    }

    // Compose: if A ≅ B and B ≅ C, then A ≅ C
    fn compose<C: 'static>(self, other: Iso<B, C>) -> Iso<A, C> {
        let to1 = self.to;
        let from2 = self.from;
        let to2 = other.to;
        let from1 = other.from;
        Iso {
            to:   Box::new(move |a| (to2)((to1)(a))),
            from: Box::new(move |c| (from2)((from1)(c))),
        }
    }
}

// Example: Celsius ↔ Fahrenheit
let celsius_fahrenheit: Iso<f64, f64> = Iso::new(
    |c| c * 9.0 / 5.0 + 32.0,   // to: Celsius → Fahrenheit
    |f| (f - 32.0) * 5.0 / 9.0, // from: Fahrenheit → Celsius
);

// Laws (testable):
// from(to(100.0)) ≈ 100.0   (round-trip)
// to(from(212.0)) ≈ 212.0   (round-trip)

// Newtype iso: every newtype wrapping T is isomorphic to T
struct Meters(f64);
let meters_iso: Iso<f64, Meters> = Iso::new(Meters, |m| m.0);
```

## What This Unlocks

- **Representation changes**: work with a convenient internal type, convert to wire format for serialization, convert back — `Iso` makes this explicit and composable.
- **Optics lifting**: `Iso<A, B>` lifts to `Iso<Vec<A>, Vec<B>>` — change the representation of a container by mapping the iso over it.
- **Refactoring safety**: when two representations are proven isomorphic, you can freely swap them — the compiler plus the iso laws guarantee no information loss.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iso definition | `{ to_: 'a -> 'b; from_: 'b -> 'a }` | `struct Iso<A, B> { to: Box<dyn Fn(A)->B>, from: Box<dyn Fn(B)->A> }` |
| Inversion | Swap `to_` and `from_` | `.inverse()` method |
| Composition | Compose the two function pairs | `.compose(other_iso)` |
| Roundtrip law | `from(to a) = a` | Testable with `assert_eq!` |
| In optics hierarchy | Most powerful — implies Lens, Prism | Same — Iso ⊆ Lens ⊆ Traversal |
| Newtype case | `let id = { to_ = fun x -> Wrapped x; from_ = fun (Wrapped x) -> x }` | `Iso::new(MyType, \|m\| m.0)` |
