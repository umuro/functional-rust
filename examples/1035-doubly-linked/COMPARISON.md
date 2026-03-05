# Doubly-Linked List — Comparison

## Core Insight
Doubly-linked lists require bidirectional pointers — each node is shared by its neighbors. In Rust, this violates single-ownership rules, requiring `Rc` (shared ownership) + `RefCell` (interior mutability). OCaml either uses mutable records (imperative) or zippers (functional alternative).

## OCaml Approach
- Mutable records: `mutable prev/next` fields with option types
- GC handles cycles — no reference counting needed
- Zipper alternative: `{ left; focus; right }` for functional bidirectional access
- Standard library has no doubly-linked list

## Rust Approach
- `Rc<RefCell<Node<T>>>` — reference-counted cells with runtime borrow checking
- `clone()` on `Rc` creates shared references
- `borrow()` / `borrow_mut()` for access (panics if rules violated at runtime)
- Must be careful to break cycles to avoid memory leaks
- `std::collections::LinkedList` exists but is rarely recommended

## Comparison Table

| Feature | OCaml (mutable) | Rust (`Rc<RefCell>`) |
|---|---|---|
| Shared ownership | GC handles it | `Rc` reference counting |
| Interior mutability | `mutable` keyword | `RefCell` runtime checks |
| Cycle handling | GC collects cycles | Must break manually |
| Borrow checking | None (runtime safe) | Runtime via RefCell |
| Ergonomics | Simple mutation | Verbose `.borrow_mut()` |
| Recommendation | Fine for imperative | Use `Vec`/`VecDeque` instead |
