📖 **[View on hightechmind.io →](https://hightechmind.io/rust/204-lens-composition)**

---

# Example 204: Lens Composition — Zoom Into Nested Structs
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Compose two `Lens` values into one so that reading or updating a deeply nested field requires a single `get` or `set` call rather than manually threading updates through every level.

## Learning Outcomes

- How to store and call closures in Rust struct fields using `Box<dyn Fn(...)>`
- How `Rc` enables two owned closures to share a single function pointer without copying
- The direct mapping from OCaml's record-of-functions `('s, 'a) lens` to Rust's struct-of-boxed-closures
- Why `Clone` is required on the intermediate type `A` in a composed lens

## OCaml Approach

OCaml represents a lens as a record `{ get; set }` where both fields hold polymorphic functions. `compose` builds a new record whose `get` chains the two getters left-to-right and whose `set` reverses the update right-to-left — the classic van Laarhoven chain. An infix operator `|>>` makes multi-level composition read naturally.

## Rust Approach

Rust stores each lens as a `struct` with two `Box<dyn Fn(...)>` fields. Composition takes ownership of both lenses, wraps `outer_get` in an `Rc` so the new `get` and `set` closures can both call it, and returns a fresh `Lens<S, B>`. Type aliases `GetFn<S, A>` and `SetFn<S, A>` tame clippy's `type_complexity` lint on the raw `Box<dyn Fn>` fields.

## Key Differences

1. **Function storage**: OCaml records hold functions as ordinary values; Rust needs heap-allocated `Box<dyn Fn>` to store closures of different concrete types in the same struct field.
2. **Shared ownership**: OCaml closures share captured values for free; Rust requires `Rc<Box<dyn Fn>>` when two closures must both own the same captured function.
3. **Immutable update**: `{ p with address = a }` (OCaml) vs `Person { address: a, ..p.clone() }` (Rust) — both produce a new value; Rust needs `Clone` for struct update syntax.
4. **Associativity**: Composition is associative in both languages: `(A ∘ B) ∘ C == A ∘ (B ∘ C)`. The test suite verifies this property explicitly.

## Exercises

1. Define a lens for every leaf field in a three-level nested struct and compose them to demonstrate that any deep field can be read and updated via a single composed lens.
2. Implement `over` — a function that lifts a regular function `A -> A` into a lens update, and use it to increment a deeply nested numeric field.
3. Implement an `iso` (isomorphism): a pair of functions `to: A -> B` and `from: B -> A`, compose it with a lens to transform the viewed type, and use it to treat a `String` field as `Vec<char>` through the lens.
