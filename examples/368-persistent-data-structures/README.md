📖 **[View on hightechmind.io →](https://hightechmind.io/rust/368-persistent-data-structures)**

---

# 368: Persistent Data Structures
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Mutable data structures destroy history — you can't go back to a previous state without making copies. Persistent (functional/immutable) data structures solve this by sharing unchanged structure between versions. A persistent linked list shares its tail: `cons(x, list)` creates a new list that shares all of `list` without copying. This structural sharing makes undo/redo O(1), version control O(changed-nodes), and functional programming idioms possible. Rust implements persistent structures using `Rc<T>` (single-threaded) or `Arc<T>` (multi-threaded) for reference-counted shared ownership.

## Learning Outcomes

- Implement a persistent linked list as `Rc<PList<T>>` with `Nil` / `Cons(T, Rc<...>)` variants
- Understand that `cons(x, tail)` is O(1) and shares `tail` without copying
- Use `Rc::clone` to create new owners of the same node (reference counting, not deep copy)
- Implement `head` and `tail` as O(1) operations on persistent lists
- Extend to persistent trees where modifications create new paths from root to changed node
- Recognize that all of OCaml's standard data structures are persistent

## Rust Application

```rust
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum PList<T> {
    Nil,
    Cons(T, Rc<PList<T>>),
}

impl<T: Clone> PList<T> {
    pub fn nil() -> Rc<Self> {
        Rc::new(Self::Nil)
    }

    pub fn cons(head: T, tail: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Cons(head, tail)) // O(1): new node, shared tail
    }

    pub fn head(list: &Rc<Self>) -> Option<&T> {
        match list.as_ref() {
            Self::Nil => None,
            Self::Cons(h, _) => Some(h),
        }
    }

    pub fn tail(list: &Rc<Self>) -> Rc<Self> {
        match list.as_ref() {
            Self::Nil => Self::nil(),
            Self::Cons(_, t) => Rc::clone(t), // O(1): share the tail
        }
    }
}

// Multiple lists can share the same tail
let shared_tail = PList::cons(3, PList::nil());
let list_a = PList::cons(1, Rc::clone(&shared_tail)); // [1, 3]
let list_b = PList::cons(2, Rc::clone(&shared_tail)); // [2, 3]
// shared_tail has refcount 3: shared_tail + list_a + list_b
```

`Rc::clone` increments the reference count — no copying. When all owners drop, the `Rc` deallocates. This gives automatic memory management for shared structure without a garbage collector.

## OCaml Approach

All OCaml lists are persistent by default — there's no special wrapper needed:

```ocaml
let nil = []
let cons head tail = head :: tail  (* O(1), shares tail *)
let head = List.hd
let tail = List.tl

(* Multiple lists sharing a tail *)
let shared = [3]
let list_a = 1 :: shared  (* [1; 3] — shares shared *)
let list_b = 2 :: shared  (* [2; 3] — shares shared *)
(* GC handles deallocation when no references remain *)
```

In OCaml, the GC tracks all live references automatically — no `Rc` counter needed. The cons cell (`head :: tail`) is one allocation on the minor heap. Structural sharing is the default behavior for all functional data structures.

## Key Differences

| Aspect | Rust `Rc<PList<T>>` | OCaml `'a list` |
|--------|--------------------|--------------------|
| Reference tracking | `Rc` reference counting | GC tracing |
| Allocation | Explicit `Rc::new(...)` | Implicit on `::` cons |
| Clone cost | O(1) `Rc::clone` (inc refcount) | O(1) (GC handles it) |
| Thread safety | `Rc` is not `Send` (use `Arc`) | GC handles concurrency |
| Cycles | `Rc` cycles leak memory | GC detects cycles |

## Exercises

1. **Persistent stack**: Implement `push(stack, val) -> Rc<PList<T>>` and `pop(stack) -> (Option<T>, Rc<PList<T>>)` using persistent lists; demonstrate multiple stacks sharing common history after a common push sequence.
2. **Path sharing**: Build a binary tree where `update(tree, key, value)` creates a new tree that shares all unchanged subtrees with the original — only the path from root to the updated node is new.
3. **Version history**: Implement a `VersionedList<T>` that stores a `Vec<Rc<PList<T>>>` of all versions; `commit(value)` prepends to the current version and saves it; `checkout(version)` returns that version's state.
