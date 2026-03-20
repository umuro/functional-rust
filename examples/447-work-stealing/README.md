📖 **[View on hightechmind.io →](https://hightechmind.io/rust/447-work-stealing)**

---

# 447: Work Stealing — Load Balancing Across Threads
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A simple thread pool with a single shared queue becomes a contention bottleneck: all threads compete for the same lock to get the next job. Work stealing solves this: each thread has its own local deque. Workers process their own tasks from the front; idle workers steal tasks from the back of busy workers' deques. This locality-aware load balancing was introduced in Cilk (MIT, 1990s) and is the algorithm behind Java's `ForkJoinPool`, `rayon`, Go's goroutine scheduler, and .NET's `Task Parallel Library`.

Work stealing powers `rayon`'s parallel iterators, Tokio's multi-threaded runtime, and scientific computing frameworks needing efficient dynamic load balancing.

## Learning Outcomes

- Understand why per-worker queues reduce contention vs. a single shared queue
- Learn the work-stealing algorithm: pop from own front, steal from others' back
- See how `Arc<Mutex<VecDeque<T>>>` simulates a work-stealing deque
- Understand the performance improvement: O(N) contention → O(1) average contention
- Learn how `rayon` uses deque-based work stealing internally

## Rust Application

In `src/lib.rs`, each worker has its own `WorkQueue<T>` (an `Arc<Mutex<VecDeque<T>>>`). Workers first pop from their own queue; if empty, they iterate other queues attempting to steal. The initial load is placed on worker 0's queue. The theft loop uses `try_lock()` to avoid blocking on busy queues, moving on to the next candidate. This simulates the core work-stealing algorithm without a production-grade deque implementation.

## OCaml Approach

OCaml 5.x's `Domainslib` uses work stealing internally for its task pool. The `Task.pool` creates a domain pool with per-domain queues and stealing. OCaml 4.x's thread-based work doesn't benefit from work stealing due to the GIL. For custom work stealing in OCaml 5.x, the `deque` package provides lock-free Chase-Lev deques — the standard work-stealing data structure.

## Key Differences

1. **Lock-free deques**: Production work stealing uses Chase-Lev lock-free deques; the Rust simulation uses `Arc<Mutex<VecDeque>>` for clarity.
2. **Contention model**: Work stealing reduces contention from O(N) threads competing on one lock to O(1) average steals from N idle workers.
3. **Library vs. manual**: `rayon` provides production work stealing; manual implementation is for educational purposes.
4. **Memory ordering**: Lock-free Chase-Lev deques require careful memory ordering (`SeqCst`/`Acquire`/`Release`); the lock-based version handles this automatically.

## Exercises

1. **Fibonacci with stealing**: Implement parallel Fibonacci using recursive task spawning where each spawn creates a new task on the current worker's queue. Show work stealing in action with logging when steals occur.
2. **Chase-Lev deque**: Implement a simplified Chase-Lev work-stealing deque using atomics. The owner pushes/pops from the bottom; thieves steal from the top. Verify correctness with concurrent producer and multiple stealers.
3. **Load measurement**: Add counters to the work-stealing demo: track how many jobs were processed from own queue vs. stolen. Compare these counts with uniform vs. skewed initial load distribution.
