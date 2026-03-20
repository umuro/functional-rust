📖 **[View on hightechmind.io →](https://hightechmind.io/rust/387-sealed-trait-pattern)**

---

# 387: Sealed Trait Pattern

## Problem Statement

Rust's trait system is open by default — any crate can implement any trait for any type (subject to orphan rules). Sometimes library authors want to prevent external implementations: a `Token` trait whose implementors are exactly the types the library defines, ensuring exhaustive handling and preventing downstream breakage when new variants are added. The sealed trait pattern uses a private `Sealed` supertrait to enforce this: only types that implement `private::Sealed` can implement the public trait, and `private::Sealed` cannot be named by external code.

This pattern appears in `tokio`'s `Sealed` trait for internal types, `bytes::Buf`, `futures::Stream`, and many API-stability-sensitive libraries.

## Learning Outcomes

- Understand why you might want to prevent external implementations of a public trait
- Learn how the `mod private` + `Sealed` supertrait pattern enforces this in Rust
- See how public trait bounds on a private supertrait create an unimplementable interface for external users
- Understand the API stability guarantee sealed traits provide
- Learn the difference between sealed traits (prevent impl) and private traits (prevent use)

## Rust Application

In `src/lib.rs`, the `mod private` module contains `pub trait Sealed {}`. The public `Token` trait has `Token: private::Sealed` as a supertrait bound. External code can use `Token` as a trait bound and call its methods, but cannot implement it — because implementing `Token` requires implementing `private::Sealed`, which requires naming the private module. `Identifier` and `Number` are the exhaustive set of implementors, both explicitly implementing `private::Sealed`.

## OCaml Approach

OCaml achieves sealed modules through the module system. A private module signature can expose a type but hide its constructors: `module type SEALED = sig type t = private Foo | Bar end`. External code can pattern-match exhaustively but cannot construct new values. For trait-like sealing, OCaml uses abstract types in signatures where the concrete representation is hidden.

## Key Differences

1. **Mechanism**: Rust uses a private supertrait in a private module; OCaml uses abstract types or private type aliases in module signatures.
2. **Error quality**: Rust's error for attempting to implement a sealed trait mentions the private `Sealed` bound; OCaml's error is a "type not accessible" module error.
3. **Use vs. impl**: Rust's sealed traits are fully usable as bounds and in `dyn Trait` positions; OCaml's abstract types can be used but not extended.
4. **Documentation**: Rust sealed traits can document the pattern explicitly in rustdoc; OCaml hides the mechanism entirely in the `.mli` file.

## Exercises

1. **Sealed codec**: Define a `Codec` sealed trait with `encode` and `decode` methods. Implement it for `JsonCodec` and `BinaryCodec` in the same crate. Write tests proving that a downstream crate cannot add a `XmlCodec` implementation.
2. **Visitor pattern with sealing**: Implement a sealed `Visitor` trait for an AST node hierarchy. Ensure all node types implement the visitor contract while preventing external AST node additions.
3. **Version-gated unsealing**: Design a library that starts with a sealed trait but plans to unseal it in a future version. Write a feature-flag-based migration path using `#[cfg(feature = "unstable")]` to expose the sealing mechanism for pre-release testing.
