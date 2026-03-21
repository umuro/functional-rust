📖 **[View on hightechmind.io →](https://hightechmind.io/rust/131-builder-pattern)**

---

# Example 131: Builder Pattern with Typestate
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Construct a complex struct step-by-step with a fluent API where `build()` only
compiles after every required field has been provided. Forgetting a required
field is a compile-time error, not a runtime panic or a `Result` error.

## Learning Outcomes

- How phantom type parameters encode presence/absence of required fields
- How `PhantomData<T>` lets Rust track type information with zero runtime cost
- Why splitting methods across `impl<...>` blocks gates availability by type
- How the same typestate technique scales to multiple independent required fields

## OCaml Approach

OCaml uses phantom type variables in a record type `('name, 'email) user_builder`
where `'name` and `'email` never appear in the concrete fields. Functions like
`set_name` accept `(unset, 'e) user_builder` and return `(set, 'e) user_builder`,
so the compiler rejects a second call to `set_name` and rejects `build` unless
both slots carry the `set` phantom. Field records are copied structurally with
`{ b with field = value }`.

## Rust Approach

Rust uses two zero-sized marker structs (`Missing`, `Present`) and
`PhantomData<(N, E)>` to achieve the same tracking. Each setter is placed in
its own `impl` block constrained to the `Missing` state for that slot:
`impl<E> UserBuilder<Missing, E>`. The return type transitions the slot to
`Present`. The `build()` method exists only on
`impl UserBuilder<Present, Present>`. Because all types are zero-sized or
erased, the pattern has zero runtime overhead — it is pure compile-time
bookkeeping.

## Key Differences

1. **Phantom syntax:** OCaml uses unconstrained type variables `('a, 'b)`
   directly in the type alias; Rust uses `PhantomData<(N, E)>` as an actual
   zero-sized field.

2. **Method gating:** OCaml gates `build` by requiring `(set, set)` in its
   argument type. Rust gates it with `impl UserBuilder<Present, Present>` —
   the method literally does not exist on other instantiations.

3. **Record update vs struct construction:** OCaml's `{ b with name = ... }`
   copies all other fields automatically. Rust must name each field in the
   new struct literal, transferring them from `self` explicitly.

4. **Ownership:** Rust's builder takes `self` by value so the old builder is
   consumed at each transition — no aliasing, no double-use. OCaml copies
   the record functionally, achieving the same single-use semantics.

## Exercises

1. Add a required field to the builder (e.g., a non-optional `name: String`) and enforce at compile time using typestate that `build()` cannot be called before `set_name` is invoked.
2. Extend the builder to support optional fields with default values, and implement `merge` that combines two partially-filled builders by preferring non-None values from the right builder.
3. Implement a fluent HTTP request builder: method, URL, headers (multiple allowed), optional body — using the typestate pattern to ensure `send()` is only available after both method and URL are set.
