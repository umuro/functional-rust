📖 **[View on hightechmind.io →](https://hightechmind.io/rust/446-thread-pool-pattern)**

---

# 446: Thread Pool Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Spawning a new OS thread for each incoming request is expensive: thread creation takes ~100μs and each thread consumes ~8MB of stack space by default. A thread pool pre-creates N worker threads that loop waiting for jobs, processing each job from a shared queue. The `rayon` crate provides a global thread pool, but understanding the implementation reveals the fundamental pattern: a channel as work queue, `Arc<Mutex<Receiver>>` for shared job pickup, and `JoinHandle`s for graceful shutdown.

Thread pools power web servers (tokio's blocking thread pool), HTTP clients, database connection management, and any system with variable-rate work items.

## Learning Outcomes

- Understand why thread pools outperform per-request thread spawning
- Learn how `Arc<Mutex<Receiver<Job>>>` shares a single channel receiver across workers
- See how `Box<dyn FnOnce() + Send + 'static>` erases job types into a uniform queue entry
- Understand graceful shutdown: dropping the sender closes the channel, workers exit their loops
- Learn how the `Drop` impl on `ThreadPool` ensures clean shutdown

## Rust Application

In `src/lib.rs`, `ThreadPool` stores `Vec<JoinHandle<()>>` and a `Sender<Job>`. Each worker thread runs `loop { rx.lock().unwrap().recv() }` — competing for jobs from the shared `Arc<Mutex<Receiver>>`. When `Drop` is called, the sender is dropped (channel closes), workers receive `Err` from `recv()` and exit. `execute<F: FnOnce() + Send + 'static>` boxes the closure and sends it. The `Option<Sender>` in the struct enables taking ownership during drop.

## OCaml Approach

OCaml's `Domainslib` provides `Task.pool` — a domain pool (OCaml 5.x's parallel unit) for distributing work. `Task.run pool (fun () -> computation)` submits work. For OCaml 4.x threads, `Thread_pool` libraries (like `moonpool`) provide similar functionality. The Lwt and Async libraries have their own thread pool abstractions for offloading blocking work from their event loops.

## Key Differences

1. **Job type**: Rust uses `Box<dyn FnOnce() + Send>` for type-erased closures; OCaml uses `unit -> unit` closures.
2. **Worker count**: Rust thread pools use OS threads; OCaml 5.x's domain pools use parallelism domains.
3. **Graceful shutdown**: Rust uses channel close for shutdown signal; OCaml typically uses a sentinel value or explicit stop flag.
4. **Rayon alternative**: `rayon::ThreadPool` provides work-stealing on top of OS threads for better load balancing than the simple queue approach.

## Exercises

1. **Priority queue**: Replace the `mpsc::channel` with `Arc<Mutex<BinaryHeap<(Priority, Job)>>>`. Support `execute_with_priority(priority: u8, f: impl FnOnce() + Send)` and verify higher-priority jobs execute first.
2. **Worker metrics**: Add per-worker job counters using `Arc<AtomicU64>`. Expose `fn job_counts(&self) -> Vec<u64>` that returns each worker's processed job count. Verify work is reasonably balanced.
3. **Timeout jobs**: Extend `ThreadPool::execute` to accept `execute_with_timeout(timeout: Duration, f: impl FnOnce() + Send)`. Spawn a monitoring thread that kills long-running jobs (simulated by tracking active jobs).
