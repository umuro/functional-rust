# 489: Arc\<str\> for Shared Strings

**Difficulty:** 2  **Level:** Intermediate

Atomically reference-counted immutable string — share one allocation safely across threads.

## The Problem This Solves

When multiple threads need to read the same string, the options are: clone it into each thread (wastes memory), use `&str` with `'static` (only works for string literals), or reach for synchronisation primitives to guard access (overkill for read-only data).

`Arc<str>` is the clean answer. A single heap allocation holds the string bytes and an atomic reference count. Any number of threads can hold a clone of the `Arc<str>`, read it freely, and the allocation lives until the last clone drops — all without locks.

`Arc<str>` is also more compact than `Arc<String>`: it's a fat pointer (address + length) directly to the string bytes, while `Arc<String>` adds an extra level of indirection through a `String` struct.

## The Intuition

A shared read-only notice board in an office. Every employee (thread) has a laminated copy of the URL to the board (the `Arc`). They can all read it at the same time without asking permission. The board itself is only taken down when every single copy of the URL has been thrown away. No locks, no queuing, just atomic reference counting.

## How It Works in Rust

1. **Create from `&str` or `String`**:
   ```rust
   use std::sync::Arc;
   let shared: Arc<str> = "configuration data".into();
   let from_string: Arc<str> = some_string.into();
   ```
2. **Clone across threads** — `Arc<str>` is `Send + Sync`:
   ```rust
   let handle = {
       let s = shared.clone();
       std::thread::spawn(move || println!("thread sees: {}", s))
   };
   handle.join().unwrap();
   ```
3. **Use all `str` methods** via `Deref`:
   ```rust
   shared.contains("config")
   shared.split(',').collect::<Vec<_>>()
   ```
4. **In data structures** — no lifetime parameters needed:
   ```rust
   struct Config {
       name: Arc<str>,
       value: Arc<str>,
   }
   ```
5. **Compare and hash** — `Arc<str>` implements `PartialEq`, `Eq`, `Hash` by comparing string *contents* (not pointer). For pointer equality use `Arc::ptr_eq`.

## What This Unlocks

- **Zero-copy sharing across threads** — pass configuration, templates, or parsed strings to worker threads without cloning bytes.
- **Smaller than `Arc<String>`** — one less indirection; the string bytes sit directly in the `Arc` allocation.
- **Foundation for string interning** — intern tables return `Arc<str>` for thread-safe shared symbols (see example 487).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread-safe shared string | GC-managed | `Arc<str>` (atomic refcount) |
| vs `Rc<str>` | — | `Rc`: single-thread; `Arc`: multi-thread |
| vs `Arc<String>` | — | `Arc<str>` is thinner (no extra `String` header) |
| Mutability | Immutable | Immutable (use `Arc<Mutex<String>>` for mutable) |
