📖 **[View on hightechmind.io →](https://hightechmind.io/rust/228-natural-transformation)**

---

# Example 228: Natural Transformations

**Difficulty:** ⭐⭐⭐
**Category:** Category Theory
**OCaml Source:** Bartosz Milewski — *Category Theory for Programmers*, Chapter 10

## Problem Statement

Implement and verify natural transformations between functors — structure-preserving
maps that commute with `fmap`. Demonstrate the naturality condition, horizontal
composition, and the relationship between List and Option functors.

## Learning Outcomes

- What a natural transformation is in programming terms: a polymorphic function `F<A> → G<A>`
- How to verify the naturality square: `nat(fmap f xs) == fmap f (nat xs)`
- How natural transformations compose to form the functor category
- How Rust's generic functions encode parametric naturality by construction

## OCaml Approach

OCaml expresses natural transformations as polymorphic functions. The naturality
condition is verified with `List.map` and `Option.map`. Higher-order functions accept
both the morphism `f` and the nat transformation `nat`, checking both sides of the
commutative square. The structural equality `=` makes the check concise.

## Rust Approach

Rust uses generic functions bounded by `Clone` and `PartialEq` to implement nat
transformations. Because Rust lacks polymorphic function values (no rank-2 types),
`verify_naturality` takes two monomorphized copies of the nat transformation — one
for each type — which the compiler instantiates automatically from the same generic
function. Composition is a simple function call chain.

## Key Differences

1. **Polymorphism:** OCaml's `'a list -> 'a option` is intrinsically polymorphic; Rust
   monomorphizes each use, so `verify_naturality` needs `nat_t` and `nat_u` separately.
2. **Ownership:** Rust returns owned `T` (via `.cloned()`) rather than references, keeping
   the API simple at the cost of requiring `T: Clone`.
3. **Naturality by construction:** Rust's parametric generics guarantee naturality for free
   — any `fn<T>(Vec<T>) -> Option<T>` that doesn't inspect `T` is automatically natural.
4. **Composition:** OCaml uses function application; Rust is identical — `option_to_vec(safe_head(list))` chains two nat transformations directly.

## Exercises

1. Implement a natural transformation from `Result<T, E>` to `Option<T>` that discards the error, and verify the naturality square holds for a sample function `f: T -> U`.
2. Write a natural transformation from `Vec<T>` to `Option<T>` that returns the first element (head), and implement its inverse as a partial natural transformation.
3. Define a `Monad` trait (with `unit` and `bind`) for `Option`, implement it, then use natural transformations to lift a computation from `Vec` context into `Option` context.
