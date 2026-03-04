# 347: Blocking in Async

**Difficulty:** 3  **Level:** Advanced

CPU-intensive or blocking I/O work must be offloaded — never block the async executor thread.

## The Problem This Solves

Async executors like tokio run many tasks on a small thread pool. If one task blocks a thread — calling `thread::sleep`, doing a CPU-intensive computation, or calling a synchronous I/O function — that thread can't run any other tasks. With tokio's default 4-thread pool, one blocking call can cut async throughput by 25%. A handful of blocking calls can stall the entire runtime.

The solution: offload blocking work to a separate thread pool. In tokio, `tokio::task::spawn_blocking(|| expensive_work())` submits the closure to a dedicated blocking thread pool (up to 512 threads by default), returning a `JoinHandle` you can `.await`. The async executor thread is free to run other tasks while the blocking work runs on the separate pool.

This example simulates the pattern using `thread::spawn` + `mpsc::channel` — structurally identical to `spawn_blocking`.

## The Intuition

Imagine Node.js: the event loop is single-threaded. Calling `fs.readFileSync` blocks the loop — no other requests can be served. That's why Node has `fs.readFile(callback)` and `worker_threads`. Rust's `spawn_blocking` is the same idea: push the blocking work off the event loop.

In Python asyncio: `await loop.run_in_executor(None, blocking_function)` does the same thing.

The rule: **if a function might take more than 100μs, offload it.**

## How It Works in Rust

```rust
// CPU-intensive work — blocks a thread for significant time
fn cpu_heavy(n: u64) -> u64 {
    (1..=n).fold(0u64, |acc, x| acc.wrapping_add(x.wrapping_mul(x)))
}

// Simulates spawn_blocking: runs on a separate thread, returns a channel
fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static)
    -> mpsc::Receiver<T>
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    rx  // caller awaits (or recv()) this
}

// In real async context, the pattern is:
// let result = tokio::task::spawn_blocking(|| cpu_heavy(1_000_000)).await?;

// Multiple concurrent blocking tasks — run in parallel, not sequentially
let handles: Vec<_> = vec![
    spawn_blocking(|| cpu_heavy(1_000_000)),
    spawn_blocking(|| cpu_heavy(500_000)),
    spawn_blocking(|| cpu_heavy(750_000)),
];
// All three run concurrently on separate threads
let results: Vec<u64> = handles.into_iter().map(|rx| rx.recv().unwrap()).collect();
```

The test `concurrent_blocking_faster` verifies that 4 tasks with 20ms each complete in <70ms total — they run in parallel, not sequentially (which would take 80ms+).

## What This Unlocks

- **File system operations** — `tokio::fs` uses `spawn_blocking` internally; for non-tokio I/O, use it explicitly.
- **Crypto / hashing** — bcrypt, Argon2 password hashing is intentionally slow; always offload.
- **Database queries (synchronous drivers)** — wrap sync DB calls in `spawn_blocking` until you can switch to an async driver.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Blocking offload | `Lwt_preemptive.detach f ()` (runs in background thread) | `tokio::task::spawn_blocking(f)` |
| CPU-bound work | `Thread.create` or `Domain.spawn` (5.x) | `spawn_blocking` — dedicated blocking pool |
| Result retrieval | Thread join | `.await` on `JoinHandle` |
| Thread pool | Manual or `Thread_pool` library | Tokio manages two pools: async + blocking |
