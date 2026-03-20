📖 **[View on hightechmind.io →](https://hightechmind.io/rust/121-closure-capture)**

---

# Closure Capture Modes
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When a closure uses a variable from its enclosing scope, it must determine how to capture it: share a reference, take a mutable reference, or move ownership. Getting this wrong leads to use-after-free (in unsafe code), data races, or borrow conflicts. Rust enforces the correct capture mode at compile time based on how the closure uses each variable. Understanding capture modes is essential for writing closures that outlive their creation scope (e.g., closures passed to threads).

## Learning Outcomes

- Understand the three capture modes: by shared reference, by mutable reference, and by move
- Learn when the `move` keyword is required and what it implies for `Copy` vs non-`Copy` types
- See how the borrow checker prevents concurrent mutation through a captured `&mut T`
- Practice building closures that are safe to return from functions or send across threads

## Rust Application

The code illustrates all three modes. `add_one_to` uses `move` to capture a `Copy` integer — equivalent to copying. `sum_with_closure` captures `total` by `&mut` — the borrow checker ensures no other access to `total` while the closure exists. `make_multiplier` uses `move` so the returned closure owns `factor` and does not reference the caller's stack. `make_appender` moves a `Vec<String>` into the closure, making the closure the sole owner.

## OCaml Approach

OCaml closures always capture variables by reference to a shared, GC-managed environment. There is no `move` keyword — the GC keeps captured values alive as long as the closure lives. Mutation inside OCaml closures requires explicit `ref` cells; a plain `let x = 5` captured in a closure is read-only by structural convention, not enforced by the type system.

## Key Differences

1. **Capture mechanism**: OCaml always captures by environment reference (GC-managed); Rust chooses the minimal capture mode or moves on `move`.
2. **Move semantics**: Rust's `move` closures take ownership — the original binding is no longer usable; OCaml has no equivalent concept.
3. **Mutable capture**: OCaml requires `ref` cells for mutable captures; Rust infers `&mut` capture automatically when the closure mutates a variable.
4. **Thread safety**: Rust `move` closures over `Send` types can be sent to threads; OCaml closures are not thread-safe by default (Domain API is separate).

## Exercises

1. Write a `make_adder(n: i32) -> impl Fn(i32) -> i32` using `move` and verify the original `n` is still usable after the closure is created (because `i32` is `Copy`).
2. Try capturing a `String` by shared reference in a returned closure — observe the lifetime error, then fix it with `move`.
3. Implement a thread-safe counter using `Arc<Mutex<u32>>` captured by a `move` closure passed to `thread::spawn`.
