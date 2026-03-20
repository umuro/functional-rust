📖 **[View on hightechmind.io →](https://hightechmind.io/rust/389-newtype-pattern)**

---

# 389: Newtype Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Primitive types like `f64` and `String` carry no semantic meaning. A function taking two `f64` parameters for distance and mass has no protection against passing them in the wrong order. The Mars Climate Orbiter was lost in 1999 because one module used metric units and another used imperial — the compiler saw only `f64`. The newtype pattern wraps primitive types in single-field structs, creating distinct types with zero runtime overhead: `Meters(f64)` and `Kilograms(f64)` cannot be accidentally interchanged.

Newtypes also enable implementing foreign traits on foreign types (since the newtype is yours), adding validation in constructors, and creating refined types like `Email(String)` that guarantee invariants.

## Learning Outcomes

- Understand how newtype structs provide type safety with zero runtime cost
- Learn how to implement `Display`, `Add`, and other traits selectively on newtypes
- See how validated constructors (`Email::new`) maintain invariants at construction time
- Understand the orphan rule benefit: newtypes enable implementing foreign traits for foreign types
- Learn when to use newtypes vs. type aliases (aliases are transparent; newtypes are opaque)

## Rust Application

In `src/lib.rs`, `Meters(pub f64)` and `Kilograms(pub f64)` are distinct types — you cannot add a `Meters` to `Kilograms`. `Add` is implemented for `Meters` producing `Meters`. `Display` formats with units appended. `Email` has a private field with a validating constructor `Email::new` returning `Option<Self>` — the only way to get an `Email` is through validation. This guarantees any `Email` value contains a plausible email address.

## OCaml Approach

OCaml achieves the same with private types in modules: `module Meters : sig type t val create : float -> t val value : t -> float end`. The `.mli` file hides the constructor so users must go through `Meters.create`. Abstract types in OCaml are the direct equivalent of Rust's opaque newtypes. For unit checking, OCaml can also use phantom types: `type 'a distance = Distance of float` where `'a` is `meters` or `feet`.

## Key Differences

1. **Zero-cost**: Rust's newtype `Meters(f64)` has identical layout to `f64` — the compiler strips the wrapper; OCaml's abstract types may carry a tag word in boxed representations.
2. **Selective impl**: Rust lets you implement exactly the traits you want on a newtype (e.g., `Add` but not `Mul`); OCaml abstract types inherit no operations from the underlying type, forcing explicit re-exposure.
3. **Pattern matching**: Rust's newtype can be destructured with `let Meters(x) = m`; OCaml uses `let Distance x = d` with the same destructuring pattern.
4. **Phantom types**: Both languages support phantom types for unit checking; Rust uses `PhantomData<T>`, OCaml uses a phantom type parameter `'a`.

## Exercises

1. **Unit system**: Build a complete unit system with `Meters`, `Feet`, `Seconds`, and `MetersPerSecond`. Implement `Div<Seconds> for Meters` producing `MetersPerSecond`, preventing nonsensical unit combinations at compile time.
2. **Validated types**: Create `Username(String)`, `Password(String)`, and `Age(u8)` newtypes with validated constructors that enforce: username 3-20 chars alphanumeric, password minimum 8 chars, age 0-150. Return `Result<Self, String>`.
3. **Newtype as foreign trait impl**: You need `Display` for `Vec<i32>` (a foreign trait on a foreign type). Create `DisplayVec(Vec<i32>)` newtype and implement `Display` for it, then implement `From<Vec<i32>>` for ergonomic conversion.
