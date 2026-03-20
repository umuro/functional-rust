📖 **[View on hightechmind.io →](https://hightechmind.io/rust/108-rc-shared)**

---

# Example 108: Rc\<T\> — Shared Ownership

**Difficulty:** ⭐⭐
**Category:** Memory Management | Smart Pointers
**OCaml Source:** Real World OCaml — Garbage Collector & Value Representation

## Problem Statement

Implement data structures (a binary tree and a shared-tail cons list) where multiple owners reference the same node, using `Rc<T>` for explicit, single-threaded reference counting.

## Learning Outcomes

- How `Rc<T>` provides multiple ownership without a garbage collector
- `Rc::clone` increments a reference count cheaply — no heap allocation
- `Rc::strong_count` lets you observe liveness at runtime
- Values are freed automatically when the last `Rc` drops — deterministic, not GC

## OCaml Approach

OCaml's GC manages all heap values implicitly. Any binding to a value increments an internal reference count (or marks it live for the major GC). Sharing is the default: `let shared = ...` followed by two uses is simply two pointers to the same node — no special syntax needed. Liveness is invisible to the programmer.

## Rust Approach

Rust's default ownership model gives each value exactly one owner and drops it at end of scope. `Rc<T>` opts into shared ownership: `Rc::clone` returns a new handle that shares the allocation. When the last handle is dropped, the inner value is freed. This is single-threaded only — use `Arc<T>` for multi-threaded sharing.

## Key Differences

1. **Sharing by default vs opt-in:** OCaml shares all heap values automatically; Rust requires explicit `Rc::new` and `Rc::clone`.
2. **Visibility:** OCaml's reference count is invisible; Rust exposes `Rc::strong_count` for inspection and reasoning.
3. **Drop timing:** OCaml's GC may defer collection; Rust's `Rc` drops deterministically the moment the last owner goes out of scope.
4. **Thread safety:** OCaml's GC handles concurrent access; Rust's `Rc` is intentionally `!Send` — use `Arc` across threads.

## Exercises

1. Build a simple directed graph using `Rc<RefCell<Node>>` where each node holds a label and a list of neighbor references, then implement a DFS traversal.
2. Implement a basic observer pattern using `Rc<dyn Fn(Event)>` callbacks: register multiple observers on an event source and fire them all when an event occurs.
3. Demonstrate the `Rc` cycle problem by creating two nodes that reference each other, confirm the memory leak using a `Drop` impl, then fix it using `Weak<T>`.
