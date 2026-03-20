📖 **[View on hightechmind.io →](https://hightechmind.io/rust/550-lifetime-cell-refcell)**

---

# Cell and RefCell for Interior Mutability

## Problem Statement

Rust's ownership rules normally require `&mut T` for mutation — impossible when a value is shared via `Rc<T>` or multiple references. Interior mutability provides a controlled escape hatch: types that allow mutation through a shared reference (`&T`). `Cell<T>` works for `Copy` types by get/set semantics. `RefCell<T>` works for non-`Copy` types by moving the borrow check to runtime — it panics on violation rather than failing at compile time. These types are foundational in GUI frameworks, mock objects, memoization, and any structure requiring shared mutable access without a `Mutex`.

## Learning Outcomes

- How `Cell<T>` enables mutation through `&self` for `Copy` types using `get`/`set`
- How `RefCell<T>` enables runtime borrow checking via `borrow()` and `borrow_mut()`
- Why `RefCell` panics on borrow violations that `&mut T` would catch at compile time
- How `Rc<RefCell<T>>` is the standard single-threaded shared mutable container
- Where interior mutability is used: `Rc<RefCell<T>>` trees, caches, test mocks, Gtk/egui widgets

## Rust Application

`Counter` uses `Cell<i32>` — `increment(&self)` mutates through a shared reference by calling `self.value.set(self.value.get() + 1)`. `Cache` uses `RefCell<Vec<String>>` — `add(&self, s)` calls `self.data.borrow_mut().push(s)`, dynamically acquiring a mutable borrow. If two `borrow_mut()` calls were active simultaneously, `RefCell` would panic. `Rc<RefCell<T>>` is shown as the building block for shared ownership with mutation — the pattern for tree nodes, graph vertices, and observer lists in single-threaded code.

Key patterns:
- `Cell<T>: get() + set()` — copy in/out, no references issued
- `RefCell<T>: borrow() -> Ref<T>`, `borrow_mut() -> RefMut<T>` — runtime borrow checks
- `Rc::clone(&shared)` + `RefCell` — shared ownership with interior mutability

## OCaml Approach

OCaml's `ref` and mutable record fields provide interior mutability natively — no wrapper type needed:

```ocaml
type counter = { mutable value: int }
let increment c = c.value <- c.value + 1
let get c = c.value
```

Since OCaml does not track ownership, all values are freely mutable through any reference with no special wrapper.

## Key Differences

1. **Compile-time vs runtime**: Rust's `&mut T` rule is compile-time; `RefCell<T>` moves the same check to runtime (with panic on violation); OCaml has no borrow check — only runtime type safety.
2. **Performance**: `Cell<T>` is zero overhead for `Copy` types; `RefCell<T>` adds a borrow counter; both are cheaper than `Mutex<T>` (which requires OS involvement).
3. **Thread safety**: `Cell` and `RefCell` are `!Send` — single-threaded only; `Mutex` or `RwLock` are the thread-safe equivalents; OCaml's `ref` is accessible from any domain in OCaml 5.x (with race conditions possible).
4. **API style**: `RefCell::borrow()` returns a smart pointer `Ref<T>` that releases the borrow when dropped; OCaml `ref` reading is just `!x` — a simple dereference.

## Exercises

1. **Memoize with Cell**: Implement `struct Memoized<T: Copy> { computed: Cell<Option<T>>, compute: Box<dyn Fn() -> T> }` with a `get()` method that lazily computes and caches the value.
2. **Observer with RefCell**: Build an `Observable<T: Clone>` struct using `RefCell<Vec<Box<dyn Fn(&T)>>>` for the listener list, so listeners can be added through `&self`.
3. **Rc<RefCell<T>> tree**: Implement a simple linked list using `type Node<T> = Rc<RefCell<NodeInner<T>>>; struct NodeInner<T> { value: T, next: Option<Node<T>> }` with append and traverse methods.
