# Read-Write Lock Pattern — Comparison

## Core Insight
RwLock encodes the **read-write exclusion invariant** in the type: `&T` access (shared) maps to read lock; `&mut T` access (exclusive) maps to write lock. This mirrors Rust's own ownership model.

## OCaml Approach
- No standard RwLock — must simulate with `Mutex` + `Condition` + reader count
- `readers: int ref` tracks active readers; writer waits until `readers = 0`
- `writer_waiting: bool` prevents reader starvation of writers
- More complex than needed — OCaml's GC handles most sharing without locks

## Rust Approach
- `RwLock::new(data)` is standard in `std::sync`
- `rw.read()` → `RwLockReadGuard` — shared, many at once
- `rw.write()` → `RwLockWriteGuard` — exclusive, blocks all others
- `try_read()` / `try_write()` non-blocking variants
- RAII: guards unlock on drop — no manual unlock

## Comparison Table

| Concept              | OCaml (simulated)                 | Rust                              |
|----------------------|-----------------------------------|-----------------------------------|
| Create               | Manual struct + Mutex + Condition | `RwLock::new(data)`               |
| Read lock            | `read_lock` / `read_unlock`       | `rw.read().unwrap()`              |
| Write lock           | `write_lock` / `write_unlock`     | `rw.write().unwrap()`             |
| Multiple readers     | Yes (via reader count)            | Yes — `RwLockReadGuard` is shared |
| Prevent writer starvation | Manual `writer_waiting` flag | Implementation-dependent          |
| Unlock               | Manual call                       | Drop the guard (RAII)             |
| Try-lock             | Not shown (custom needed)         | `try_read()` / `try_write()`      |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
