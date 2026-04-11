📖 **[View on hightechmind.io →](https://hightechmind.io/rust/018-slice-list)**

---

# 018 — Extract a Slice from a List

## Problem Statement

Extracting a contiguous subsequence from positions i to k (OCaml 99 Problems #18) is the subarray operation. It is one of the most common list operations: substring extraction in text editors, windowed queries in time-series databases, submatrix extraction in linear algebra, and range queries in sorted data structures all reduce to slice extraction.

The key insight is understanding indexing conventions: OCaml 99 Problems uses 1-based inclusive indexing `[i..k]`, while Rust slices use 0-based exclusive end `[i..k]`. Getting the off-by-one right is a classic source of bugs, and this problem forces careful reasoning about boundary conditions.

## Learning Outcomes

- Use Rust's slice syntax `&v[start..end]` for O(1) subarray extraction
- Handle 1-based vs 0-based index conversion
- Validate bounds to prevent panics
- Understand that slice extraction on arrays is O(1) (pointer + length), unlike O(k-i) copy
- Implement both reference-returning (O(1)) and copy-producing (O(n)) versions

- Use `v.get(start..end)` returning `Option<&[T]>` for safe slicing that avoids panics on invalid indices
- Convert between 1-based inclusive (OCaml 99) and 0-based exclusive-end (Rust) index conventions

## Rust Application

The idiomatic Rust approach: `fn slice(v: &[i32], i: usize, k: usize) -> &[i32] { &v[i..=k] }` — O(1), returns a reference. For owned output: `v[i..=k].to_vec()`. With 1-based indexing as in OCaml 99 Problems: `&v[(i-1)..k]` where `i` is 1-based start and `k` is 1-based inclusive end. The recursive approach counts down to position i, then collects until position k.

## OCaml Approach

OCaml's version uses 1-based indexing: `let slice lst i k = let rec aux acc n = function | [] -> List.rev acc | x :: t -> if n > k then List.rev acc else if n < i then aux acc (n+1) t else aux (x :: acc) (n+1) t in aux [] 1 lst`. The counter `n` advances through 1-based positions, collecting elements from i to k inclusive.

OCaml's `sub list i k` (1-based inclusive) uses a recursive countdown: skip the first `i-1` elements, then take the next `k-i+1` elements. There is no direct equivalent to Rust's `&v[i..=k]`. OCaml's `List` module doesn't provide a slice operation — only third-party libraries like `Core.List.sub` offer it.

## Key Differences

1. **O(1) vs O(n)**: Rust's `&v[i..k]` is a zero-copy O(1) borrow of existing memory. OCaml's recursive extraction is O(k-i) because it traverses from the start of the list.
2. **Indexing convention**: OCaml 99 Problems uses 1-based inclusive `[i, k]`. Rust slices use 0-based exclusive end `[i, k)` or inclusive `[i..=k]`. Always clarify the convention.
3. **Bounds checking**: Rust panics on out-of-bounds slice access. Use `.get(i..k)` returning `Option<&[T]>` for safe access. OCaml's version silently stops at end of list.
4. **Borrow vs copy**: Rust's `&v[i..k]` borrows without copying. `v[i..k].to_vec()` copies. OCaml always allocates a new list.

1. **O(1) reference:** Rust's `&v[i..k]` returns a reference into existing memory — no allocation. OCaml must build a new list by traversal — O(k-i).
2. **Indexing conventions:** OCaml 99 Problems uses 1-based inclusive. Rust uses 0-based with exclusive end for `[i..k]` or inclusive end for `[i..=k]`. Document clearly which convention your function uses.
3. **Bounds checking:** Rust's indexing panics on out-of-bounds. Use `get(i..k)` which returns `Option<&[T]>` for safe slicing.

## Exercises

1. **Sliding window**: Write `windows_of(v: &[i32], size: usize) -> Vec<&[i32]>` that returns all contiguous subsequences of length `size`. Use Rust's built-in `v.windows(size)`.
2. **Circular slice**: Write `circular_slice(v: &[i32], start: usize, len: usize) -> Vec<i32>` that wraps around the end of the vector using modular arithmetic.
3. **Non-contiguous select**: Write `select(v: &[i32], indices: &[usize]) -> Vec<i32>` that extracts elements at the specified positions (generalization of slice).

4. **Windowed slices**: Implement `windows(list: &[T], width: usize) -> Vec<&[T]>` returning all contiguous windows of the given width. Compare with Rust's built-in `.windows(width)` method.
5. **Safe substring**: Write `safe_slice(v: &[T], start: usize, end: usize) -> Option<&[T]>` that returns `None` instead of panicking when indices are out of bounds. Use `v.get(start..end)`.
