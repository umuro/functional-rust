📖 **[View on hightechmind.io →](https://hightechmind.io/rust/111-refcell-runtime)**

---

# Example 111: RefCell<T> — Runtime Borrow Checking

**Difficulty:** ⭐⭐  
**Category:** Ownership & Borrowing | Interior Mutability  
**OCaml Source:** Conceptual — OCaml mutable refs and mutable record fields

## Problem Statement

Enable mutation through a shared (`&self`) reference by deferring Rust's borrow rules from compile time to runtime, using `RefCell<T>` to enforce "one writer XOR many readers" dynamically.

## Learning Outcomes

- Understand when the borrow checker can't prove safety but the programmer can
- Use `borrow()` / `borrow_mut()` to obtain guarded references at runtime
- Design structs with `&self` mutation (no `&mut self` needed) via `RefCell`
- Use `try_borrow()` / `try_borrow_mut()` for fallible, panic-free access

## OCaml Approach

OCaml has no borrow rules — a `ref` value or `mutable` record field can be read and written freely at any time. The programmer bears full responsibility for correctness. This makes code concise but removes the compile-time safety net that Rust provides.

## Rust Approach

`RefCell<T>` wraps a value and hands out `Ref<T>` (shared) or `RefMut<T>` (exclusive) guard objects. The counts are tracked at runtime; any attempt to hold a mutable borrow alongside any other borrow causes an immediate panic. Sequential borrows — where each guard is dropped before the next is acquired — are always safe.

## Key Differences

1. **Borrow enforcement:** OCaml enforces nothing; Rust enforces "one writer XOR multiple readers" — just at runtime instead of compile time with `RefCell`.
2. **Receiver type:** OCaml `mutable` field methods implicitly allow mutation; Rust requires `&mut self` unless `RefCell` provides interior mutability, enabling `&self` methods.
3. **Failure mode:** OCaml allows races silently; Rust panics immediately on double-mutable-borrow, making bugs loud and reproducible.
4. **Fallible API:** `try_borrow()` / `try_borrow_mut()` let library code handle contention gracefully rather than panicking.

## Exercises

1. Build a simple event bus using `Rc<RefCell<Vec<Box<dyn Fn(&Event)>>>>` that allows subscribers to register closures and the bus to broadcast events to all of them.
2. Implement a mutable graph using `HashMap<NodeId, RefCell<Node>>` where edges can be added at runtime, and write a BFS traversal that borrows each node only when needed.
3. Deliberately trigger a `RefCell` borrow panic by holding a mutable borrow and attempting a second mutable borrow in the same scope; then refactor the code to avoid the panic using split borrows or restructuring.
