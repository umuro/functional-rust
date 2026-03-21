📖 **[View on hightechmind.io →](https://hightechmind.io/rust/074-bifunctor)**

---

# 074 — Bifunctor
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A bifunctor is a type with two type parameters where both can be independently mapped. `Result<T, E>` is the canonical bifunctor: `map` transforms the `T` (success side), `map_err` transforms the `E` (error side), and `bimap` does both. Other bifunctors: tuples `(A, B)`, `Either<L, R>`.

Bifunctors formalize the observation that both sides of a sum type or product type can be transformed independently. This is the generalization of `Result::map` + `Result::map_err` into a single concept. Used in category theory, functional programming libraries (Haskell's `Data.Bifunctor`, Scala's `cats`), and error type transformation pipelines.

## Learning Outcomes

- Understand a bifunctor as a type with two mappable channels
- Implement `bimap(f, g)` that transforms both sides of a `Result`
- Implement `first(f)` and `second(g)` as single-side maps
- Recognize `(A, B)` as a bifunctor: `bimap(f, g)` on a pair
- See how `bimap` generalizes `map` and `map_err` into a unified interface

## Rust Application

For `Result<T, E>`: `bimap(r, f, g) = match r { Ok(x) => Ok(f(x)), Err(e) => Err(g(e)) }`. `first(r, f) = bimap(r, f, id)` — maps only the `Ok` side. `second(r, g) = bimap(r, id, g)` — maps only the `Err` side. For pairs: `bimap_pair((a, b), f, g) = (f(a), g(b))` — transforms both elements independently. The `Bifunctor` trait in Rust requires explicit implementation; there is no stdlib trait for it.

## OCaml Approach

OCaml's Result is a bifunctor: `let bimap f g = function Ok x -> Ok (f x) | Error e -> Error (g e)`. `Result.map f r` is `bimap f Fun.id r`. `Result.map_error g r` is `bimap Fun.id g r`. For pairs: `let bimap_pair f g (a, b) = (f a, g b)`. The `Bifunctor` typeclass from Haskell is not part of OCaml stdlib but is trivially implemented per-type.

## Key Differences

1. **No stdlib trait**: Neither Rust nor OCaml has a standard `Bifunctor` trait/typeclass. Implement the functions per type. Haskell has `Data.Bifunctor`.
2. **`map` as `first`**: Rust's `Result::map` is exactly `bifunctor::first` — it maps the left (`Ok`) side. `map_err` is `bifunctor::second`. Recognizing this shows `Result`'s functor structure.
3. **Pair bifunctor**: `(A, B)` maps to `(f(A), g(B))`. Rust: `let (new_a, new_b) = (f(a), g(b))`. This is the product type bifunctor.
4. **Laws**: Bifunctor laws: `bimap id id = id`, `bimap (f . g) (h . k) = bimap f h . bimap g k`. These ensure `bimap` preserves function composition.

## Exercises

1. **Either type**: Define `enum Either<L, R> { Left(L), Right(R) }`. Implement `bimap`, `map_left`, and `map_right`. Implement `From<Result<R, L>> for Either<L, R>`.
2. **Profunctor**: A profunctor is "contravariant in the first argument, covariant in the second". Implement `dimap(f: B -> A, g: C -> D) -> impl Fn(A -> C) -> impl Fn(B -> D)` for functions. This is the foundation of optics (lenses).
3. **BiTraverse**: Define `bi_traverse(v: Result<T, E>, f: impl Fn(T) -> Option<T2>, g: impl Fn(E) -> Option<E2>) -> Option<Result<T2, E2>>` that transforms both sides optionally.
