# 341: Buffered Concurrent Stream Processing

**Difficulty:** 4  **Level:** Expert

Limit concurrent work with a semaphore — process up to N items at once without overwhelming downstream resources.

## The Problem This Solves

When processing a large stream of items — API calls, database writes, file transforms — spawning an unbounded number of threads will exhaust memory, saturate connections, and cause cascading failures. You need a bounded pipeline that keeps N workers busy at all times and blocks new work from starting until a slot opens.

This is the **sliding-window concurrency** pattern. It appears everywhere: web scrapers that must respect rate limits, batch upload clients, parallel test runners, and data pipelines with expensive per-item work.

The naive solution — `collect` all tasks first, then join — either starves workers waiting for the slow ones or blows up memory. A semaphore solves this: it acts as a counter of available slots, automatically refilling as each task completes.

## The Intuition

Think of a bank with exactly N teller windows. Customers (tasks) queue up, and each one waits until a window opens. The moment a teller finishes, the next customer steps forward. You don't need to know how long each transaction takes — the semaphore enforces the limit automatically.

In Rust, a `Semaphore` wraps a `Mutex<usize>` + `Condvar`. Acquiring decrements the counter (blocking at zero); releasing increments it and wakes a waiter.

## How It Works in Rust

1. **Semaphore** — `Mutex<usize>` for the permit count, `Condvar` to park threads waiting for a permit.
2. **Spawn all tasks** — iterate over input with `.enumerate()`, capturing `(index, item)` per thread.
3. **Each thread acquires** before calling `f(item)` and **releases** immediately after — not at drop.
4. **Collect indexed results** into `Arc<Mutex<Vec<(usize, U)>>>`.
5. **Restore order** — sort by index after joining all handles.

The semaphore is shared via `Arc::clone`, the closure via `Arc<F>`. Results vec is also `Arc`-shared. No async runtime needed — pure `std::thread`.

```rust
sem.acquire();       // blocks if N tasks already running
let result = f(item);
sem.release();       // opens a slot for the next waiter
results.lock().unwrap().push((i, result));
```

## What This Unlocks

- **Backpressure without async** — rate-limit concurrent work using only `std` primitives.
- **Ordered output from unordered execution** — index-tagged results survive any completion ordering.
- **Drop-in parallel map** — `buffered_map(items, 4, heavy_fn)` replaces a sequential loop with bounded concurrency.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Concurrency cap | `Semaphore` module / manual counter | `Mutex<usize>` + `Condvar` |
| Result collection | `Array.of_seq` with futures | `Arc<Mutex<Vec<(usize, U)>>>` |
| Ordered output | Future array preserves order | Sort by captured index after join |
| Backpressure | Bounded channel / `Lwt_pool` | Semaphore blocks the spawning thread |
