# Monoid Pattern — Trait-Based Combining

## Problem Statement
Define a `Monoid` trait with an identity element and an associative `combine` operation, then implement `concat_all` to fold any list of monoids into a single value using the trait.

## Learning Outcomes
- How OCaml's first-class module system (`module type MONOID`) translates to Rust traits
- Implement `concat_all` as a generic fold using trait bounds: works for any type that is a `Monoid`
- See how `i32` (addition monoid) and `String` (concatenation monoid) share the same interface

## Rust Application
`trait Monoid { fn empty() -> Self; fn combine(self, other: Self) -> Self; }` defines the contract. `concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M` folds the iterator using `fold(M::empty(), M::combine)` — a direct translation of the mathematical definition.

## OCaml Approach
OCaml uses `module type MONOID = sig type t; val empty : t; val combine : t -> t -> t end` and a higher-kinded `concat_all (type a) (module M : MONOID with type t = a)`. First-class modules are passed at the call site: `concat_all (module Sum) [1;2;3;4;5]`.

## Key Differences
1. **Polymorphism mechanism:** OCaml uses first-class modules passed as values; Rust uses generic type parameters with trait bounds resolved at compile time
2. **Identity access:** OCaml calls `M.empty` via the module; Rust calls `M::empty()` as an associated function — both are monomorphized per type
3. **Call site:** OCaml explicitly passes the module `(module Sum)`; Rust infers the monoid implementation from the type of the collection elements

## Exercises
1. Implement `Monoid` for `Vec<T>` where `empty()` is `vec![]` and `combine` is concatenation — verify with `concat_all`
2. Implement a `Product` newtype over `i32` (identity = 1, combine = multiplication) and fold `[1, 2, 3, 4, 5]` into their product
3. Implement `mconcat` as an alias for `concat_all` and verify that `mconcat([empty]) == empty` for all implementations
