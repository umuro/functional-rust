# One-Time Initialization — Comparison

## Core Insight
`Lazy.t` and `OnceLock` both implement **deferred singleton**: compute a value at most once, cache forever. The difference is thread safety — OCaml's `Lazy.t` uses GC for safety; Rust's `OnceLock` uses atomics for lock-free concurrent initialization.

## OCaml Approach
- `lazy expr` wraps an expression; `Lazy.force` evaluates it (once)
- Result is cached — subsequent `Lazy.force` calls return immediately
- Thread-safe in OCaml 5 (domains); not safe across threads in OCaml 4 without wrapping
- Typical use: module-level initialization, expensive config loading
- `Lazy.is_val` checks if already evaluated without forcing

## Rust Approach
- `OnceLock<T>` is in `std::sync` since Rust 1.70
- `get_or_init(f)` runs `f` at most once, returns `&T` thereafter
- `get()` returns `Option<&T>` — `None` if not yet initialized
- Works for `static` globals and instance fields
- `set(v)` for explicit single-write (returns error if already set)
- `LazyLock<T>` (Rust 1.80+) for lock-free lazy static

## Comparison Table

| Concept              | OCaml                          | Rust                              |
|----------------------|--------------------------------|-----------------------------------|
| Declare lazy         | `let x = lazy (expr)`          | `static X: OnceLock<T> = OnceLock::new()` |
| Force / initialize   | `Lazy.force x`                 | `X.get_or_init(\|\| expr)`         |
| Check if ready       | `Lazy.is_val x`                | `X.get().is_some()`               |
| Thread-safe          | OCaml 5 only                   | Yes — std guarantees              |
| Instance level       | `let _ = lazy (...)` in struct | `OnceLock<T>` field               |
| Type annotation      | `'a Lazy.t`                    | `OnceLock<T>`                     |
| Return type          | `'a` (the value)               | `&'static T` (reference)          |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
