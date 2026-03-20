📖 **[View on hightechmind.io →](https://hightechmind.io/rust/141-singleton-types)**

---

# Singleton Types

## Problem Statement

A singleton type has exactly one value — the type and its single inhabitant are isomorphic. Singleton types bridge the gap between values and types, enabling dependent-type-like programming where a runtime value is "promoted" to the type level. This is used in type-safe array indexing (a bound checked once becomes a type-level certificate), in protocol specifications, and in any setting where you want to carry proof of a runtime fact in the type system.

## Learning Outcomes

- Understand what singleton types are and how they connect runtime values to type-level information
- Learn the `Singleton` trait pattern in Rust: a type with a unique `VALUE` constant
- See how singletons enable safe array access without repeated bounds checks
- Understand the relationship to dependent types in Agda, Coq, and Idris

## Rust Application

A singleton type in Rust is typically a zero-sized struct paired with a `const VALUE` via a trait. `struct True; impl Singleton<bool> for True { const VALUE: bool = true; }` defines a type that uniquely represents the value `true` at the type level. More practically, `Index<N>` where `N: Singleton<usize>` can be used to perform a bounds check once (when the index is created) and then use the type as a certificate of validity for subsequent accesses.

## OCaml Approach

OCaml can encode singleton types using GADTs where each constructor carries a unique type index:
```ocaml
type 'a sing =
  | STrue : bool sing
  | SFalse : bool sing
  | SZero : int sing
  | SSucc : 'n sing -> (int) sing
```
OCaml's GADTs allow the singleton value to be recovered from the type directly in pattern matches, making the bridge between types and values more natural than in Rust.

## Key Differences

1. **Pattern matching**: OCaml's GADT singletons can be matched to recover the underlying value; Rust's trait-based singletons use `const VALUE` for recovery.
2. **Dependent functions**: OCaml's GADT singletons enable locally dependent functions; Rust achieves similar effects but with more boilerplate.
3. **Erasure**: Both approaches erase singleton types at runtime (zero-sized types in Rust, phantom types in OCaml); the singleton value is recovered from `VALUE` or pattern matching, not stored.
4. **Practical limit**: Rust's const generics (example 126) often serve as a more ergonomic alternative to singleton types for numeric domains.

## Exercises

1. Implement `struct True; struct False;` as Rust boolean singletons with a `BoolSingleton` trait and `const VALUE: bool`.
2. Write a `BoundedIndex<const N: usize>` type that can only be constructed when the index is less than `N`, providing a compile-time certificate of validity.
3. Implement `to_runtime<S: Singleton<bool>>(s: S) -> bool` that recovers the runtime value from the singleton type.
