📖 **[View on hightechmind.io →](https://hightechmind.io/rust/409-drop-trait)**

---

# 409: Drop Trait and RAII

**Difficulty:** 2  **Level:** Intermediate

Run custom cleanup code when a value goes out of scope — guaranteed by the borrow checker to run exactly once.

## The Problem This Solves

Resources — file handles, network sockets, database connections, locks, memory allocations — must be released when you're done with them. In languages without RAII, you must remember to call `close()`, `free()`, `unlock()` everywhere, including on every error path. Miss one, and you have a resource leak or a deadlock.

Rust's `Drop` trait solves this at the language level. When a value goes out of scope — normally, via `return`, via `panic`, or via explicit `std::mem::drop()` — the compiler calls `drop()` automatically. There's no way to forget: the borrow checker tracks ownership and guarantees cleanup runs exactly once. This is **RAII** (Resource Acquisition Is Initialization): acquire the resource in the constructor, release it in `Drop`.

The standard library is built on this. `MutexGuard` unlocks the mutex when dropped. `File` closes the file descriptor when dropped. `Box<T>` frees the heap allocation when dropped. You get the same power for your own resource types.

## The Intuition

`Drop` is a single-method trait: `fn drop(&mut self)`. The compiler calls it automatically when the value's lifetime ends. You cannot call `drop()` directly (the compiler prevents double-drop). For early cleanup, use `std::mem::drop(value)` which moves the value into a function that immediately drops it.

Drop order is deterministic: in a scope, values are dropped in **reverse declaration order** (last declared, first dropped — like a stack). Fields in a struct are dropped in **declaration order**. This is guaranteed and stable.

## How It Works in Rust

```rust
struct FileHandle {
    name: String,
    is_open: bool,
}

impl FileHandle {
    fn open(name: &str) -> Self {
        println!("Opening: {}", name);
        FileHandle { name: name.to_string(), is_open: true }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if self.is_open {
            println!("Closing: {}", self.name); // automatic cleanup
            self.is_open = false;
        }
    }
}

// RAII lock guard: lock acquired on creation, released on drop
struct LockGuard<'a> {
    resource: &'a str,
}

impl<'a> LockGuard<'a> {
    fn acquire(resource: &'a str) -> Self {
        println!("Lock acquired: {}", resource);
        LockGuard { resource }
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        println!("Lock released: {}", self.resource);
    }
}

fn main() {
    // Scope-based cleanup
    {
        let f = FileHandle::open("data.txt");
        println!("Using file...");
    }  // drop() called here — "Closing: data.txt"

    // Drop order: reversed declaration order
    {
        let _a = FileHandle::open("a.txt");
        let _b = FileHandle::open("b.txt");
        let _c = FileHandle::open("c.txt");
    }  // Drops: c, b, a (reverse order)

    // Lock guard: always released, even on panic
    {
        let _guard = LockGuard::acquire("database");
        println!("Doing database work...");
        // panicking here would still release the lock
    }  // "Lock released: database"

    // Explicit early drop
    let f2 = FileHandle::open("early.txt");
    println!("Using...");
    std::mem::drop(f2);  // explicit drop before end of scope
    println!("After explicit drop");
}
```

A `Timer` using RAII for profiling:
```rust
use std::time::Instant;
struct Timer { name: String, start: Instant }
impl Timer {
    fn new(name: &str) -> Self { Timer { name: name.to_string(), start: Instant::now() } }
}
impl Drop for Timer {
    fn drop(&mut self) {
        println!("'{}' took {:?}", self.name, self.start.elapsed());
    }
}

fn expensive_function() {
    let _t = Timer::new("expensive_function"); // starts here
    // ... work ...
} // logs elapsed time here, even if function panics
```

## What This Unlocks

- **Automatic resource management** — open files, connections, and locks in constructors; forget about closing them; `Drop` handles it on every exit path including panics.
- **Profiling and instrumentation** — `Timer`, `Span`, and `Guard` types that measure time or record events without any manual start/stop bookkeeping.
- **Safe FFI** — wrap C resources in Rust structs with `Drop` impls that call the C cleanup function; the borrow checker prevents use-after-free and double-free.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Cleanup | `with_file name f` — explicitly pass a callback; cleanup in `finally` | `Drop` trait — cleanup auto-runs when value leaves scope |
| Error safety | `Fun.protect ~finally:cleanup f` — handles exceptions | `Drop` runs on panic too — no special handling needed |
| Explicit early cleanup | `close_file fh` — manual call | `std::mem::drop(value)` — ownership moved and dropped |
| Ordering | Function scope / `try...finally` | Deterministic reverse-declaration order — specified by the language |
