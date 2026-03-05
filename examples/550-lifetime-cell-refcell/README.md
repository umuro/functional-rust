# 550: Interior Mutability — Cell and RefCell

**Difficulty:** 3  **Level:** Intermediate

Mutate data through a shared reference by moving borrow checks to runtime.

## The Problem This Solves

Rust's normal rules say: you can have many shared `&T` references or one `&mut T`, but not both. This is safe and efficient, but sometimes it's too restrictive. A stats counter embedded in a struct that's shared across many places shouldn't require `&mut self` just to increment a field. A tree node needs to store children that can be updated even when the tree is accessed through shared references.

`Cell<T>` and `RefCell<T>` are the standard solution: they move the borrow check from compile time to runtime (or eliminate it entirely for `Copy` types), letting you mutate through `&self`. The trade-off is explicit: `Cell` has no borrow overhead but only works for `Copy` types, while `RefCell` tracks borrows at runtime and panics (or returns `Err`) if you violate the rules.

This is the foundation for `Rc<RefCell<T>>` — the idiomatic pattern for shared-ownership mutable graphs, trees, and event systems in single-threaded Rust.

## The Intuition

`Cell<T>` is like a lockbox for `Copy` types. You can call `.get()` to copy out the value and `.set()` to replace it, all through a shared reference. No borrow is created — just a copy. It's zero overhead.

`RefCell<T>` is like a runtime borrow checker you carry around. `.borrow()` returns a smart pointer that acts like `&T` and increments a counter. `.borrow_mut()` does the same for `&mut T`. If you try to get a mutable borrow while a shared one exists, it panics. The rule is the same as the compile-time rule — it's just enforced at runtime instead.

## How It Works in Rust

1. **`Cell<T>` for `Copy` types** — `cell.get()` copies the value out, `cell.set(v)` writes a new value in; all through `&self`, zero runtime overhead.
2. **`RefCell<T>` borrows** — `data.borrow()` returns a `Ref<T>` (acts like `&T`); `data.borrow_mut()` returns a `RefMut<T>` (acts like `&mut T`); both are dropped normally.
3. **Runtime enforcement** — two simultaneous `borrow_mut()` calls panic; use `try_borrow_mut()` for non-panicking fallible access.
4. **`Rc<RefCell<T>>`** — combine for shared ownership *plus* interior mutability; standard pattern for tree nodes, graph adjacency, observer lists.
5. **Single-threaded only** — `Cell` and `RefCell` are `!Sync`; for multi-threaded use `Mutex<T>` or `RwLock<T>`.

## What This Unlocks

- Embed counters, caches, and lazy fields in structs that are accessed through `&self` — no `&mut self` needed.
- Build mutable trees and graphs with `Rc<RefCell<T>>` without fighting the borrow checker.
- Bridge the gap between Rust's strict compile-time rules and dynamic data structure requirements.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared mutable state | `ref` and mutable record fields; GC-backed | Requires `Cell` or `RefCell`; explicit interior mutability |
| Borrow checking | None; runtime errors for aliasing | Compile-time by default; `RefCell` moves check to runtime |
| Thread safety | GC handles it | `Cell`/`RefCell` are `!Sync`; use `Mutex` for threads |
| Shared ownership + mutation | Mutable records shared freely | `Rc<RefCell<T>>` — explicit, composable, single-threaded |
