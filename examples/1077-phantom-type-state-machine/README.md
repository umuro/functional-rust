📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1077-phantom-type-state-machine)**

---

# Example 1077: Phantom Type State Machine — File Handle

**Difficulty:** ⭐⭐⭐
**Category:** Type System / Phantom Types
**OCaml Source:** https://dev.realworldocaml.org/

## Problem Statement

Use phantom types to enforce that a file handle can only be read when open, and that closing it prevents further reads — all checked at compile time, not runtime.

## Learning Outcomes

- Phantom types in Rust via `PhantomData<T>` vs OCaml's type parameter trick
- Zero-cost type-level state machines (no runtime overhead)
- How `move` semantics enforce state transitions (consuming the old handle)
- Comparison with runtime state checks via enums

## OCaml Approach

OCaml uses phantom type parameters on a record type. The `opened` and `closed` types are abstract — they have no values. Functions constrain which phantom type is accepted, so `read_line` only works on `opened handle` values. The type checker enforces this statically.

## Rust Approach

Rust uses zero-sized marker types (`struct Opened;`) and `PhantomData<State>` to carry the type parameter without runtime cost. Methods are implemented only on `FileHandle<Opened>`, so calling `read_line` on a closed handle is a compile error. The `close` method *consumes* the open handle (move semantics), preventing use-after-close.

## Key Differences

1. **Phantom types:** OCaml uses abstract types; Rust uses `PhantomData<T>` with zero-sized marker structs
2. **State transition:** OCaml returns a new value; Rust *moves* the old one, making reuse impossible
3. **Method dispatch:** OCaml uses standalone functions with type constraints; Rust uses `impl` blocks on specific type parameters
4. **Runtime comparison:** Both languages can also do runtime checks (enum/variant), but phantom types are zero-cost
