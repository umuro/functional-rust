📖 **[View on hightechmind.io →](https://hightechmind.io/rust/897-borrowing-shared)**

---

# 897-borrowing-shared — Shared Borrowing (&T)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Reading data from multiple places simultaneously is safe as long as no one is writing. Rust formalizes this with shared references (`&T`): unlimited readers can hold `&T` simultaneously, but no writer can hold `&mut T` while any `&T` exists. This compile-time "readers-writer lock" prevents data races without runtime overhead. OCaml's GC and immutability-by-default avoid the need for explicit borrowing — values are implicitly shared. Rust's borrow checker is the mechanism that makes systems-level Rust safe without garbage collection.

## Learning Outcomes

- Use `&T` to borrow data without transferring ownership
- Understand the "multiple readers, zero writers" rule enforced at compile time
- Pass `&str` and `&[T]` to functions to avoid unnecessary cloning
- Recognize that functions accepting `&[T]` are more flexible than those accepting `Vec<T>`
- Compare with OCaml's GC-based sharing where borrowing is not a programmer concern

## Rust Application

`string_info(s: &str)` borrows the string — the caller still owns the original. `sum_slice(data: &[i32])`, `max_slice`, and `min_slice` each borrow the same slice independently — three simultaneous shared references coexist. `stats(data: &[i32])` demonstrates this: `sum_slice(data)`, `max_slice(data)`, and `min_slice(data)` all hold borrows simultaneously. `contains_duplicate(data: &[i32])` uses `data[..i].contains(&data[i])` — multiple sub-borrows of the same slice at once. The function signature `&[i32]` signals "read-only, no ownership transfer."

## OCaml Approach

OCaml has no equivalent borrowing concept. Passing a list or array to a function shares a pointer — the runtime ensures safety via GC. Multiple functions can read the same list simultaneously without annotation. OCaml's functional style and immutable defaults mean sharing is safe by default. For mutable data (`ref`, `Array`), OCaml relies on the programmer to avoid concurrent mutation — there is no compile-time check like Rust's borrow checker for sequential code.

## Key Differences

1. **Compile-time enforcement**: Rust's borrow checker prevents aliased mutation at compile time; OCaml relies on immutable-by-default and runtime GC.
2. **Explicit annotation**: Rust requires `&T` in function signatures to indicate borrowing; OCaml passes pointers implicitly.
3. **Lifetime tracking**: Rust tracks how long borrows live to prevent use-after-free; OCaml's GC prevents this at runtime.
4. **Slice vs list**: Rust `&[T]` borrows a contiguous slice zero-copy; OCaml `list` is a linked structure — no zero-copy subslice without the `Array` type.

## Exercises

1. Write `statistics(data: &[f64]) -> (f64, f64, f64, f64)` returning (min, max, mean, variance) using only shared references.
2. Implement `common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str` that returns the longest common prefix as a borrowed slice.
3. Write `find_duplicates(data: &[i32]) -> Vec<i32>` that returns only values appearing more than once, using only shared borrows.
