📖 **[View on hightechmind.io →](https://hightechmind.io/rust/104-borrowing-mutable)**

---

# 104-borrowing-mutable — Mutable Borrowing
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Mutable references are the mechanism for safe mutation in Rust: you can modify data through a `&mut T` reference, but only one `&mut T` can exist at a time, and no shared `&T` references can coexist with it. This is Rust's compile-time implementation of the mutual exclusion principle: writers exclude all other accessors.

This rule prevents entire classes of bugs: iterator invalidation (mutating a collection while iterating over it), data races in concurrent code, and aliased mutation bugs common in C.

## Learning Outcomes

- Understand the exclusive mutable borrow rule: only one `&mut T` at a time
- Pass `&mut T` to functions to modify data without transferring ownership
- Understand that `&mut T` and `&T` cannot coexist for the same value
- Use `split_at_mut` to get two non-overlapping mutable slices
- Recognise the borrow checker errors that enforce the exclusivity rule

## Rust Application

`src/lib.rs` demonstrates `increment(&mut i32)`, `push_doubled(&mut Vec<i32>, i32)`, and `swap_first_last(&mut [i32])`. The commented-out `bad_example` functions show the compiler errors: you cannot create two `&mut v` references simultaneously, and you cannot hold a `&v` reference while also having a `&mut v`.

`split_at_mut` is the standard library's answer to the "I need two mutable slices from the same buffer" problem — it proves to the compiler that the slices do not overlap, enabling both to be mutable simultaneously.

## OCaml Approach

OCaml has no borrow checker. Mutable values use `ref` or mutable record fields:

```ocaml
let increment x = x := !x + 1

let () =
  let n = ref 42 in
  increment n;     (* n is still accessible *)
  let alias = n in  (* both n and alias point to same ref *)
  increment alias; (* modifies through alias *)
  Printf.printf "%d
" !n  (* 44 — aliasing is silent *)
```

OCaml allows aliased mutation freely. The programmer must reason about aliasing manually. Rust's borrow checker makes aliasing provably absent at compile time.

## Key Differences

1. **Exclusivity enforcement**: Rust's borrow checker prevents aliased mutation at compile time; OCaml allows it silently.
2. **Concurrent safety**: Rust's `&mut T` exclusivity means mutable borrows are inherently thread-safe (no data races); OCaml requires explicit synchronisation.
3. **split_at_mut**: Rust needs a special function to prove two slices do not overlap; OCaml can take two slices from the same array without compiler complaints.
4. **Function signatures**: Rust's `fn f(v: &mut Vec<i32>)` explicitly declares mutation intent; OCaml's type system does not track mutation in function types.

## Exercises

1. Write a `rotate_left(v: &mut Vec<i32>, n: usize)` function that rotates the vector in place using only mutable references.
2. Implement `merge_in_place(a: &mut Vec<i32>, b: &[i32])` that merges `b` into the sorted `a` without allocating a new vector.
3. Use `split_at_mut` to implement `parallel_transform(v: &mut [i32])` that modifies the left half and right half of a slice independently.
