📖 **[View on hightechmind.io →](https://hightechmind.io/rust/924-work-stealing)**

---

# 924-work-stealing — Work Stealing
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A fixed thread pool with a single shared work queue can become a bottleneck: all workers contend for the same lock. Work-stealing distributes this: each worker has its own local queue; when idle, a worker "steals" tasks from another worker's queue. This approach was invented for the Cilk parallel runtime at MIT and is used in Java's `ForkJoinPool`, .NET's `ThreadPool`, Go's goroutine scheduler, and Rust's `rayon`. Local queues use lock-free algorithms for pushes; stealing requires only occasional locking. The result is near-linear scaling on multi-core processors for divide-and-conquer workloads.

## Learning Outcomes

- Understand the work-stealing algorithm and why it scales better than a shared queue
- Recognize divide-and-conquer as the canonical work-stealing workload
- Understand why local queues use LIFO (stack) order — better cache locality
- Understand why stealing takes from the other end (FIFO) — avoids the biggest tasks being stolen
- Compare with OCaml's `Domainslib` and `Moonpool` for parallel work stealing

## Rust Application

The placeholder implementation indicates where a work-stealing pool would be built. In practice: each thread has a local `VecDeque` (deque). `push_front` adds new work locally. `pop_front` takes the most recently added work (LIFO — cache warm). When the local deque is empty, steal from another thread's `pop_back` (FIFO — takes oldest work, which is usually the largest). The `crossbeam-deque` crate provides a production lock-free work-stealing deque. `rayon` builds on this for the `par_iter()` parallel iterator API.

## OCaml Approach

OCaml 5 introduces `Domain.spawn` for true parallelism. `Domainslib` builds work-stealing on top of domains. `Task.pool` creates a pool; `Task.async pool f` submits a task; `Task.await` retrieves the result. `Moonpool` provides an alternative with configurable scheduling. OCaml's cooperative (`Lwt`, `Eio`) runtimes do not benefit from work-stealing (they run on one domain unless using `Eio.Fiber`). The biggest difference: OCaml 5 domains are heavyweight compared to Rust threads.

## Key Differences

1. **Lock-free deques**: Rust work-stealing implementations use lock-free double-ended queues for near-contention-free local operations; OCaml requires explicit domain synchronization.
2. **rayon integration**: Rust's `rayon` provides work-stealing transparently via `par_iter()` — no manual thread management; OCaml requires explicit `Domainslib` API calls.
3. **Recursion and fork-join**: Rust `rayon::join(f, g)` is the idiomatic parallel divide-and-conquer entry point; OCaml `Task.async` is the equivalent.
4. **Production readiness**: `rayon` is the production Rust solution for CPU-bound parallel work; OCaml's parallel ecosystem is still maturing (OCaml 5 is relatively new).

## Exercises

1. Implement parallel merge sort using a work-stealing approach: spawn two recursive tasks for the two halves.
2. Write a parallel `find_any` that searches for a value across N worker threads, cancelling others when found.
3. Measure the speedup of a parallel sum over a large array using 1, 2, 4, and 8 workers with different chunk sizes.
