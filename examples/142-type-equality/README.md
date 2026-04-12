📖 **[View on hightechmind.io →](https://hightechmind.io/rust/142-type-equality)**

---

# Type Equality
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

In advanced generic programming, you sometimes need to express and prove that two type parameters are the same type. This arises in type-safe casts, in GADT-style encodings, and in generic code that needs to branch on whether two type parameters are equal. A type equality witness `Eq<A, B>` is a value that proves `A = B` at the type level, enabling safe casts without `unsafe` code and without dynamic type checking.

## Learning Outcomes

- Understand what a type equality proof is and why it is useful
- Learn how to encode type equality witnesses in Rust using marker traits
- See how a `TypeEq<A, B>` value enables safe coercion between `A` and `B`
- Understand the connection to Leibniz equality and substitution in type theory

## Rust Application

The simplest form in Rust uses a marker trait `TypeEq<A, B>` implemented only for `TypeEq<T, T>`. A witness value `Refl` for `TypeEq<T, T>` can be used to coerce `A` to `B` safely. More practically, `std::any::TypeId` provides runtime type equality checks, while the `downcast_ref` pattern uses type equality to enable safe extraction from `Box<dyn Any>`. Compile-time type equality proofs are used in frunk and type-level programming crates.

## OCaml Approach

OCaml encodes type equality via GADTs:
```ocaml
type (_, _) eq = Refl : ('a, 'a) eq
let cast : type a b. (a, b) eq -> a -> b = fun Refl x -> x
```
`Refl` is only constructible for equal types, and pattern matching on `Refl` refines the type checker to know `a = b` in the match branch, enabling safe coercions. This is the standard technique in OCaml generic programming.

## Key Differences

1. **GADT convenience**: OCaml's `Refl : ('a, 'a) eq` directly encodes the equality; Rust must use workarounds since native GADTs are not available.
2. **Coercion**: OCaml's `cast` using `Refl` requires no `unsafe`; Rust's type equality coercions sometimes require `unsafe transmute` unless carefully structured.
3. **Runtime vs. compile time**: Rust's `TypeId`-based equality is a runtime check; OCaml's GADT `eq` is purely compile-time.
4. **Standard library**: OCaml's `eq` type is definable in user code and used in `Base`/`Core`; Rust lacks a standard type equality witness, though crates provide them.

## Exercises

1. Implement a `same_type<A: 'static, B: 'static>() -> bool` using `TypeId` for runtime type equality checking.
2. Design a `TypeEq<A, B>` marker trait with an `unsafe fn coerce(a: A) -> B` method, implemented only for `TypeEq<T, T>`.
3. Write a generic function that branches on type equality: `fn stringify_if_str<T: 'static>(val: T) -> Option<String>`.
