📖 **[View on hightechmind.io →](https://hightechmind.io/rust/920-buffered-stream)**

---

# 920-buffered-stream — Buffered Stream
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When processing a stream of items where each item requires significant I/O or computation, processing them one at a time creates a bottleneck. Buffered or concurrent processing allows N items to be in-flight simultaneously, keeping the processing pipeline saturated. This pattern appears in web crawlers (N concurrent HTTP requests), image processing pipelines (N images being resized in parallel), and database batch operations (N rows being processed simultaneously). The key challenge is bounding concurrency — allowing N concurrent operations without spawning unbounded threads. This example uses a semaphore to implement bounded concurrency.

## Learning Outcomes

- Implement bounded concurrent processing using a counting semaphore
- Use `Arc<Mutex<Vec<(usize, U)>>>` for thread-safe result collection with ordering
- Understand how semaphore acquire/release controls concurrency level
- Use `thread::spawn` + `join` for parallel map with bounded parallelism
- Compare with OCaml's `Lwt_pool` for bounded concurrent processing

## Rust Application

`Semaphore` wraps `Mutex<usize> + Condvar` — acquire blocks when count is 0, release increments and notifies. `buffered_map` spawns one thread per item but limits concurrent execution via `sem.acquire()` before work and `sem.release()` after. Results are collected into `Arc<Mutex<Vec<(usize, U)>>>` with the original index to allow sorted reconstruction. Threads spawn eagerly but wait on the semaphore for execution — this is the "buffered" part. The test verifies order preservation after semaphore-bounded parallel execution.

## OCaml Approach

OCaml's `Lwt` library uses `Lwt_pool.create n f` for bounded resource pools. `Lwt_pool.use pool (fun resource -> ...)` acquires a slot, runs the task, releases. For `async`/`await`: `Eio.Pool.run pool task`. OCaml's `Thread` module (and later `Domain` in OCaml 5) can implement similar patterns with `Mutex` and `Condition` primitives. The OCaml concurrent ecosystem is richer than the std primitives allow, while Rust's std library is intentionally minimal for concurrency.

## Key Differences

1. **Semaphore in std**: Rust std has no built-in `Semaphore` — must be built from `Mutex + Condvar`; OCaml's `Lwt_pool` is a higher-level abstraction.
2. **Result ordering**: Both need to track original indices for ordered output from parallel processing.
3. **Thread vs lightweight**: Rust uses OS threads (expensive per thread); OCaml's `Lwt` uses cooperative green threads (lighter per task).
4. **Bounded vs unbounded**: The semaphore pattern bounds OS thread execution; Rust's `rayon` or `tokio` provide more ergonomic bounded parallelism.

## Exercises

1. Refactor `buffered_map` to use `std::sync::mpsc` channels instead of `Arc<Mutex<Vec>>` for result collection.
2. Implement `buffered_filter_map<T, U, F>(items: Vec<T>, n: usize, f: F) -> Vec<U>` that discards None results.
3. Add a timeout to each task: if a task takes longer than a specified duration, return a default value instead.
