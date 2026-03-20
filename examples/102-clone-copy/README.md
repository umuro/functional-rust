📖 **[View on hightechmind.io →](https://hightechmind.io/rust/102-clone-copy)**

---

# 102-clone-copy — Clone vs Copy
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Copying data efficiently is fundamental to both performance and memory safety. C++ distinguishes copy constructors from move constructors. Rust makes the distinction even sharper: the `Copy` trait marks types for which a bitwise duplicate is always safe and cheap (integers, booleans, small structs with no heap pointers), while `Clone` is the explicit mechanism for deep copies that may allocate.

This distinction has a practical impact: if you pass a `Copy` type to a function, the original remains usable. If you pass a non-`Copy` type, it is moved. This drives API design decisions about which types should derive `Copy`.

## Learning Outcomes

- Understand the `Copy` trait and which types implement it
- Use `.clone()` explicitly for heap-allocated types
- Know why `String` is not `Copy` but `&str` is `Copy`
- Recognise the performance implications of deep clone versus shallow copy
- Design types that intentionally are or are not `Copy`

## Rust Application

`src/lib.rs` defines `Point` (derives both `Clone` and `Copy` because it holds only `f64` values) and `Person` (derives `Clone` only because it holds `String`). `demonstrate_copy` shows that assigning a `Point` to another variable does not invalidate the original. `demonstrate_clone` shows that `Person` requires `.clone()` to copy while keeping the original; a plain assignment moves instead.

The rule: a type can derive `Copy` only if all its fields implement `Copy`. Anything with a `String`, `Vec`, or `Box` field cannot be `Copy`.

## OCaml Approach

OCaml has no explicit copy/clone distinction. All values are either unboxed (integers, booleans) or heap-allocated with GC-managed sharing. Assigning a value in OCaml always creates an alias — both bindings point to the same heap node. Deep copying requires explicit library functions:

```ocaml
let deep_copy_list lst = List.map Fun.id lst
let deep_copy_array arr = Array.copy arr
```

The GC handles memory without requiring the programmer to track who owns what.

## Key Differences

1. **Implicit vs explicit**: OCaml copies are always implicit (aliases via GC); Rust requires explicit `.clone()` for heap types, making allocation visible.
2. **`Copy` marker**: Rust's `Copy` is a compiler-enforced opt-in; OCaml has no equivalent — all types are freely assignable.
3. **Mutable state**: In OCaml, aliasing a mutable value (like a `ref`) means both names see mutations; in Rust, `Clone` creates an independent copy with no shared state.
4. **Performance visibility**: Rust's explicit `.clone()` makes heap allocation visible at call sites; OCaml hides GC costs.

## Exercises

1. Create a `Matrix2x2([[f64; 2]; 2])` struct. Explain why it can derive `Copy` even though it contains an array, and verify by using it after a move.
2. Write a function that takes a `Vec<String>` by value, clones each element, and returns a `Vec<String>` with all strings uppercased — keeping the original unmodified.
3. Design a `Handle(u64)` newtype that intentionally does NOT derive `Copy` to force explicit ownership management. Demonstrate the compile error when trying to use it after a move.
