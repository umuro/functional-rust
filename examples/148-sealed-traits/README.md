📖 **[View on hightechmind.io →](https://hightechmind.io/rust/148-sealed-traits)**

---

# Sealed Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A public trait that external crates can implement is an extension point — any third party can add new implementations. Sometimes that is undesirable: a trait for serialization formats should not be implementable outside the library, or a set of marker types should be closed for soundness. The sealed trait pattern prevents external implementations by requiring implementors to also implement a private "seal" trait that is inaccessible outside the crate.

## Learning Outcomes

- Understand why you might want to prevent external implementations of a public trait
- Learn the sealed trait pattern: a public trait with a private supertrait bound
- See how sealed traits enable exhaustive matching and soundness guarantees
- Recognize the pattern in standard library traits: `Sealed` is used in `std::io::Write` internals

## Rust Application

The sealed trait pattern in Rust:
```rust
mod private {
    pub trait Sealed {}
}
pub trait MyTrait: private::Sealed {
    fn method(&self);
}
// Only types in this crate can implement `Sealed`, so only they can implement `MyTrait`
impl private::Sealed for MyType {}
impl MyTrait for MyType { ... }
```
External crates can use `MyTrait` as a bound and call its methods, but cannot implement it because `private::Sealed` is inaccessible. This is the canonical way to close a trait set in Rust.

## OCaml Approach

OCaml achieves closed type sets through module type signatures — a module with an opaque type `t` and a specific set of operations can only be extended within its defining module. More directly, OCaml's closed variant types (regular `type t = A | B | C`) prevent external extension by design — new variants cannot be added from outside the module. This is the idiomatic OCaml approach for closed sum types.

## Key Differences

1. **Openness by default**: Rust traits are open by default (any crate can implement); OCaml's variant types are closed by default (new variants require source changes).
2. **Sealing mechanism**: Rust uses a private supertrait; OCaml uses module type opacity (the implementation type is hidden).
3. **Polymorphic variants**: OCaml's open variant types (`[> `A | `B]`) can be extended by external code, providing the opposite: open extension — similar to Rust's open traits.
4. **Exhaustiveness**: Both sealed Rust traits and closed OCaml variants enable exhaustive analysis by the compiler.

## Exercises

1. Create a sealed `Numeric` trait that is implemented only for `i32`, `f64`, and `u64` within your crate.
2. Verify that attempting to implement `Numeric` on a type from another crate produces a compile error referencing the inaccessible `Sealed` supertrait.
3. Use a sealed trait to implement a visitor pattern where the set of visitable node types is fixed and known to the library.
