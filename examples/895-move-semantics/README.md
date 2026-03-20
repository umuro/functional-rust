📖 **[View on hightechmind.io →](https://hightechmind.io/rust/895-move-semantics)**

---

# 895-move-semantics — Move Semantics
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

In C++, copying data is implicit and expensive — passing a `std::string` to a function copies the heap allocation by default. C++11 introduced move semantics as an opt-in optimization. Rust inverts the default: passing a value to a function always moves ownership, making the original binding invalid. This prevents use-after-free at compile time without a garbage collector. OCaml avoids the issue through garbage collection — all values are GC-managed, and the runtime ensures safety. Understanding Rust's move semantics is the entry point to its ownership model and the foundation for the borrow checker.

## Learning Outcomes

- Understand that passing a heap-owning type by value transfers ownership in Rust
- Recognize the compiler error when attempting to use a moved value
- Use borrowing (`&T`) to share without transferring ownership
- Implement closures that capture by move (`move` keyword)
- Compare with OCaml's GC-managed model where ownership is not a programmer concern

## Rust Application

`consume_string(s: String) -> usize` takes ownership — the caller cannot use `s` after this call. `borrow_string(s: &str) -> usize` borrows — the caller retains ownership. `greet(p: Person) -> String` consumes the `Person`; `greet_ref(p: &Person) -> String` borrows it. `make_prefixer(prefix: String) -> impl Fn(&str) -> String` uses `move` to capture `prefix` by value into the closure — the closure takes ownership. This is required when the closure outlives the frame where `prefix` was created.

## OCaml Approach

OCaml has no move semantics. All values are heap-allocated and GC-managed (except small integers and unboxed floats in certain contexts). Passing a value to a function passes a pointer — the original binding remains valid. "Ownership" is not a concept in OCaml; instead, the GC tracks reachability. Closures capture values by reference implicitly. The trade-off: OCaml avoids ownership complexity at the cost of GC pauses and less predictable memory behavior.

## Key Differences

1. **Default behavior**: Rust moves ownership by default for heap types; OCaml passes a pointer with GC tracking — no ownership transfer.
2. **Compile-time vs runtime safety**: Rust proves memory safety at compile time via the borrow checker; OCaml proves it at runtime via the GC.
3. **Closure capture**: Rust closures must declare capture mode (`move` for ownership, implicit reference otherwise); OCaml closures capture by reference implicitly.
4. **Copy types**: Rust `Copy` types (integers, booleans, etc.) are copied bitwise on assignment — no move. OCaml integers are always passed by value too.

## Exercises

1. Write a function `take_and_return(s: String) -> (String, usize)` that returns both the string and its length, demonstrating how to give ownership back.
2. Implement a `Builder` struct that takes a `Vec<String>` by move, appends items mutably, and returns the completed `Vec<String>` when `.build()` is called.
3. Write a closure that captures a `HashMap<String, i32>` by move and returns a lookup function — explain why `move` is necessary.
