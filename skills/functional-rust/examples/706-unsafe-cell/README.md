# 706: UnsafeCell — The Foundation of Interior Mutability

**Difficulty:** 4  **Level:** Expert

The single primitive that makes mutation through a shared reference sound — and the basis of `Cell`, `RefCell`, and `Mutex`.

## The Problem This Solves

Rust's aliasing rules are absolute: if you hold a `&T`, the compiler assumes the data behind that reference cannot change. This assumption lets the compiler cache values in registers, reorder loads, and eliminate redundant reads. It also means you *cannot* mutate through a `&T` — at least not without telling the compiler the assumption no longer holds.

`UnsafeCell<T>` is the only way to opt out. It is the designated "I need to mutate through a shared reference" primitive, and every interior-mutability type in the standard library is built on top of it: `Cell<T>` wraps `UnsafeCell<T>`, `RefCell<T>` does too, and so does `Mutex<T>` (via `UnsafeCell` inside the OS synchronisation primitive). There is no other mechanism — if you find yourself doing interior mutability without `UnsafeCell`, you are invoking undefined behaviour.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

`UnsafeCell<T>` is a signal to the compiler: "Do not assume this memory is frozen just because there's a shared reference to it." The compiler inserts an appropriate memory barrier and stops caching. You are responsible for ensuring that the actual mutations are safe — no concurrent writes, or synchronisation in place if there are.

Because `UnsafeCell<T>` is `!Sync`, any type wrapping it directly is also `!Sync` by default. This prevents sharing across threads without an explicit synchronisation wrapper. `Mutex` restores `Sync` by wrapping the `UnsafeCell` with a lock.

## How It Works in Rust

```rust
use std::cell::UnsafeCell;

pub struct MyCell<T> {
    inner: UnsafeCell<T>,
}

// UnsafeCell<T>: !Sync → MyCell<T>: !Sync (cannot share across threads)

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        Self { inner: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        unsafe {
            // SAFETY: We are single-threaded (MyCell: !Sync).
            // UnsafeCell opts out of the shared-reference aliasing rule,
            // making mutation through &self sound.
            *self.inner.get() = value;
        }
    }

    pub fn get_copy(&self) -> T where T: Copy {
        unsafe {
            // SAFETY: Same single-threaded guarantee.
            *self.inner.get()
        }
    }
}

// A counter that increments through a shared reference — impossible without UnsafeCell.
struct Counter { value: MyCell<u64> }
impl Counter {
    fn new() -> Self { Counter { value: MyCell::new(0) } }
    fn inc(&self) { self.value.set(self.value.get_copy() + 1); }  // &self, not &mut self
}
```

The critical invariant you must maintain: never create two `&mut` references to the interior simultaneously. `Cell<T>` enforces this by offering only value-level get/set (no reference to the interior). `RefCell<T>` enforces it with runtime borrow tracking.

## What This Unlocks

- **`Cell<T>` and `RefCell<T>`** — safe single-threaded interior mutability for GUI frameworks, recursive data structures, and callback-driven code.
- **`Mutex<T>` and `RwLock<T>`** — multi-threaded interior mutability where the OS lock replaces the single-threaded invariant.
- **Custom lock-free structures** — wrap atomics or platform-specific synchronisation primitives with `UnsafeCell` to build bespoke concurrent types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable shared state | `ref` / mutable record fields (always allowed) | Requires `UnsafeCell` to be sound |
| Single-threaded mutation | `let x = ref 0` | `Cell<T>` / `RefCell<T>` (both wrap `UnsafeCell`) |
| Thread-safe mutation | `Mutex.create ()` | `Mutex<T>` (contains `UnsafeCell` + OS primitive) |
| Raw interior mutability | `Obj.set_field` (dangerous) | `UnsafeCell::get()` in `unsafe` |
| Compiler aliasing assumption | No optimization barrier | `UnsafeCell` disables the "frozen behind &T" optimization |
