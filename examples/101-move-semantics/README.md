📖 **[View on hightechmind.io →](https://hightechmind.io/rust/101-move-semantics)**

---

# 101-move-semantics — Move Semantics

## Problem Statement

Memory safety without a garbage collector requires a clear ownership model. C++ introduced move semantics in C++11 to avoid expensive deep copies when transferring ownership of heap resources. Rust takes this further: every value has exactly one owner, and assigning a non-`Copy` value to a new variable transfers ownership — the original binding becomes invalid. The compiler enforces this statically, eliminating use-after-free and double-free at zero runtime cost.

OCaml sidesteps the problem with a garbage collector that tracks all references. Rust's move semantics achieve the same memory safety guarantee at compile time, with no runtime overhead.

## Learning Outcomes

- Understand Rust's ownership rule: each value has exactly one owner
- Distinguish between types that move (heap-allocated) and types that copy (stack-only)
- Know that passing a value to a function transfers ownership unless the function borrows
- Understand how returning a value transfers ownership back to the caller
- Recognise the compiler error that results from using a moved value

## Rust Application

`src/lib.rs` shows three scenarios. `take_ownership` receives a `String` by value — the caller can no longer use the original binding after the call. `demonstrate_copy` uses `i32`, which implements `Copy`; assignment does a bitwise copy so both variables remain valid. `demonstrate_vec_move` shows that `Vec<T>` moves on assignment, invalidating `v1`. `create_string` demonstrates that returning a value transfers ownership to the caller, which is how Rust avoids the problem of returning stack-allocated temporaries.

## OCaml Approach

OCaml has no move semantics. All values are managed by the GC, and bindings are references into heap-allocated nodes. You can freely pass a string to multiple functions — the GC ensures the value lives as long as any binding refers to it. The simulated ownership model in `example.ml` uses `ref` and a custom `Moved` sentinel to illustrate the concept, but the compiler does not enforce it.

## Key Differences

1. **Enforcement**: Rust's move semantics are enforced at compile time; OCaml's GC handles safety at runtime with no such restriction.
2. **Copy types**: Rust distinguishes `Copy` types (bitwise copy, both bindings valid) from non-`Copy` types (move, original invalid); OCaml makes no such distinction.
3. **Explicit clone**: To keep ownership in Rust after a move, call `.clone()` explicitly; OCaml structural sharing is implicit.
4. **Function call semantics**: In Rust, passing a value to a function moves it unless you borrow with `&`; in OCaml, all arguments are passed by value but the GC tracks the underlying data.

## Exercises

1. Write a function that accepts a `String` by reference (`&String`) instead of by value, and confirm the caller retains ownership.
2. Implement a `clone_and_modify` function that takes a `Vec<i32>`, clones it, appends a value to the clone, and returns both the original and the modified copy.
3. Create a struct `Wrapper(String)` that does not implement `Copy`. Show that assigning one `Wrapper` to another moves it, then fix the code using `.clone()`.
