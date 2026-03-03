# Example 100: Phantom Types — Type-Safe Units

**Difficulty:** ⭐⭐⭐
**Category:** Functors and Modules
**OCaml Source:** [Real World OCaml](https://dev.realworldocaml.org/)

## Problem Statement

Use phantom type parameters to create a `Quantity<Unit>` type that prevents adding meters to seconds at compile time, while storing only a float at runtime.

## Learning Outcomes

- Map OCaml's abstract phantom types to Rust's `PhantomData<T>`
- Understand zero-cost abstractions: phantom types add no runtime overhead
- Implement trait-based operator overloading (`Add`)
- Compare phantom types with newtype wrappers

## Key Insight

OCaml declares phantom types (`type meters`) as abstract types with no constructors. Rust uses `PhantomData<T>` — a zero-sized marker that tells the compiler "this type is parameterized by T" without storing anything. Both achieve the same compile-time safety at zero runtime cost.
