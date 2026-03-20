📖 **[View on hightechmind.io →](https://hightechmind.io/rust/279-iterator-nth)**

---

# 279: Random Access with nth()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Accessing the element at a specific zero-based index in an iterator requires advancing past all preceding elements. Unlike array indexing (O(1)), `nth(n)` on a generic iterator is O(n) — consuming and discarding the first `n` elements. Crucially, calling `nth(n)` advances the iterator state, so subsequent calls continue from where the previous one left off. This makes `nth()` useful for interleaved stepping through a stream.

## Learning Outcomes

- Understand that `nth(n)` advances the iterator past elements 0..n and returns element n
- Recognize that calling `nth(n)` mutates the iterator — subsequent calls continue from n+1
- Use consecutive `nth()` calls to step through an iterator at varying intervals
- Distinguish from indexing: `nth()` is O(n) for generic iterators, O(1) for indexed collections

## Rust Application

`Iterator::nth(n)` consumes elements 0 through n-1 and returns `Option<T>` at position n, or `None` if fewer than n+1 elements exist. After calling `nth(n)`, the iterator's position is at n+1:

```rust
let v = [10i32, 20, 30, 40];
assert_eq!(v.iter().nth(2), Some(&30));
assert_eq!(v.iter().nth(5), None); // out of bounds

// State-advancing: consecutive nth() calls continue from current position
let mut it = [1i32, 2, 3, 4, 5].iter();
assert_eq!(it.nth(1), Some(&2)); // consumes 1, 2 — position now at 3
assert_eq!(it.nth(0), Some(&3)); // takes next from position 3
```

## OCaml Approach

OCaml uses `List.nth lst n` for direct indexed access (zero-based, raises `Invalid_argument` on out-of-bounds). For safe access with `Option`, wrap in a try-catch or use `List.nth_opt` (OCaml 4.05+):

```ocaml
let nth_opt lst n =
  try Some (List.nth lst n) with Invalid_argument _ -> None

(* Or tail-recursive: *)
let rec nth_opt lst n = match lst with
  | [] -> None
  | x :: _ when n = 0 -> Some x
  | _ :: xs -> nth_opt xs (n-1)
```

## Key Differences

1. **Safety**: Rust returns `Option<T>` and never panics; OCaml's `List.nth` raises on invalid index, requiring `nth_opt` or exception handling.
2. **Stateful advancement**: Rust's `nth(n)` advances the iterator state — calling it repeatedly steps through positions; OCaml's `List.nth` on the original list does not have this property.
3. **O(n) for lists**: Both languages traverse up to n elements to access position n on linked structures; arrays/slices do O(1) indexing.
4. **Iterator position**: After `nth(n)`, Rust's iterator is positioned at n+1 — the next `next()` or `nth(0)` returns element n+1.

## Exercises

1. Use `nth()` to implement a `take_every_nth` function that collects every nth element of an iterator without using `step_by`.
2. Write a function that takes an iterator and an index `i`, returning the i-th element but consuming the iterator up to that point — show that the iterator is advanced.
3. Implement `nth_from_back` for a `DoubleEndedIterator` using `next_back()` and a loop.
