📖 **[View on hightechmind.io →](https://hightechmind.io/rust/009-pack-consecutive)**

---

# Example 009: Pack Consecutive Duplicates

**Difficulty:** ⭐⭐
**Category:** Lists & Grouping
**OCaml Source:** OCaml 99 Problems #9

## Problem Statement

Pack consecutive duplicate elements into sublists.

## Learning Outcomes

- Build nested data structures (`Vec<Vec<T>>`) from flat input
- Use `fold` for stateful accumulation — the functional alternative to loops
- Understand zero-copy grouping with slice references (`&[T]`)
- Compare OCaml's accumulator-based recursion with Rust's imperative and fold styles
- See how borrowing enables efficient grouping without cloning

## OCaml Approach

Uses a tail-recursive helper with two accumulators: `current` (current group) and `acc` (completed groups). Compares consecutive elements and either extends the current group or starts a new one. Returns reversed result.

## Rust Approach

1. **Imperative**: Iterate with a mutable `current` group and `result` vector
2. **Fold**: `fold()` with `last_mut()` to extend or create groups — closest to OCaml's accumulator
3. **Slice-based**: Returns `Vec<&[T]>` — borrows into the original slice, zero copying

## Key Differences

1. **Slice references**: `pack_slices` returns `&[T]` views — no data copied, impossible in OCaml's GC model
2. **`last_mut()`**: Rust can mutate the last element of a Vec through a mutable reference — efficient for the fold pattern
3. **Ownership spectrum**: Three levels offered — owned + cloned, owned + fold, borrowed slices
4. **No `List.rev` needed**: Rust's `Vec::push` appends to the end (O(1) amortized); OCaml prepends to lists and reverses
5. **Memory layout**: Rust's `Vec<Vec<T>>` is contiguous blocks; OCaml's `'a list list` is chains of cons cells

## Exercises

1. Implement `pack_by` — a variant that groups consecutive elements using a key function `f: &T -> K` instead of direct equality.
2. Write `pack_into_counts` that converts a packed list into a list of `(usize, T)` pairs representing run lengths, using your pack function as a building block.
3. Implement `unpack` — the inverse: given a list of `(usize, T)` pairs, produce the original expanded list.
