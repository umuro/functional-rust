# 456: OnceLock — Initialize a Global Exactly Once

**Difficulty:** 3  **Level:** Intermediate

Use `OnceLock<T>` for thread-safe lazy initialization of global state — the safe replacement for `static mut` and the `lazy_static` crate.

## The Problem This Solves

Global state is often necessary: database connection strings, compiled regexes, configuration loaded from environment variables, expensive lookup tables. But initializing them at the `static` declaration (`static CONFIG: Config = init()`) requires a `const` expression — no runtime computation, no `std::env::var`, no file I/O.

The common workaround in unsafe languages is `static mut` with manual initialization: check a flag, if unset, initialize, set the flag. This has an obvious race condition if multiple threads check simultaneously. The traditional Rust fix was the `lazy_static` crate (or its successor `once_cell`). Since Rust 1.70, `OnceLock<T>` is in the standard library and covers the thread-safe case directly.

`OnceLock` guarantees that the initializer closure runs exactly once across all threads, even if multiple threads call `get_or_init` simultaneously. The first thread to arrive runs the closure; others wait and then get the same initialized value. After initialization, reads are lock-free — just a check of an internal flag.

## The Intuition

`OnceLock<T>` is a container that transitions from "empty" to "filled" exactly once. Before filling: reads return `None`; `get_or_init` runs the closure. After filling: reads return `Some(&T)` directly, no locking needed.

In Java: `static` fields with class-loading guarantees (complicated) or double-checked locking (error-prone). In Python: module-level initialization runs once (but has subtleties with threads). In Go: `sync.Once`. In Rust, `OnceLock` is the standard answer — explicit, composable, and correct without understanding JVM class loading rules.

## How It Works in Rust

```rust
use std::sync::OnceLock;
use std::collections::HashMap;

// Global static — OnceLock is the type-safe container
static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

// Function that initializes once and returns reference forever after
fn config() -> &'static HashMap<&'static str, &'static str> {
    CONFIG.get_or_init(|| {
        println!("init config");  // runs exactly once, even with 100 threads
        [("host", "localhost"), ("port", "8080")]
            .iter().cloned().collect()
    })
}

fn main() {
    // All three calls return the same reference — init runs once
    println!("{}", config()["host"]);  // "init config" printed here
    println!("{}", config()["host"]);  // no print — already initialized
    assert!(std::ptr::eq(config(), config())); // literally the same pointer

    // Explicit set (instead of get_or_init)
    static GREETING: OnceLock<String> = OnceLock::new();
    GREETING.set("Hello!".to_string()).ok(); // Ok(()) or Err(value) if already set
    println!("{}", GREETING.get().unwrap());
}
```

For non-global (per-instance) lazy initialization, use `OnceCell<T>` from `std::cell` — same semantics but single-threaded (no `Sync`). This is useful for caching a computed field on a struct.

```rust
use std::cell::OnceCell;

struct Expensive { cache: OnceCell<Vec<u32>>, limit: u32 }
impl Expensive {
    fn computed(&self) -> &[u32] {
        self.cache.get_or_init(|| {
            (2..=self.limit).filter(|&n| (2..n).all(|d| n % d != 0)).collect()
        })
    }
}
```

## What This Unlocks

- **Global configuration** — load config once from environment/file at first use; return a reference for all subsequent calls without any locking overhead.
- **Compiled regexes** — store a `Regex` in a `OnceLock` so the expensive compilation happens once, not per-call.
- **Lazy singleton services** — database pools, HTTP clients, or any resource that should exist once and be shared everywhere.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy value | `lazy (expensive ())` | `OnceLock::new()` with `get_or_init(|| ...)` |
| Force evaluation | `Lazy.force v` | `lock.get_or_init(\|\| compute())` |
| Thread-safe | `Lazy.t` safe in OCaml 5 | `OnceLock<T>` — `SeqCst` guarantees, single init |
| Per-instance | same `Lazy.t` | `OnceCell<T>` from `std::cell` — single-threaded |
| Static global | `let v = lazy (...)` | `static V: OnceLock<T> = OnceLock::new()` |
| Replaces | N/A | `static mut`, `lazy_static` crate, `once_cell` crate |
