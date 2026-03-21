📖 **[View on hightechmind.io →](https://hightechmind.io/rust/551-lifetime-rc-weak)**

---

# Rc and Weak for Shared Ownership
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Tree and graph structures require nodes to reference each other, but reference cycles prevent reference-counted memory from being freed. A tree node holding a strong `Rc` reference to its parent and the parent holding a strong `Rc` to its children creates a cycle — the counts never reach zero. The solution: break cycles with `Weak<T>` — a non-owning reference that does not prevent deallocation. `Rc::downgrade` creates a `Weak`; `weak.upgrade()` returns `Option<Rc<T>>` — `None` if the target has been freed. This pattern is fundamental in GUI widget trees, DOM implementations, and doubly-linked lists.

## Learning Outcomes

- How `Rc<T>` provides shared ownership with reference counting in single-threaded code
- Why `Weak<T>` breaks cycles: it does not increment the strong reference count
- How `Rc::downgrade(parent)` and `weak.upgrade()` work together
- How `Rc<RefCell<T>>` combines shared ownership with interior mutability for tree nodes
- Where this pattern appears: GUI trees, DOM, doubly-linked lists, observer patterns

## Rust Application

`Node` holds `parent: RefCell<Weak<Node>>` and `children: RefCell<Vec<Rc<Node>>>`. `add_child` calls `Rc::downgrade(parent)` to store a weak parent reference. `parent()` calls `weak.upgrade()` — returning `None` if the parent was dropped. Since `Weak` does not increment the strong count, dropping the root node correctly frees the tree even with parent pointers. `Rc::new(Node { ... })` creates reference-counted nodes; `RefCell` provides interior mutability for the child list and parent pointer.

Key patterns:
- `Rc::downgrade(&rc)` — create `Weak<T>`, non-owning pointer
- `weak.upgrade()` — `Option<Rc<T>>`, `None` if dropped
- `RefCell<Weak<Node>>` — interior mutable weak reference

## OCaml Approach

OCaml's GC handles cycles automatically — no weak references needed for simple tree structures. The GC can collect cycles of `ref`-connected values. Weak references exist in OCaml (`Weak` module) for cache-like use cases where you want GC to collect entries:

```ocaml
type 'a node = { value: 'a; parent: 'a node option ref; children: 'a node list ref }
(* Cycles are handled by GC — no Weak needed for correctness *)
```

## Key Differences

1. **Cycle handling**: Rust `Rc` cannot collect cycles — `Weak` breaks them; OCaml's tracing GC collects cycles of regular references without special handling.
2. **Upgrade cost**: `weak.upgrade()` is an atomic compare-and-increment; OCaml `Weak.get` checks liveness via the GC; both are O(1) but with different overhead.
3. **Explicit cycle breaking**: Rust programs must consciously choose which direction of a bidirectional relationship uses `Weak`; OCaml programs can use strong references in both directions.
4. **Arc vs Rc**: For multi-threaded code, Rust uses `Arc<Mutex<T>>` with `Arc::downgrade`; OCaml uses `Mutex.t` and GC-managed values in OCaml 5.x domains.

## Exercises

1. **Graph with cycles**: Implement a directed graph using `HashMap<usize, Rc<RefCell<GraphNode>>>` where each node holds `Vec<Weak<RefCell<GraphNode>>>` edges to prevent retain cycles.
2. **Observer cleanup**: Build an observable value using `Vec<Weak<dyn Fn(&T)>>` for listeners — when the listener is dropped, its `Weak` returns `None` and is automatically removed from the list.
3. **Drop order verification**: Create a parent node and two children, then drop the parent first — verify (using a `Drop` impl with `println!`) that the children are also dropped despite the parent backpointer.
