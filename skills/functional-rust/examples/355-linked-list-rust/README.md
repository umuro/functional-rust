# 355: Linked List in Rust

**Difficulty:** 3  **Level:** Advanced

Rust's ownership system makes linked lists surprisingly tricky — and teaches you why `Vec` is almost always better.

## The Problem This Solves

Linked lists are the default collection in OCaml and Haskell — they're elegant, immutable, and naturally recursive. In Rust, they're an educational challenge. The ownership system prevents the naive recursive struct definition: a `Node` that contains a `Box<Node>` is fine, but circular references or doubly-linked lists require `Rc<RefCell<T>>` or `unsafe` — complexity that wouldn't exist in a GC language.

Understanding why linked lists are hard in Rust teaches you core ownership concepts: why you can't have two owners, why self-referential structs need `Pin`, and why cache locality makes `Vec` faster for most use cases even when a linked list would have better asymptotic complexity.

`std::collections::LinkedList` exists but is rarely recommended. The Rust community's `Learn Rust With Entirely Too Many Linked Lists` is a famous tutorial precisely because linked lists stress-test ownership.

## The Intuition

In OCaml, a list is *the* default: `[1;2;3]` is syntactic sugar for `1 :: 2 :: 3 :: []`. Each `::` cons cell is a heap-allocated node. The GC handles all sharing and memory management.

In Rust, `Vec<T>` is the default. It's a contiguous heap-allocated array — sequential memory access is 10–100× faster on modern CPUs due to cache lines. A linked list scatters nodes across the heap; every `next` pointer dereference is a potential cache miss.

Use `LinkedList` when: you need O(1) splits and appends in the middle, or you're implementing an intrusive data structure. Use `Vec` for everything else.

## How It Works in Rust

```rust
// Safe recursive list using Box (heap allocation)
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),  // Box breaks the infinite-size recursion
}

impl<T: Clone> List<T> {
    fn new() -> Self { Self::Nil }

    // Prepend — creates a new Cons node, takes ownership of previous list
    fn cons(self, x: T) -> Self {
        Self::Cons(x, Box::new(self))
    }

    fn to_vec(&self) -> Vec<T> {
        let mut v = Vec::new();
        let mut cur = self;
        while let Self::Cons(x, next) = cur {
            v.push(x.clone());
            cur = next;
        }
        v
    }
}

// Usage: builds list in reverse (stack-like)
let list = List::new().cons(5).cons(4).cons(3).cons(2).cons(1);
// list.to_vec() == [1, 2, 3, 4, 5]

// std LinkedList: doubly-linked, O(1) push/pop from both ends
let mut ll: LinkedList<i32> = LinkedList::new();
ll.push_back(1);
ll.push_front(0);
```

`Box<List<T>>` is essential: without it, `List<T>` contains a `List<T>` which contains a `List<T>` — an infinitely-sized type the compiler rejects. `Box` indirects through a pointer of known size.

## What This Unlocks

- **Ownership intuition** — recursive ownership chains, the role of `Box`, and why self-referential types are hard.
- **Performance awareness** — understand why `Vec` beats `LinkedList` for iteration-heavy workloads.
- **Functional patterns** — implement OCaml-style recursive list algorithms in Rust (fold, map, filter) to compare idioms.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default list | `list` — singly-linked, immutable, GC-managed | `Vec<T>` — contiguous array, owned |
| Cons cell | `1 :: rest` (no allocation visible) | `Cons(1, Box::new(rest))` (explicit heap) |
| Recursive definition | Direct: `type 'a t = Nil \| Cons of 'a * 'a t` | Requires `Box`: `Cons(T, Box<List<T>>)` |
| Linked list stdlib | `List` module (core) | `std::collections::LinkedList` (rarely used) |
