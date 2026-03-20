📖 **[View on hightechmind.io →](https://hightechmind.io/rust/923-thread-pool)**

---

# 923-thread-pool — Thread Pool

## Problem Statement

Spawning a new OS thread for each task is expensive: thread creation costs ~10-100 microseconds and each thread consumes stack memory. For workloads with many short tasks, the thread creation overhead dominates. A thread pool pre-creates N worker threads and reuses them for many tasks. Work is submitted to a queue; idle workers pick up and execute tasks. This is the foundation of most concurrent runtime systems: Java's `Executors`, Python's `concurrent.futures.ThreadPoolExecutor`, .NET's `ThreadPool`, and Rust's `rayon`. This example shows the manual implementation.

## Learning Outcomes

- Implement a fixed-size thread pool using `Arc<Mutex<Receiver<Task>>>` for work stealing
- Understand `Box<dyn FnOnce() + Send>` as the type-erased task representation
- Use the Drop trait to cleanly shut down workers when the pool is dropped
- Recognize how a shared receiver with mutex enables fair work distribution
- Compare with OCaml's `Thread` module and `Domain` for work pools

## Rust Application

`Task = Box<dyn FnOnce() + Send + 'static>` is the type-erased work unit. `ThreadPool::new(n)` creates one `mpsc::channel` and wraps the receiver in `Arc<Mutex<>>`. Each worker thread loops: `lock.recv()` blocks until a task arrives, executes it, then loops again. `execute(&self, f: impl FnOnce() + Send + 'static)` sends a task. The `Drop` implementation should signal workers to exit — the channel closure terminates all blocked `.recv()` calls.

## OCaml Approach

OCaml's `Thread` module: `Thread.create f x` spawns; `Thread.join t` waits. A pool: create N threads sharing a `Queue.t` protected by `Mutex`. OCaml 5 `Domain.spawn` enables parallel execution on multi-core. The `Thread_pool` library on opam provides a production-quality implementation. OCaml's `Lwt` and `Eio` runtimes implement work-stealing pools internally for their task schedulers. The main OCaml advantage: green threads in Lwt/Eio are cheaper than OS threads.

## Key Differences

1. **Type-erased tasks**: Rust's `Box<dyn FnOnce() + Send>` is the idiomatic task type; OCaml uses `unit -> unit` functions with explicit type casting.
2. **Shared receiver**: Rust wraps `Receiver` in `Arc<Mutex>` for fair multi-worker distribution; OCaml uses `Queue.t + Mutex + Condition` explicitly.
3. **Drop-based shutdown**: Rust's `Drop` on the pool gracefully shuts down via channel closure; OCaml requires explicit `Thread.join` coordination.
4. **rayon alternative**: For CPU-bound parallel work, Rust's `rayon` crate provides a work-stealing thread pool as a library; OCaml's `parallel` library provides similar functionality.

## Exercises

1. Add a `submit_with_result<T: Send>(f: impl FnOnce() -> T) -> impl Future<Output=T>` method using channels.
2. Implement a priority thread pool where high-priority tasks are executed before low-priority ones.
3. Add metrics: track total tasks submitted, completed, and average wait time using atomic counters.
