📖 **[View on hightechmind.io →](https://hightechmind.io/rust/003-kth-element)**

---

# Example 003: K-th Element
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Find the k-th element of a list. The OCaml version uses 1-based indexing. We provide both 1-based (matching OCaml) and 0-based (idiomatic Rust) versions.

Safe element access by index is a universal problem. Languages like C use raw pointer arithmetic and segfault on out-of-bounds access. Java throws `IndexOutOfBoundsException` at runtime. Rust's `get(index)` returns `Option<&T>`, making the possibility of out-of-bounds explicit at the type level, forcing callers to handle both cases before using the value. This is one of the simplest but most impactful applications of the `Option` type.

## Learning Outcomes

- Safe indexing with `Option` return types (no panics)
- Understanding 0-based vs 1-based indexing conventions
- `slice.get(i)` as the idiomatic safe accessor in Rust
- Recursive indexing via slice pattern matching
- How Rust's `ExactSizeIterator` makes `.nth()` O(1) on slices

## OCaml Approach

Recursive pattern match: if `k = 1`, return the head; otherwise recurse on the tail with `k - 1`. Returns `None` for empty lists. Natural and concise with linked lists.

OCaml's `List.nth list n` raises `Failure "nth"` or `Invalid_argument` on invalid indices — runtime exceptions. A safe OCaml version: `let rec nth_safe = function | ([], _) -> None | (x :: _, 1) -> Some x | (_ :: t, n) -> nth_safe (t, n-1)`. Note that accessing the nth element in a linked list is O(n) — this is an inherent limitation of the data structure, not an implementation choice.

## Rust Approach

`slice.get(index)` provides O(1) safe access. The recursive version uses slice patterns to mirror OCaml. A 1-based wrapper handles the indexing convention difference.

## Key Differences

1. **Indexing convention:** OCaml uses 1-based; Rust uses 0-based (we provide both)
2. **Access complexity:** Rust slices → O(1) random access; OCaml lists → O(k) traversal
3. **Safe access:** Rust's `.get()` returns `Option<&T>`; OCaml's pattern match returns `option`
4. **Borrowing:** Rust returns `&T` (reference); OCaml copies the value
5. **Underflow risk:** 1-based `k - 1` can underflow `usize` in Rust — must guard against `k = 0`

4. **Complexity:** `slice.get(i)` is O(1); OCaml's `List.nth l n` is O(n). This is the fundamental performance argument for arrays over linked lists in most practical use cases.
5. **Indexing convention:** OCaml 99 Problems uses 1-based indexing (element 1 is the first). Rust and most modern languages use 0-based indexing. Always document which convention your function uses.

## Exercises

1. Write `kth_from_end` that returns the `k`-th element counting from the end of the list (1-indexed), returning `None` if out of bounds.
2. Implement `every_kth` that collects every `k`-th element of a slice into a new `Vec` (e.g., every 3rd element starting from index `k-1`).
3. Write `kth_element_sorted` that finds the `k`-th smallest element of an unsorted slice without fully sorting it (selection algorithm), returning `None` if `k` exceeds the length.

4. **Safe index**: Write a wrapper type `SafeIndex<T>` that wraps a `&[T]` and implements `Index<usize>` returning `Option<&T>` instead of panicking.
5. **Negative indexing**: Implement `at_from_end(slice: &[T], k: usize) -> Option<&T>` where `k=1` is the last element, like Python's `list[-1]` — without using negative numbers.
