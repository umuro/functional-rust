# 145: GAT Collections — Generic Associated Types

**Difficulty:** 5  **Level:** Advanced

Use Generic Associated Types (GATs) to define container traits whose iterators borrow from `self` — enabling lending iterators and length-indexed collections.

## The Problem This Solves

Before GATs (stabilized in Rust 1.65), you couldn't write a `Container` trait whose `iter()` method returned an iterator that borrowed from `self`. The associated type couldn't carry a lifetime parameter. This forced trait designers into awkward workarounds or runtime allocation.

GATs fix this: `type Iter<'a>: Iterator where Self: 'a` lets the associated type vary over a lifetime. This enables *lending iterators* — iterators that hold a reference back into the container, which is how real iterators actually work.

The second use case: length-indexed collections, where the length is encoded in the type using const generics. This catches out-of-bounds access at compile time.

## The Intuition

A GAT is an associated type that takes type parameters — most commonly a lifetime. Without GATs, `type Iter = ...` is fixed. With GATs, `type Iter<'a> = ...` can express "an iterator that borrows from self for lifetime 'a."

Length-indexed vectors use Rust's const generics: `LenVec<T, const N: usize>` carries the length `N` as part of the type. A `LenVec<i32, 3>` and `LenVec<i32, 4>` are different types — you can't pass one where the other is expected.

Phantom Peano numbers (`Zero`, `Succ<N>`) encode length at the type level without const generics — the OCaml-style approach. `pvec_head` is only callable on `PVec<T, Succ<N>>` — the compiler prevents calling it on an empty vector.

## How It Works in Rust

```rust
/// A GAT trait: the iterator type borrows from self for lifetime 'a.
pub trait Container {
    type Item;
    type Iter<'a>: Iterator<Item = &'a Self::Item> where Self: 'a;
    //          ^^^  — GAT: associated type with lifetime parameter

    fn iter(&self) -> Self::Iter<'_>;
    fn len(&self) -> usize;
}

/// Stack implements Container — iter yields top-first.
pub struct Stack<T>(Vec<T>);

impl<T> Container for Stack<T> {
    type Item = T;
    type Iter<'a> = std::iter::Rev<std::slice::Iter<'a, T>> where T: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().rev()   // borrows from self — no allocation
    }
    fn len(&self) -> usize { self.0.len() }
}
```

Length-indexed vector with const generics:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct LenVec<T, const N: usize>(pub [T; N]);
//                   ^^^^^^^^^^^^^^ — length is part of the type

// zip_with is only defined for two LenVec with the SAME N
impl<T: Copy + Default, const N: usize> LenVec<T, N> {
    pub fn zip_with<U, V, F>(&self, other: &LenVec<U, N>, f: F) -> LenVec<V, N>
    where U: Copy + Default, V: Copy + Default, F: Fn(T, U) -> V
    { /* ... */ }
}
```

Peano-typed safe head (OCaml style):

```rust
pub fn pvec_head<T: Clone, N>(v: &PVec<T, Succ<N>>) -> T {
    v.data[0].clone()
    // Only callable when length is Succ<N> — never empty
}
```

Calling `pvec_head` on an empty `PVec<T, Zero>` is a compile error.

## What This Unlocks

- **Lending iterators** — iterators that yield references into the container, without copying
- **Length-safe APIs** — functions that only accept fixed-size arrays of the right length
- **Type-safe matrix operations** — `Matrix<f64, 3, 4>` × `Matrix<f64, 4, 2>` = `Matrix<f64, 3, 2>` verified at compile time

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Length-indexed type | GADT: `type _ t = Nil : zero t | Cons : 'a * 'n t -> 'n succ t` | `LenVec<T, const N: usize>` (const generics) |
| Peano numbers | Type-level via GADT constructors | Phantom types: `Zero`, `Succ<N>` |
| GAT equivalent | Functors with parameterized output types | `type Item<'a>` in trait definition |
| Safe head | GADT ensures non-empty at type level | `pvec_head<T, N>(v: &PVec<T, Succ<N>>)` |
| Lending iterator | No equivalent (GC handles it) | Core motivation for GATs |
