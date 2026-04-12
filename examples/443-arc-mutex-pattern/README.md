📖 **[View on hightechmind.io →](https://hightechmind.io/rust/443-arc-mutex-pattern)**

---

# 443: `Arc<Mutex<T>>` — Shared Mutable State
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Multiple threads sharing a single mutable value is a fundamental concurrency pattern: a counter, a shared cache, a work queue. Rust's ownership model normally prevents this — only one owner can have mutable access. `Arc<T>` enables multiple-ownership across threads (atomic reference counting), and `Mutex<T>` ensures exclusive access — only one thread holds the lock at a time. Together, `Arc<Mutex<T>>` is the standard Rust pattern for shared mutable state across threads.

`Arc<Mutex<T>>` appears in every multi-threaded Rust program: shared caches, event buses, job queues, game state machines, and any pattern requiring coordinated mutation from multiple threads.

## Learning Outcomes

- Understand how `Arc` (atomic reference counting) enables shared ownership across threads
- Learn how `Mutex` provides exclusive access with automatic unlock on drop (RAII guard)
- See the `Arc::clone(&counter)` pattern for sharing ownership across spawned threads
- Understand the lock guard pattern: `counter.lock().unwrap()` returns a `MutexGuard`
- Learn the performance implication: every lock acquisition is a synchronization point

## Rust Application

In `src/lib.rs`, `parallel_increment` creates `Arc::new(Mutex::new(0u64))`. Each spawned thread clones the `Arc` (incrementing the reference count), then acquires the mutex with `.lock().unwrap()` and increments the counter. The `MutexGuard` automatically releases the lock when it goes out of scope. After all threads join, the counter holds the correct total. `parallel_collect` uses `Arc<Mutex<Vec<T>>>` for shared collection building.

## OCaml Approach

OCaml uses `Mutex.create()` and `Mutex.lock`/`Mutex.unlock` for mutual exclusion. Shared mutable state is `ref` or mutable record fields protected by a mutex. OCaml 5.x's `Atomic.t` handles simple counter patterns without locks. Unlike Rust, OCaml doesn't enforce that all accesses to a shared value go through a mutex — the type system doesn't track this invariant.

## Key Differences

1. **Type enforcement**: Rust's `Mutex<T>` forces all access through the lock; OCaml's `Mutex.t` is advisory — you can access the value without locking.
2. **Poisoning**: Rust's mutex becomes "poisoned" if a thread panics while holding it; subsequent `lock()` calls return `Err`. OCaml has no poisoning concept.
3. **RAII unlock**: Rust's `MutexGuard` unlocks on drop automatically; OCaml requires explicit `Mutex.unlock` (forgetting it = deadlock).
4. **Arc vs. GC**: Rust needs `Arc` for shared ownership; OCaml's GC manages reference counting transparently.

## Exercises

1. **Rate limiter**: Build a `RateLimiter` using `Arc<Mutex<(u32, Instant)>>` tracking (count, window_start). `fn check_and_increment(&self) -> bool` returns true if the rate limit allows the request, false if exceeded. Test with concurrent threads.
2. **Bounded queue**: Implement a `BoundedQueue<T>` wrapping `Arc<Mutex<VecDeque<T>>>` with a capacity limit. `push` returns `Err(val)` when full; `pop` returns `None` when empty. Verify correct behavior with producer/consumer threads.
3. **Deadlock avoidance**: Write a program that could deadlock with `Arc<Mutex<T>>` (two threads each holding one lock waiting for the other's lock). Then fix it using lock ordering or `try_lock` with backoff.
