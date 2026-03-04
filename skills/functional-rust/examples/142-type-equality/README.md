# 142: Type Equality Witnesses

**Difficulty:** 5  **Level:** Expert

Encode a proof that two types are equal as a Rust value — and use it to safely coerce at compile time.

## The Problem This Solves

In type-theoretic programming, you sometimes need to convince the compiler that two type variables are actually the same type — not just compatible, but identical. This arises in generic data structures, type-indexed containers, and translations from languages with GADTs (Generalized Algebraic Data Types).

OCaml with GADTs can express this directly: a constructor `Refl : ('a, 'a) eq` is only inhabitable when both type parameters are the same. Rust doesn't have GADTs. But we can simulate the pattern with a `TypeEq<A, B>` struct that can only be constructed when `A = B`. Any function accepting a `TypeEq<A, B>` has a compile-time proof that the types are identical — and can use it to justify coercions that would otherwise be unsound.

This is niche, advanced type-level programming, but it's exactly the pattern used in type-safe heterogeneous maps, typed abstract syntax trees, and dependently-typed encodings in Rust.

## The Intuition

`TypeEq<A, B>` is a "proof certificate." You can only mint one via `TypeEq::refl()`, which is only callable when `A` and `B` are the same type. Once you have the certificate, you can pass it around and use it as evidence in generic functions that need to know two types are identical.

The key properties of type equality hold: reflexivity (`A = A`), symmetry (`A = B → B = A`), and transitivity (`A = B, B = C → A = C`). All three are implementable as methods on `TypeEq`, and all three are vacuously sound — they don't change the concrete types, only reflect logical relationships.

## How It Works in Rust

```rust
use std::marker::PhantomData;

pub struct TypeEq<A, B>(PhantomData<(A, B)>);

impl<T> TypeEq<T, T> {
    // Only constructible when A = B (same type parameter)
    pub fn refl() -> TypeEq<T, T> {
        TypeEq(PhantomData)
    }
}

impl<A, B> TypeEq<A, B> {
    pub fn sym(self) -> TypeEq<B, A> { TypeEq(PhantomData) }

    pub fn trans<C>(self, _: TypeEq<B, C>) -> TypeEq<A, C> {
        TypeEq(PhantomData)
    }
}

// Use the proof in generic code
fn string_length<S: Into<String>>(val: S, _proof: TypeEq<S, String>) -> usize {
    val.into().len()
}

// Only callable with TypeEq<String, String> — proving S *is* String
let proof: TypeEq<String, String> = TypeEq::refl();
println!("{}", string_length("hello".to_string(), proof));
```

The `PhantomData` carries the type information without any runtime representation. The struct is zero-sized — type equality witnesses have no runtime cost.

## What This Unlocks

- **Type-safe heterogeneous containers** — a `TypeEq<A, B>` witness enables safe coercions inside generic code that would otherwise require `unsafe`
- **GADT simulation** — data structures that carry type-level constraints (like typed ASTs where the phantom type tracks the result type of each expression)
- **Leibniz equality** — the function-based encoding `Leibniz<A, B> { coerce: Box<dyn Fn(A) -> B> }` is the classical alternative, closer to the original Leibniz definition

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type equality proof | `type ('a, 'b) eq = Refl : ('a, 'a) eq` (GADT) | `struct TypeEq<A,B>(PhantomData<(A,B)>)` |
| Construction | `Refl` constructor (GADT constraint) | `TypeEq::refl()` (only for `<T,T>`) |
| Coercion via proof | `let Refl = proof in use_as_same_type` | Method or `Into` bound |
| Runtime cost | Zero (erased) | Zero (`PhantomData` is zero-sized) |
| Symmetry | `let sym : ('b,'a) eq = Refl` | `.sym()` method |
