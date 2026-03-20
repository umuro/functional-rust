📖 **[View on hightechmind.io →](https://hightechmind.io/rust/103-borrowing-shared)**

---

# 103-borrowing-shared — Shared Borrowing

## Problem Statement

Safe concurrent read access is the foundation of parallel computing. In C, multiple threads reading the same data simultaneously is fine as long as no thread writes, but the language does not enforce this rule — a race condition occurs silently. Rust's borrow checker enforces the reader-writer invariant at compile time: any number of shared references (`&T`) can coexist, but no mutable reference (`&mut T`) can coexist with any other reference.

This is Rust's implementation of the "readers-writers" lock at zero runtime cost — the type system acts as the lock.

## Learning Outcomes

- Understand Rust's shared borrow rule: multiple `&T` references can coexist
- Pass `&T` to functions without transferring ownership
- Hold multiple shared references to the same value simultaneously
- Understand why shared references guarantee the data will not change
- Contrast shared borrowing with mutable borrowing and ownership

## Rust Application

`src/lib.rs` demonstrates `sum` and `count` both borrowing the same slice simultaneously — this compiles because both borrows are shared. `average` calls both functions on the same data, with two `&[i32]` borrows active at the same time. `demonstrate_multiple_borrows` creates three explicit shared references `r1`, `r2`, `r3` to the same `Vec` and uses them concurrently.

In Rust's ownership model, passing `data` to a function as `&data` (shared borrow) is the most common operation — the function reads the data without taking ownership, so the caller retains access.

## OCaml Approach

OCaml has no borrow checking. Any binding can read any value at any time because the GC manages all values:

```ocaml
let data = [1; 2; 3; 4; 5]
let s = List.fold_left (+) 0 data  (* data still accessible *)
let n = List.length data            (* data still accessible *)
let avg = float_of_int s /. float_of_int n
```

Shared reading is always safe in OCaml because the GC ensures the data lives as long as any reference exists. There is no way to express or enforce mutation exclusivity at the type level (without external libraries like `Base.Ref` with locking).

## Key Differences

1. **Compile-time enforcement**: Rust enforces the readers-writers rule at compile time; OCaml relies on the programmer to avoid races in concurrent code.
2. **Zero runtime cost**: Rust's shared references are raw pointers at the machine level; OCaml's references go through GC indirection.
3. **Concurrent reads**: Rust guarantees that a `&T` reference means the data cannot change — no locking needed for read-only parallel access; OCaml requires explicit synchronisation for safe concurrent mutation.
4. **Lifetime tracking**: Rust's compiler tracks how long each borrow lives to prevent dangling pointers; OCaml's GC prevents dangling references at runtime.

## Exercises

1. Write a `stats(data: &[f64]) -> (f64, f64, f64)` function that returns (min, max, mean) using three separate passes over the borrowed slice.
2. Create a struct `ReadOnlyView<'a, T> { data: &'a [T] }` that exposes read-only operations and confirm you cannot store a `&mut [T]` in it.
3. Demonstrate that passing `&data` to two concurrent threads (using `std::thread::scope`) works without locks when neither thread mutates the data.
