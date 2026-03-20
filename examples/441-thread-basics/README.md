📖 **[View on hightechmind.io →](https://hightechmind.io/rust/441-thread-basics)**

---

# 441: Thread Basics — Spawn and Join
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Modern CPUs have multiple cores that sit idle when code runs single-threaded. `std::thread::spawn` creates OS threads that run truly in parallel on separate cores. Unlike async tasks (which are lightweight but still single-core unless you use an async runtime with a thread pool), OS threads run independently and can leverage all available cores for CPU-bound work. The challenge is safely sharing data between threads — Rust's type system enforces this at compile time via `Send` and `Sync` bounds.

Thread spawning is the foundation of parallel computing in Rust: used in build tools, data processing pipelines, game engines, scientific computing, and any CPU-bound workload.

## Learning Outcomes

- Understand how `thread::spawn` creates OS threads with `move` closures
- Learn how `JoinHandle::join()` waits for thread completion and propagates panics
- See how `T: Send + 'static` bounds enforce safe thread-boundary crossing
- Understand the parallel_compute pattern: map work items to threads, collect results
- Learn how thread panics are handled via `Result<T, Box<dyn Any + Send>>`

## Rust Application

In `src/lib.rs`, `parallel_compute` maps each item to a thread using `thread::spawn(move || f(item))`. The `F: Clone` bound enables cloning the function for each thread. All threads are joined in order, collecting results into a `Vec`. The `T: Send + 'static` bounds on the function signature are compile-time guarantees that the data can safely cross thread boundaries. `spawn_and_join` shows the simpler single-thread pattern.

## OCaml Approach

OCaml 4.x uses `Thread.create f arg` to spawn threads, but the GIL limits true parallelism to I/O-bound work. OCaml 5.x introduces `Domain.spawn` for true parallel domains without GIL. `Thread.join t` waits for completion. Unlike Rust, OCaml has no compile-time `Send` checking — any value can cross thread boundaries, and data races are possible in OCaml 5.x without explicit synchronization.

## Key Differences

1. **GIL**: OCaml 4.x threads share a GIL preventing parallel execution; Rust threads run truly in parallel for both CPU and I/O bound work.
2. **Type safety**: Rust's `Send + 'static` bounds prevent data races at compile time; OCaml threads can share any value without type-system enforcement.
3. **Panic handling**: Rust's `JoinHandle::join()` returns `Result` — panics are caught and returned; OCaml's `Thread.create` propagates exceptions differently.
4. **Performance**: Rust OS threads match C pthreads in performance; OCaml 5.x domains are slightly heavier than pthreads.

## Exercises

1. **Parallel sort**: Implement parallel merge sort using `thread::spawn`. Split the array in half, sort each half in a separate thread, then merge. Verify it produces the same result as `sort()` and benchmark the speedup on 10M elements.
2. **Thread pool manual**: Without using a crate, build a simple `ThreadPool` with N threads that processes jobs from a `Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>`. Verify it processes all jobs.
3. **Panic recovery**: Spawn 10 threads where some randomly panic. Use `JoinHandle::join()` to collect both successful results and panics, returning a `Vec<Result<T, String>>` where errors show the panic message.
