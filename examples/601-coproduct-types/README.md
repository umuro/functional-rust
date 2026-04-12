📖 **[View on hightechmind.io →](https://hightechmind.io/rust/601-coproduct-types)**

---

# Coproduct Types (Sum Types)
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Coproduct types (also called sum types, tagged unions, or discriminated unions) represent "one of several possibilities." They are the mathematical dual of product types. In category theory, a coproduct `A + B` is the "either A or B" type — represented as Rust's `enum` with two variants. `Either<A, B>` is the canonical general-purpose coproduct, equivalent to Haskell's `Either`, OCaml's `('a, 'b) result` or `Either` from `Either` libraries. Sum types are the foundation of error handling (`Result`), optional values (`Option`), and any branching data.

## Learning Outcomes

- How `Either<A, B>` generalizes `Result<T, E>` and `Option<T>` to arbitrary coproducts
- How `bimap`, `map_left`, `map_right` transform either side of a sum type
- How `fold` collapses a coproduct by providing functions for each case
- How Rust's enums are sum types and what category theory says about them
- Where coproducts appear: error handling, branching data, type-safe union types

## Rust Application

`Either<A, B>` has `Left(A)` and `Right(B)` variants. `bimap(f, g)` applies `f` to `Left` and `g` to `Right`. `map_left(f)` and `map_right(g)` transform one side. `fold<C>(on_left, on_right)` eliminates the coproduct by providing a handler for each variant. `either(l, r)` is a smart constructor. The source also shows `n-ary` coproducts using enums with more variants.

Key patterns:
- `bimap(f: A -> C, g: B -> D) -> Either<C, D>` — functor over both sides
- `fold(on_left: A -> C, on_right: B -> C) -> C` — eliminator
- `map_right(f)` — `fmap` from the Right functor (biased `Either`)

## OCaml Approach

OCaml uses `result` as the built-in asymmetric coproduct:

```ocaml
type ('a, 'b) either = Left of 'a | Right of 'b
let bimap f g = function Left a -> Left (f a) | Right b -> Right (g b)
let fold fl fr = function Left a -> fl a | Right b -> fr b
```

## Key Differences

1. **Built-in vs library**: Rust's `Result<T, E>` is the standard coproduct; `Either<A, B>` is a library type; OCaml has `('a, 'b) result` and `('a, 'b) Either.t` separately.
2. **Biased vs symmetric**: `Result<T, E>` is right-biased (`map` transforms `Ok`); `Either<A, B>` is symmetric; OCaml's `result` is also right-biased.
3. **Functor instance**: Rust manually implements `map_left`/`map_right` methods; Haskell/OCaml have typeclass/functor instances enabling generic `fmap`.
4. **GAT limit**: A true polymorphic `Functor` trait for `Either` requires GATs (Generic Associated Types) in Rust — complex but supported since Rust 1.65.

## Exercises

1. **Either chain**: Implement `fn partition_either<A, B>(items: Vec<Either<A, B>>) -> (Vec<A>, Vec<B>)` that splits an `Either` collection into two separate lists.
2. **Error accumulation**: Build `Either<Vec<Error>, Success>` where you accumulate errors on the `Left` side — implement `combine(a: Either<Vec<E>, A>, b: Either<Vec<E>, B>) -> Either<Vec<E>, (A, B)>`.
3. **From Result**: Implement `From<Result<A, B>> for Either<B, A>` (note the flipped convention) and `From<Either<B, A>> for Result<A, B>`.
