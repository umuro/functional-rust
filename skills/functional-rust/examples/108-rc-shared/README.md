# 108: Rc\<T\> — Shared Ownership

**Difficulty:** 2  **Level:** Intermediate

`Rc<T>` enables multiple owners for the same value in single-threaded code — Rust's explicit, opt-in equivalent of garbage collection.

## The Problem This Solves

Rust's default ownership model — one owner, automatic drop at end of scope — solves most problems cleanly. But some data structures genuinely need shared ownership: a graph where multiple nodes reference the same edge, a GUI widget tree where parent and child both need a reference to shared state, a cache where the same value is referenced from multiple call sites.

In C, you'd use raw pointers and manage lifetimes manually — tedious and error-prone, leading to double-frees (freeing the same memory twice) and use-after-free bugs. In Python or OCaml, the GC handles this automatically, but you pay the GC overhead for everything, even values that don't need sharing.

Rust gives you `Rc<T>`: explicit, opt-in reference counting. The `Rc` (Reference Counted) smart pointer tracks how many clones exist. When the last one drops, the value is freed. No GC, no manual tracking. You pay for reference counting only when you actually need shared ownership.

## The Intuition

`Rc<T>` is reference counting you explicitly opt into — clone the `Rc` to share ownership, and the value is freed automatically when the last `Rc` to it is dropped.

## How It Works in Rust

```rust
use std::rc::Rc;

fn demo_basic_sharing() {
    let data = Rc::new(String::from("shared value"));
    
    // Clone the Rc — this clones the *pointer*, not the data
    let owner1 = Rc::clone(&data);  // reference count: 2
    let owner2 = Rc::clone(&data);  // reference count: 3
    
    println!("Count: {}", Rc::strong_count(&data)); // 3
    println!("{}", owner1);  // "shared value"
    println!("{}", owner2);  // "shared value"
    
    drop(owner1); // count: 2
    drop(owner2); // count: 1
    // data dropped here: count goes to 0, String is freed
}

// Tree where multiple nodes share a parent
#[derive(Debug)]
enum Tree {
    Leaf(i32),
    Node(i32, Rc<Tree>, Rc<Tree>),
}

fn demo_tree() {
    let shared_leaf = Rc::new(Tree::Leaf(5));
    
    // Both branches share the same leaf — no copying
    let branch1 = Tree::Node(1, Rc::clone(&shared_leaf), Rc::clone(&shared_leaf));
    println!("Shared leaf referenced {} times", Rc::strong_count(&shared_leaf));
}

// Rc<T> is immutable — no mutation through shared ownership
// Use Rc<RefCell<T>> when you need shared mutable ownership
use std::cell::RefCell;
let shared_mutable = Rc::new(RefCell::new(vec![1, 2, 3]));
let clone = Rc::clone(&shared_mutable);
clone.borrow_mut().push(4);  // mutate through shared ownership
println!("{:?}", shared_mutable.borrow()); // [1, 2, 3, 4]
```

## What This Unlocks

- **Shared ownership without GC** — reference-counted sharing only where you need it; the rest of your code uses zero-cost ownership semantics.
- **Explicit sharing** — `Rc::clone` in code review signals "this is shared ownership; be aware of reference cycles."
- **Graph and tree structures** — data structures where multiple nodes reference the same data are expressible safely without raw pointers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared ownership | Automatic (GC manages all values) | Explicit — wrap in `Rc<T>` to opt in |
| Reference counting | Built into GC | Manual via `Rc<T>` |
| Thread safety | GC is thread-safe | `Rc<T>` is single-threaded only; use `Arc<T>` for threads |
| Mutation | Mutable fields freely | `Rc<T>` is immutable; combine with `RefCell<T>` for mutation |
| Reference cycles | GC handles (or weak refs needed) | Can leak memory; use `Weak<T>` to break cycles |
