📖 **[View on hightechmind.io →](https://hightechmind.io/rust/150-coherence-rules)**

---

# Coherence Rules
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

If two crates independently implement the same trait for the same type, the compiler cannot decide which to use — this is the "coherence problem." Rust's orphan rule prevents this: you can only implement a trait for a type if at least one of the trait or the type is defined in your crate. This guarantees at most one implementation per `(Trait, Type)` pair globally, making trait resolution unambiguous. Understanding these rules prevents confusing "conflicting implementations" errors.

## Learning Outcomes

- Understand why the orphan rule exists and what problem it solves
- Learn what is allowed: implementing your trait for external types, external traits for your types
- Understand what is forbidden: implementing external traits for external types
- See workarounds: newtype wrapper pattern, blanket implementations

## Rust Application

The orphan rule in practice:
- **Allowed**: `impl MyTrait for Vec<i32>` (your trait, their type)
- **Allowed**: `impl Display for MyStruct` (their trait, your type)
- **Forbidden**: `impl Display for Vec<i32>` (both external — coherence violation)
- **Workaround**: `struct MyVec(Vec<i32>); impl Display for MyVec { ... }` (newtype)

Blanket implementations like `impl<T: Debug> MyTrait for T` are allowed if `MyTrait` is yours. The "fundamental" types (`Box<T>`, `&T`) have special rules. The `#[fundamental]` attribute (unstable) relaxes orphan rules for specific types.

## OCaml Approach

OCaml has no orphan rule. Any module can implement any type class (module signature) for any type. This can lead to coherence problems in practice — two modules providing different `compare` functions for the same type. OCaml's solution is convention (use the canonical `compare` from the standard library) and the functor pattern (pass the implementation explicitly rather than resolving it globally).

## Key Differences

1. **Global coherence**: Rust enforces global coherence at compile time; OCaml relies on convention and explicit passing to avoid incoherence.
2. **Implicit resolution**: Rust resolves trait impls globally and implicitly; OCaml passes modules explicitly, so incoherence is visible and manageable.
3. **Orphan workaround**: Rust's newtype is the standard orphan workaround; OCaml has no need for it since there is no orphan restriction.
4. **Blanket impls**: Rust's blanket impls (`impl<T: Trait> OtherTrait for T`) are powerful but interact subtly with coherence; OCaml's equivalent (functor application) is always explicit.

## Exercises

1. Demonstrate the orphan rule: try `impl Display for Vec<i32>` and observe the error, then fix it with a newtype.
2. Write a blanket implementation `impl<T: Clone + Debug> Describable for T` and verify it applies to `i32`, `String`, and a custom struct.
3. Implement the same trait for both `i32` and a custom type in the same crate and verify the compiler resolves calls correctly.
