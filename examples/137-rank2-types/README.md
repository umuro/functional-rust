📖 **[View on hightechmind.io →](https://hightechmind.io/rust/137-rank2-types)**

---

# Rank-2 Types Simulation
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A rank-2 type is a function that takes a polymorphic argument — a function that must work for all types, not just one specific type chosen by the caller. The classic example: `runST :: (forall s. ST s a) -> a` in Haskell prevents mutable state from leaking out of a computation. In Rust and OCaml, rank-2 polymorphism is simulated using traits with generic methods, which enforce "the callee chooses the type" rather than "the caller chooses the type."

## Learning Outcomes

- Understand the difference between rank-1 (caller chooses type) and rank-2 (callee applies to all types) polymorphism
- Learn how Rust traits with generic methods simulate rank-2 types
- Understand why traits with generic methods are not dyn-compatible in Rust
- See practical applications: ST monad, generic fold callbacks, universally quantified data

## Rust Application

`trait IdFn { fn apply<T>(&self, x: T) -> T; }` is the rank-2 contract — any implementor must handle every possible `T`, not a specific one chosen at the call site. `apply_id<F: IdFn>(f: &F)` calls `f.apply` with both `i32` and `String` in a single invocation, which would be impossible if `F` were only a `Fn(i32) -> i32`. The `F: IdFn` bound (static dispatch) is required because generic methods are not dyn-compatible.

## OCaml Approach

OCaml supports rank-2 types natively via record polymorphism:
```ocaml
type id_fn = { apply : 'a. 'a -> 'a }
let apply_id f = (f.apply 42, f.apply "hello")
let identity = { apply = fun x -> x }
```
The `'a.` prefix in the record field type means "for all `'a`". This is more ergonomic than Rust's trait simulation and supports `dyn`-style usage naturally. OCaml's `runST` equivalent is straightforward with this mechanism.

## Key Differences

1. **Native support**: OCaml has native rank-2 types via record polymorphism (`'a.`); Rust simulates them with trait-based static dispatch.
2. **Dyn compatibility**: OCaml's rank-2 records can be stored in lists naturally; Rust's rank-2 traits cannot be used with `dyn` (generic methods prevent it).
3. **Ergonomics**: OCaml's `{ apply = fun x -> x }` is compact; Rust requires defining a struct and an `impl IdFn` block.
4. **Type inference**: OCaml infers rank-2 types in most cases; Rust requires explicit trait bounds everywhere rank-2 behavior is needed.

## Exercises

1. Implement a `PolyMapper` trait with `fn map<T, U>(&self, f: impl Fn(T) -> U, x: T) -> U` and use it to apply transformations to different types in one call.
2. Write `apply_to_pair<F: IdFn>(f: &F, x: i32, y: String) -> (i32, String)` using the same `F` for both.
3. Implement a `RunST`-like pattern where a computation over a phantom state type `S` prevents values from escaping their scope.
