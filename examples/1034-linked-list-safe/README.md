📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1034-linked-list-safe)**

---

# 1034-linked-list-safe — Safe Linked List

## Problem Statement

Linked lists are a foundational data structure in functional programming — OCaml's list type is a linked list at the machine level. In Rust, implementing a linked list in safe code is notoriously tricky because the borrow checker must verify that no two nodes share ownership. The canonical safe approach uses `Option<Box<Node<T>>>` for the next pointer, which gives single ownership through the `Box` and `None` for the terminator.

This example demonstrates why Rust linked lists require more thought than in garbage-collected languages, and how `Option<Box<Node<T>>>` cleanly solves the ownership problem.

## Learning Outcomes

- Implement a singly-linked list using `Option<Box<Node<T>>>` in safe Rust
- Understand why `Box<T>` is needed to break the recursive size computation
- Use `.take()` for efficient ownership transfer from `Option<Box<_>>`
- Implement `push`, `pop`, `peek`, and iteration
- Know when to use `std::collections::LinkedList` vs `Vec` in production

## Rust Application

`src/lib.rs` implements `List<T>` with a `head: Option<Box<Node<T>>>` field. `push` wraps a new node in `Box` and sets `node.next = self.head.take()` — the `.take()` replaces `head` with `None` and moves the old head into the new node's `next` field in a single operation. `pop` takes the head, sets `self.head = node.next`, and returns `node.value`.

The `Option<Box<Node>>` pattern is the simplest safe linked list in Rust and is used in stack implementations, undo history, and expression trees.

## OCaml Approach

OCaml's built-in list `'a list` is a singly-linked list with structural sharing:

```ocaml
(* OCaml's list IS a linked list at the machine level *)
type 'a list = [] | (::) of 'a * 'a list

let push value lst = value :: lst
let pop = function [] -> None | x :: rest -> Some (x, rest)
```

OCaml lists are immutable and use GC-managed shared nodes. Appending to the front is O(1); appending to the back is O(n). Both languages agree on the O(1) prepend for linked lists.

## Key Differences

1. **Ownership**: Rust's `Box<Node>` enforces single ownership; OCaml nodes can be shared (immutable structural sharing via GC).
2. **Mutability**: Rust's list is mutable (push/pop in place); OCaml's lists are immutable (prepend creates new node, old list unchanged).
3. **Safety without GC**: Rust's `Option<Box<Node>>` is safe Rust with no garbage collector; OCaml's GC handles the memory automatically.
4. **`std::collections::LinkedList`**: Rust's stdlib doubly-linked list is rarely recommended; `Vec` is almost always better due to cache locality.

## Exercises

1. Add a `reverse(&mut self)` method that reverses the list in place by relinking nodes.
2. Implement `IntoIterator` for `List<T>` so you can use it in `for` loops.
3. Write a `merge_sorted(a: List<i32>, b: List<i32>) -> List<i32>` function that merges two sorted lists into a new sorted list.
