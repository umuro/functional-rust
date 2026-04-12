📖 **[View on hightechmind.io →](https://hightechmind.io/rust/146-opaque-types)**

---

# Opaque Types
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Information hiding is a foundational software engineering principle: expose what a module does, not how it does it. Opaque types make a concrete type invisible outside its defining module — callers can use the type but cannot inspect its internals, pattern match on it, or construct it directly. This enforces abstraction boundaries, enables safe evolution of internals, and prevents invariant violations from outside the module.

## Learning Outcomes

- Understand opaque types as a module-level abstraction mechanism
- Learn how Rust's `pub struct` with private fields achieves opacity
- See how `impl Trait` in return position creates function-level opaque types
- Compare with OCaml's module type signatures as the standard opacity mechanism

## Rust Application

Rust achieves opacity at two granularities. At the module level, a struct with private fields is opaque: `pub struct Handle(u64);` cannot be constructed or inspected from outside the module — only the module's API is accessible. At the function level, `fn make() -> impl Trait` returns an opaque type — the caller knows only the trait interface, not the concrete type. The `pub use` mechanism controls what the module re-exports, enabling fine-grained encapsulation.

## OCaml Approach

OCaml's module system is the primary mechanism for opacity. A module with signature `sig type t val create : unit -> t val use : t -> unit end` hides the definition of `t` from callers. The implementation can be `struct type t = int ... end` — the `int` is invisible outside the module. This is the standard way OCaml libraries expose abstract types: `Buffer.t`, `Hashtbl.t`, `Printf.formatter` are all opaque types.

## Key Differences

1. **Granularity**: OCaml's opacity is at the module level via signatures; Rust supports both module-level (private fields) and function-level (`impl Trait`) opacity.
2. **Pattern matching**: OCaml's opaque types cannot be pattern matched by callers; Rust's structs with private fields similarly prevent construction and field access.
3. **Multiple opaque types**: OCaml modules can expose several opaque types in one signature; Rust modules expose one opaque type per struct definition.
4. **Dynamic opacity**: Rust's `Box<dyn Trait>` erases the concrete type dynamically; OCaml achieves this with first-class modules or GADTs.

## Exercises

1. Create a `Token` opaque type (private `u64` field) with a factory function `Token::new()` and an `is_valid` method — verify it cannot be constructed from outside the module.
2. Implement a session handle: `fn open_session() -> impl Session` where `Session` is a trait with `read`, `write`, and `close` methods.
3. Design an opaque `Config` type that loads from a file and exposes a `get(key: &str) -> Option<&str>` method, hiding all implementation details.
