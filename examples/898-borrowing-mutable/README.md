📖 **[View on hightechmind.io →](https://hightechmind.io/rust/898-borrowing-mutable)**

---

# 898-borrowing-mutable — Mutable Borrowing (&mut T)

## Problem Statement

In-place mutation is often more efficient than allocating new values. Sorting a large array in place, incrementing a counter, reversing a buffer — all benefit from direct mutation. Rust allows mutable access via `&mut T` with one strict rule: only one `&mut T` can exist at a time, and no `&T` can coexist with it. This prevents data races and iterator invalidation — the bugs that plagued C++ with concurrent mutation. The rule "exclusive access implies safe mutation" is Rust's solution to the aliasing problem that makes optimizing languages hard.

## Learning Outcomes

- Use `&mut T` to mutate data through a reference without taking ownership
- Understand the "exactly one mutable reference" rule and why it prevents data races
- Perform in-place operations: reversal, doubling, and accumulation
- Recognize how `iter_mut()` provides mutable iteration over a collection
- Compare with OCaml's `ref` cells and `Array` for explicit mutable state

## Rust Application

`increment(c: &mut Counter)` mutates the counter through a reference — the caller retains ownership. `sum_into(data: &[i32], total: &mut i32)` writes the accumulated sum into a caller-owned variable. `reverse_in_place(arr: &mut [i32])` uses `arr.swap(i, n-1-i)` for zero-allocation in-place reversal. `double_all(values: &mut [i32])` uses `values.iter_mut()` for element-wise mutation. The borrow checker prevents calling both `sum_into(data, total)` and reading `*total` simultaneously — the reference must be released first.

## OCaml Approach

OCaml uses `ref` cells for mutable variables: `let total = ref 0 in List.iter (fun x -> total := !total + x) xs`. Arrays are mutable: `Array.iteri (fun i x -> arr.(i) <- x * 2) arr`. OCaml's `Buffer.t` is a mutable string builder. Unlike Rust, OCaml allows multiple references to the same mutable value — no "one writer" restriction. This means OCaml programs can have aliasing mutations that would be compile errors in Rust, potentially leading to harder-to-debug state bugs.

## Key Differences

1. **Exclusivity**: Rust enforces one `&mut T` at a time; OCaml allows multiple references to the same `ref` cell.
2. **Data race prevention**: Rust's single-writer rule prevents data races even in single-threaded code (for consistent reasoning); OCaml relies on the programmer.
3. **Signature signals intent**: Rust `&mut T` in a function signature explicitly declares "this function may modify the argument"; OCaml has no such declaration.
4. **Slice mutation**: Rust `&mut [T]` enables in-place algorithms on slices with zero overhead; OCaml arrays support the same via `arr.(i) <- value`.

## Exercises

1. Implement `normalize_in_place(data: &mut [f64])` that divides each element by the maximum value, mutating in place.
2. Write `fill_with<T: Clone>(data: &mut [T], value: T)` that overwrites all elements with the given value.
3. Implement a `Queue<T>` backed by `Vec<T>` with `enqueue(&mut self, item: T)` and `dequeue(&mut self) -> Option<T>` methods.
