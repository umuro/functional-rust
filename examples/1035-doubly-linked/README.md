📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1035-doubly-linked)**

---

# 1035-doubly-linked — Doubly-Linked List
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Doubly-linked lists support O(1) insertion and removal at any node when you already hold a reference to that node. This makes them ideal for LRU cache eviction (move a node to the front when accessed), editor cursor movement, and undo/redo history. The challenge in Rust is that each node needs a pointer to both its predecessor and successor, creating a cycle of shared ownership that cannot be expressed with simple `Box<T>`.

The standard safe solution uses `Rc<RefCell<Node<T>>>` — reference counting for shared ownership and interior mutability for the back-pointer updates.

## Learning Outcomes

- Use `Rc<RefCell<Node<T>>>` as the node link type for shared mutable ownership
- Understand why doubly-linked lists require shared ownership (both neighbors own the node)
- Implement push_back, push_front, pop_back, pop_front, and iteration
- Understand the memory overhead of `Rc<RefCell<_>>` versus raw pointers
- Know when to use `Arc<Mutex<_>>` instead for thread-safe variants

## Rust Application

`src/lib.rs` uses `type Link<T> = Option<Rc<RefCell<DNode<T>>>>`. Each `DNode` holds `prev` and `next` as `Link<T>`. `push_back` creates a new node, links it to the old tail, and updates the tail pointer. The `RefCell` is needed because updating `prev` and `next` requires mutating nodes accessed through shared `Rc` references.

This is the pattern behind `std::collections::LinkedList` (which uses raw pointers internally) and the LRU cache crate.

## OCaml Approach

OCaml's mutable doubly-linked list uses `ref` for the pointers:

```ocaml
type 'a node = {
  mutable value: 'a;
  mutable prev: 'a node option;
  mutable next: 'a node option;
}
```

OCaml's GC handles cycles — a doubly-linked list forms a reference cycle but the GC's cycle collector reclaims it. Rust's `Rc<RefCell<_>>` creates reference cycles that cause memory leaks unless you use `Weak<T>` for back-pointers.

## Key Differences

1. **Cycle handling**: OCaml's GC handles reference cycles automatically; Rust's `Rc` cycles leak unless `Weak<T>` is used for back-pointers.
2. **Interior mutability**: Rust needs `RefCell` to mutate nodes through shared `Rc` references; OCaml uses `mutable` record fields directly.
3. **Thread safety**: Rust can upgrade to `Arc<Mutex<_>>` for thread-safe doubly-linked lists; OCaml uses `Mutex` explicitly.
4. **Overhead**: Rust's `Rc<RefCell<_>>` adds two heap allocations and reference counting per node; OCaml has one heap allocation per node.

## Exercises

1. Add `Weak<RefCell<DNode<T>>>` back-pointers instead of `Rc` for the `prev` link to prevent memory leaks in cyclic structures.
2. Implement an LRU cache using the doubly-linked list with a `HashMap<K, Rc<RefCell<DNode<(K, V)>>>>` for O(1) node lookup.
3. Write an iterator that traverses the list in reverse order using the `prev` pointers.
