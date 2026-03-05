# Barrier Synchronization — Comparison

## Core Insight
A barrier is a **collective synchronization point** — like a countdown latch where N threads each decrement, and all are released when it hits zero. Used in parallel algorithms where phases must complete before the next begins.

## OCaml Approach
- No built-in barrier — simulate with `Mutex` + `Condition` + generation counter
- `generation` counter prevents spurious wakeup confusion across rounds
- `Condition.broadcast` wakes all waiting threads simultaneously
- Reusable: increment `generation` and reset `count` atomically

## Rust Approach
- `std::sync::Barrier::new(n)` — built-in, no boilerplate
- `barrier.wait()` blocks until `n` threads have called it
- Returns `BarrierWaitResult` — `.is_leader()` is true for exactly one thread
- Automatically resets — reusable for multiple rounds
- Thread-safe by design; panic-safe

## Comparison Table

| Concept              | OCaml (simulated)               | Rust                              |
|----------------------|---------------------------------|-----------------------------------|
| Create               | Manual struct with mutex+condvar| `Barrier::new(n)`                 |
| Wait at barrier      | `barrier_wait b`                | `barrier.wait()`                  |
| Leader detection     | Not built-in                    | `result.is_leader()`              |
| Reuse after trigger  | Manual generation counter       | Automatic                         |
| Prevent spurious wake| `while gen = b.generation`      | Handled internally                |
| Wake mechanism       | `Condition.broadcast`           | Internal (implementation-defined) |
| Stdlib               | No                              | Yes (`std::sync::Barrier`)        |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
