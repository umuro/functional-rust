# Comparison: Phantom Types — OCaml vs Rust

## Core Insight

Phantom types demonstrate zero-cost type safety in both languages. OCaml uses abstract types (declared but never defined) as phantom parameters. Rust uses `PhantomData<T>` — a zero-sized type from `std::marker`. In both cases, the compiler enforces that you can't add meters to seconds, but the runtime representation is just a float.

## OCaml

```ocaml
type meters
type seconds
type 'a quantity = Q of float

let meters x : meters quantity = Q x
let add (Q a : 'a quantity) (Q b : 'a quantity) : 'a quantity = Q (a +. b)
```

## Rust

```rust
use std::marker::PhantomData;

pub struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Quantity::new(self.value + rhs.value) }
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Phantom type | `type meters` (abstract) | `struct Meters;` (zero-sized) |
| Marker | Implicit in type param | `PhantomData<Unit>` |
| Runtime cost | Zero | Zero (`PhantomData` is ZST) |
| Operator overload | Functions only | `impl Add for Quantity<U>` |
| Alternative | Module signatures | Newtype wrappers |
| Size check | N/A | `size_of::<Quantity<M>>() == size_of::<f64>()` |

## Learner Notes

- **`PhantomData`**: Rust requires explicit marking because it tracks all type parameters for drop checking and variance
- **Zero-sized types (ZSTs)**: `struct Meters;` takes 0 bytes — the compiler optimizes it away entirely
- **Trait-based ops**: Rust's `impl Add` gives `+` syntax; OCaml just uses named functions
- **Newtype alternative**: `struct Meters(f64)` is simpler but requires implementing ops for each unit separately
- **OCaml's elegance**: Abstract types + type annotations is more concise than Rust's PhantomData approach
