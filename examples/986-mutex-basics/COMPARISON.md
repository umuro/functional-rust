# Mutex-Protected State — Comparison

## Core Insight
Both languages use mutexes for shared mutable state, but Rust's type system **enforces** correct use: `Mutex<T>` wraps the data itself, so you physically cannot access it without locking. OCaml's `Mutex.t` is a separate object — the programmer must remember to lock/unlock around every access.

## OCaml Approach
- `Mutex.create ()` creates a mutex independent of the data
- `Mutex.lock m` / `Mutex.unlock m` must be called manually — easy to forget
- Exception-unsafe: if `f()` throws, you must catch and unlock (bracket pattern)
- `with_lock` helper function needed for exception safety
- Data lives in `ref` cells separate from the mutex

## Rust Approach
- `Mutex::new(data)` wraps data and mutex together — inseparable
- `mutex.lock().unwrap()` returns a `MutexGuard<T>` — acts like `&mut T`
- Guard unlocks automatically when dropped (RAII) — exception safe
- `Arc<Mutex<T>>` for sharing across threads (atomically ref-counted)
- Compiler prevents accessing data without locking — zero runtime overhead

## Comparison Table

| Concept              | OCaml                            | Rust                              |
|----------------------|----------------------------------|-----------------------------------|
| Create               | `Mutex.create ()` + `ref data`   | `Mutex::new(data)`                |
| Lock                 | `Mutex.lock m`                   | `m.lock().unwrap()`               |
| Unlock               | `Mutex.unlock m` (manual)        | Drop the `MutexGuard` (RAII)      |
| Access data          | Access `ref` directly            | Dereference guard: `*guard`       |
| Share across threads | `ref` in closure                 | `Arc::clone(&mutex)`              |
| Exception safety     | Manual bracket pattern           | Automatic via RAII                |
| Forget to unlock     | Possible (deadlock)              | Impossible — type system prevents |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
