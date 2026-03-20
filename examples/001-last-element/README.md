📖 **[View on hightechmind.io →](https://hightechmind.io/rust/001-last-element)**

---

# 001 — Last Element of a List
**Difficulty:** ⭐  
**Category:** Functional Programming  


**Difficulty**: ⭐
**Category**: Lists & Pattern Matching
**Source**: [OCaml.org 99 Problems #1](https://ocaml.org/problems#1)

---

## Problem Statement

Find the last element of a list. Return `None` if the list is empty.

```
last([1, 2, 3, 4])  →  Some(4)
last([])             →  None
```

---

## Learning Outcomes

- How Rust slice patterns (`[x]`, `[_, rest @ ..]`) mirror OCaml list patterns
- The difference between borrowing (`&[T]` / `Option<&T>`) and ownership
- Why Rust prefers iteration over recursion for list traversal
- Three idiomatic ways to reach the same result

---

## OCaml Approach

OCaml uses linked lists and recursive pattern matching as a first-class idiom:

```ocaml
let rec last = function
  | []  -> None
  | [x] -> Some x
  | _ :: t -> last t
```

The compiler optimises tail calls, so deep recursion is safe. The standard
library's `List.rev` + head-match is also common.

## Rust Approach

Rust represents sequences as slices (`&[T]`), which are contiguous in memory.
This enables O(1) random access, so `slice::last()` is the natural first
choice. Recursive slice-pattern matching is supported but not tail-call
optimised, making it educational rather than production-grade.

```rust
// O(1) — preferred
pub fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}
```

---

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Primary sequence type | Linked list | Slice / Vec |
| Recursive style | Idiomatic, TCO guaranteed | Supported, no TCO |
| Null safety | `option` type | `Option<T>` |
| Memory model | GC-managed | Borrow checker enforces lifetimes |
| Stdlib call | `List.rev lst \| List.hd_opt` | `slice.last()` |

## Exercises

1. Implement `second_to_last` that returns the second-to-last element of a list as an `Option`.
2. Implement `last_n` that returns the last `n` elements of a list as a `Vec`, returning an empty vec if the list is shorter.
3. Generalize `last_element` into a `last_by` function that accepts a predicate and returns the last element satisfying it, then use it to find the last even number in a list of integers.
