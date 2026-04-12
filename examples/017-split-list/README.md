📖 **[View on hightechmind.io →](https://hightechmind.io/rust/017-split-list)**

---

# 017 — Split a List at Position
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Splitting a list at position n (OCaml 99 Problems #17) — producing `(list[..n], list[n..])` — is the fundamental decomposition operation. It is the inverse of concatenation and the basis for divide-and-conquer algorithms like merge sort, binary search, and quickselect. Every partitioning problem reduces to some variant of split.

Rust's slice operations make this trivial via `v.split_at(n)`, but understanding the recursive construction — peel elements from the front until the counter reaches zero — builds intuition for structural recursion on sequences. This pattern appears in parser combinators (split remaining input after consuming n tokens) and stream processing.

## Learning Outcomes

- Use `v.split_at(n)` for efficient O(1) splitting of slices
- Understand the recursive "count down" pattern for splitting linked lists
- Handle edge cases: n=0, n >= list.length
- Return a tuple `(Vec<T>, Vec<T>)` as the two halves
- Recognize split as the fundamental list decomposition primitive

- Use `v.split_at(n.min(v.len()))` to avoid panics when n exceeds the slice length
- Distinguish between reference-returning `split_at` (zero allocation, O(1)) and value-returning `take/skip` (O(n))

## Rust Application

The idiomatic Rust approach uses `v.split_at(n.min(v.len()))` to safely handle n > v.len(). For owned `Vec` data, `v.into_iter().take(n).collect()` and `v.into_iter().skip(n).collect()` split by consuming the iterator. The recursive approach peels elements from the front while a counter is positive, collecting into the left half, then returns the remainder as the right half.

## OCaml Approach

OCaml's version: `let split lst n = let rec aux acc n = function | [], _ -> (List.rev acc, []) | rest, 0 -> (List.rev acc, rest) | x :: t, n -> aux (x :: acc) (n - 1) (t, n - 1) in aux [] n (lst, n)`. The counter decrements with each element consumed. When it reaches zero or the list is empty, the accumulator becomes the left half and the remainder is the right half.

OCaml's `split_at n list` uses a recursive helper with a counter: `let rec split_at_aux acc n = function | [] -> (List.rev acc, []) | h :: t when n = 0 -> (List.rev acc, h :: t) | h :: t -> split_at_aux (h :: acc) (n-1) t`. The accumulator must be reversed at the end. OCaml has no built-in equivalent to Rust's `split_at`.

## Key Differences

1. **O(1) vs O(n)**: Rust's `slice.split_at(n)` is O(1) — it just computes two pointer/length pairs without copying. OCaml's list split is always O(n) because linked lists require traversal.
2. **Ownership**: Rust's slice split returns references (`&[T]`, `&[T]`). Splitting an owned `Vec` into two owned `Vec`s requires cloning or draining.
3. **Bounds safety**: Rust panics on out-of-bounds slice operations. Always use `n.min(v.len())` for safe splitting. OCaml returns `(List.rev acc, [])` when the list runs out — silently handling the case.
4. **Destructive vs functional**: Rust's `Vec::split_off(n)` mutates the original Vec (left half stays, right half is returned). OCaml's split always produces new lists.

1. **O(1) vs O(n):** `slice.split_at(n)` is O(1) — it just creates two references into the same memory. OCaml's recursive version is O(n) because it must traverse the linked list.
2. **Bounds handling:** Rust's `split_at` panics if `n > len`. Use `n.min(v.len())` to clamp. OCaml's version returns an empty right side when n exceeds the list length.
3. **No copy:** Rust's `split_at` returns two `&[T]` slices — zero allocation, zero copying. Both point into the original data. OCaml's split builds two new lists.

## Exercises

1. **Split at predicate**: Write `split_while(list: &[i32], pred: impl Fn(&i32) -> bool) -> (&[i32], &[i32])` that splits at the first element that does not satisfy `pred`. This is OCaml's `List.partition` / Haskell's `span`.
2. **Chunk by n**: Write `chunks(list: &[i32], n: usize) -> Vec<Vec<i32>>` that splits the list into chunks of size n (with the last chunk potentially smaller). Use `v.chunks(n).map(|c| c.to_vec()).collect()`.
3. **Merge sorted**: Given two sorted `Vec<i32>` from a split+sort, write `merge(left: &[i32], right: &[i32]) -> Vec<i32>` that produces a single sorted result — completing the merge sort algorithm.

4. **Split at predicate**: Implement `split_when<T, F: Fn(&T) -> bool>(list: &[T], pred: F) -> (&[T], &[T])` that splits at the first element satisfying the predicate — like OCaml's `List.split_while`.
5. **Chunked split**: Implement `chunks(list: &[T], size: usize) -> Vec<&[T]>` that splits a list into chunks of `size` (last chunk may be smaller). Use `slice.chunks(n)` as the standard library equivalent.
