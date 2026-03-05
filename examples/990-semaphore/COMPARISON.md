# Semaphore — Comparison

## Core Insight
A semaphore is a generalized mutex: where a mutex allows 1 concurrent holder, a semaphore allows N. Both OCaml and Rust implement it the same way — an integer protected by a mutex, with threads waiting on a condition variable when the count hits zero.

## OCaml Approach
- `count: int` protected by `Mutex.t` + `Condition.t`
- `acquire`: lock, wait while `count = 0`, decrement, unlock
- `release`: lock, increment, signal, unlock
- `with_semaphore` bracket pattern for exception safety
- No built-in semaphore in OCaml's stdlib — always custom

## Rust Approach
- `Mutex<usize>` for count, `Condvar` for waiting
- `acquire`: lock guard, `while *count == 0 { count = cond.wait(count) }`, decrement
- `release`: lock, increment, `notify_one()`
- `with_permit(f)` RAII-style wrapper
- External crates (tokio, parking_lot) provide optimized async semaphores

## Comparison Table

| Concept          | OCaml                              | Rust                               |
|------------------|------------------------------------|------------------------------------|
| Count storage    | `mutable count: int`               | `Mutex<usize>`                     |
| Wait mechanism   | `Condition.wait cond m`            | `cond.wait(guard).unwrap()`        |
| Signal waiter    | `Condition.signal cond`            | `cond.notify_one()`                |
| Bracket acquire  | `with_semaphore sem f`             | `sem.with_permit(f)`               |
| Binary mode      | `make_semaphore 1`                 | `Semaphore::new(1)`                |
| Built into stdlib| No                                 | No (use parking_lot or tokio)      |
| Overflow guard   | `if count < max_count`             | `if *count < self.max`             |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
