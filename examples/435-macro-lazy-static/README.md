📖 **[View on hightechmind.io →](https://hightechmind.io/rust/435-macro-lazy-static)**

---

# 435: Lazy Static Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Global state initialization in Rust is tricky: `static` variables require `const` initializers, but many useful values (HashMap, compiled regex, config loaded from env) can only be built at runtime. The `lazy_static!` macro (now largely superseded by `std::sync::OnceLock` in Rust 1.70+) solves this by wrapping initialization in a once-executed closure. `OnceLock<T>` provides thread-safe initialization on first access. This pattern enables global singletons, compiled regex caches, and runtime-initialized configuration that is accessed efficiently after the first call.

`OnceLock`/`lazy_static` patterns appear in compiled regex caches (the `regex` crate recommends this), global configuration, connection pool singletons, and any value that is expensive to initialize and needs global access.

## Learning Outcomes

- Understand why `static mut` is unsafe and why `OnceLock` is the safe alternative
- Learn how `OnceLock::get_or_init` guarantees single initialization across all threads
- See how `thread_local!` provides per-thread storage without synchronization
- Understand the historical `lazy_static!` macro and how `OnceLock` replaces it
- Learn when to use `OnceLock` (global singleton) vs. `Arc<Mutex<T>>` (shared mutable state)

## Rust Application

In `src/lib.rs`, `CONFIG: OnceLock<Config>` is initialized on first call to `Config::global()` using `get_or_init`. The closure runs exactly once, thread-safely. `thread_local!` declares `COUNTER: Cell<u32>` as per-thread storage — each thread has its own counter, so no synchronization is needed. `COUNTER.with(|c| ...)` provides access to the thread-local within a closure.

## OCaml Approach

OCaml uses `Lazy.t` for lazy values: `let config = lazy (make_config ())` where `Lazy.force config` triggers initialization on first access. Thread safety in OCaml 4.x relies on the GIL; OCaml 5.x's `Mutex.t` and `Atomic.t` are needed for true concurrent lazy initialization. `Thread_local.t` provides per-domain storage in OCaml 5.x. The `lazy` keyword is built into the OCaml language, unlike Rust's library-based `OnceLock`.

## Key Differences

1. **Language vs. library**: OCaml's `lazy` is a language keyword; Rust's `OnceLock` is a standard library type.
2. **Thread safety**: `OnceLock` is explicitly designed for concurrent initialization; OCaml's `lazy` requires explicit locking in OCaml 5.x multi-domain programs.
3. **Thread-local**: Rust's `thread_local!` macro uses OS thread-local storage; OCaml 5.x's `Domain.DLS` provides domain-local storage.
4. **Legacy**: The `lazy_static!` crate was widely used before `OnceLock`; OCaml's `Lazy.t` has been stable for decades.

## Exercises

1. **Compiled regex cache**: Use `OnceLock<Regex>` to cache a compiled regex pattern. Implement `fn is_valid_email(s: &str) -> bool` that initializes the regex once and reuses it. Verify with a test that the regex is only compiled once.
2. **Config from env**: Create `AppConfig::global()` using `OnceLock` that reads configuration from environment variables on first call. Include `DATABASE_URL`, `PORT`, and `LOG_LEVEL`. Use `once_cell::sync::Lazy` or `OnceLock` to make the initialization thread-safe.
3. **Per-thread ID**: Use `thread_local!` to assign each thread a unique ID on first access. Spawn 4 threads and verify each has a unique ID by collecting thread IDs from all threads and asserting they are all distinct.
