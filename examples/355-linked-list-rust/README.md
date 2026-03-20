📖 **[View on hightechmind.io →](https://hightechmind.io/rust/355-linked-list-rust)**

---

# 355: LinkedList in Rust
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Linked lists are the canonical recursive data structure of computer science — every functional language builds on them. Yet in practice, `Vec` outperforms `LinkedList` for almost every use case on modern hardware because CPU caches favor contiguous memory. Rust's `std::collections::LinkedList` is a doubly-linked list that exists mainly for O(1) `append` (splicing) and O(1) `split_off` at arbitrary positions. Understanding when linked lists are appropriate — and when `Vec` is better — is a key data structure competency. This example demonstrates Rust's stdlib `LinkedList` while explaining why it's rarely the right choice.

## Learning Outcomes

- Build a `LinkedList<T>` from an iterator with `.collect()`
- Splice two lists together in O(1) using `.append(&mut other)`
- Split a list at an index in O(n) using `.split_off(at)`
- Iterate from front or back with `.iter()` and `.iter().rev()`
- Understand why `Vec` beats `LinkedList` for most use cases (cache locality)
- Know the one legitimate use case: O(1) splice at a `CursorMut` position

## Rust Application

```rust
use std::collections::LinkedList;

pub fn build_list<T>(items: Vec<T>) -> LinkedList<T> {
    items.into_iter().collect()
}

pub fn concat<T>(mut a: LinkedList<T>, mut b: LinkedList<T>) -> LinkedList<T> {
    a.append(&mut b); // O(1) — just pointer updates, no element copying
    a
}

pub fn split_at<T>(mut list: LinkedList<T>, at: usize) -> (LinkedList<T>, LinkedList<T>) {
    let second = list.split_off(at.min(list.len())); // O(n) to find position
    (list, second)
}
```

`append` is O(1) because it just updates the `tail` pointer of `a` to point to the `head` of `b`. This is the main advantage over `Vec::extend`, which copies all elements. However, individual element access is O(n), and even iteration is slower than `Vec` due to pointer chasing.

## OCaml Approach

In OCaml, singly-linked lists ARE the language's primary data structure — `list` is built-in:

```ocaml
let build_list items = items  (* list is already a linked list *)

let concat a b = List.append a b  (* O(|a|) — copies a, shares b *)

(* split_at: O(n) *)
let split_at lst n =
  let rec go acc lst = function
    | 0 -> (List.rev acc, lst)
    | n -> match lst with
      | [] -> (List.rev acc, [])
      | x :: rest -> go (x :: acc) rest (n - 1)
  in
  go [] lst n
```

OCaml lists are persistent (immutable) singly-linked lists. `List.append` is O(|a|) because it copies the first list, sharing the tail. Rust's `LinkedList::append` is O(1) because it's a doubly-linked list with owned nodes.

## Key Differences

| Aspect | Rust `LinkedList` | OCaml `list` |
|--------|------------------|--------------|
| Link type | Doubly-linked | Singly-linked |
| Mutability | Mutable in-place | Immutable (persistent) |
| `append` cost | O(1) (pointer swap) | O(n) (copies first list) |
| `prepend` cost | O(1) | O(1) (`x :: rest`) |
| Cache performance | Poor (heap-allocated nodes) | Poor (same) |
| Recommended? | Rarely; prefer `Vec` | Yes — OCaml's primary collection |

## Exercises

1. **Performance comparison**: Implement insertion-at-front for both `Vec` and `LinkedList` 10,000 times; measure elapsed time; verify that `Vec::insert(0, x)` is O(n) while `LinkedList::push_front` is O(1).
2. **Recursive processing**: Implement a function that reverses a `LinkedList<i32>` by draining it into a stack (`Vec`) and rebuilding; compare to `list.into_iter().rev().collect()`.
3. **Cursor editing**: Using the nightly `cursor_mut` API (or simulate with `split_off`/`append`), implement a text buffer that supports O(1) character insertion at the current cursor position.
