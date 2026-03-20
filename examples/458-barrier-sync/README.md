📖 **[View on hightechmind.io →](https://hightechmind.io/rust/458-barrier-sync)**

---

# 458: Barrier Synchronization

## Problem Statement

Parallel algorithms often have phases: all threads complete phase 1 before any begins phase 2. A barrier synchronization point ensures all N threads reach the barrier before any proceed. Without barriers, fast threads would start phase 2 while slow threads are still in phase 1, reading partially-computed data. Barriers are the foundation of parallel algorithm phases in scientific computing (iterative solvers, particle simulations), data pipeline stages, and distributed systems consensus.

`std::sync::Barrier` is used in parallel numerical solvers, image processing pipelines, distributed consensus protocols (simulated with threads), and test synchronization.

## Learning Outcomes

- Understand how `Barrier::new(n)` creates a synchronization point for n threads
- Learn how `barrier.wait()` blocks until all n threads have called it
- See how `BarrierWaitResult::is_leader()` identifies one thread per barrier crossing
- Understand barrier phases: a barrier can be reused to synchronize multiple phases
- Learn when barriers are appropriate vs. `join` (all-to-one vs. all-to-all)

## Rust Application

In `src/lib.rs`, `Barrier::new(n)` creates a barrier for `n` threads. Each thread increments a counter, then calls `b.wait()`. The assertion `c.load() == n` after the barrier verifies all threads have incremented before any thread proceeds. `test_one_leader` demonstrates `is_leader()` — exactly one thread per barrier crossing gets `true` from `wait()`, enabling post-barrier coordination by one designated thread.

## OCaml Approach

OCaml doesn't have a built-in `Barrier` type. A barrier is implemented with a `Mutex` + `Condvar` + counter: increment the counter under the mutex, then wait on the condvar until the count reaches N, then `Condition.broadcast`. OCaml 5.x's `Domainslib.Task.async`/`await` provides structured synchronization without manual barriers. The `Thread.join` approach works for one-shot synchronization.

## Key Differences

1. **Built-in**: Rust's `std::sync::Barrier` is in the standard library; OCaml requires manual implementation with `Mutex` + `Condvar`.
2. **Reusability**: Rust's `Barrier` can be reused across multiple phases; OCaml's manual implementation requires explicit reset.
3. **Leader selection**: Rust's `BarrierWaitResult::is_leader()` identifies one thread per crossing; OCaml's implementation would need explicit tracking.
4. **Phase alignment**: Rust's barrier naturally aligns parallel phases; OCaml uses `Task.await` chains for structured coordination.

## Exercises

1. **Two-phase computation**: Use a barrier to implement two-phase Gaussian elimination: phase 1 all threads compute row reductions, barrier, phase 2 all threads back-substitute. Verify the result matches sequential elimination.
2. **Leader action**: Use `is_leader()` to have exactly one thread write intermediate results to a file between phases. All other threads continue with phase 2 while the leader writes.
3. **Barrier timeout**: Implement a `TimedBarrier` that acts like `Barrier` but returns an error if not all threads arrive within a timeout. Use `Condvar::wait_timeout_while` internally.
