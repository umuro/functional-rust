📖 **[View on hightechmind.io →](https://hightechmind.io/rust/457-condvar-pattern)**

---

# 457: Condition Variable Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

A mutex protects shared state but provides no way for a thread to wait until the state changes. A consumer thread checking "is the queue empty?" must repeatedly check (busy-wait) or sleep — wasting CPU or adding latency. Condition variables solve this: a thread calls `wait` (which atomically releases the mutex and sleeps), and another thread calls `notify_one` or `notify_all` when the state changes. The waiting thread re-acquires the mutex and re-checks the condition. This is the foundation of thread synchronization: producer-consumer queues, barrier synchronization, and event notification.

Condition variables are used in `thread::park`, bounded channel internals, database row locking, async runtime wakers, and any producer-consumer pattern requiring efficient sleep-until-ready.

## Learning Outcomes

- Understand how `Condvar` enables efficient waiting for state changes without busy-waiting
- Learn the `Arc<(Mutex<State>, Condvar)>` pair pattern for sharing condition variables
- See how `wait_while(guard, |s| condition)` atomically releases lock and blocks until condition is false
- Understand spurious wakeups and why condition variables always use a predicate loop
- Learn `notify_one` vs. `notify_all` semantics

## Rust Application

In `src/lib.rs`, the test creates `Arc<(Mutex<bool>, Condvar)>`. The spawned thread sets the boolean to true and calls `condvar.notify_one()`. The main thread calls `condvar.wait_while(guard, |&mut v| !v)` — blocking until the boolean is true. The `wait_while` handles spurious wakeups automatically by re-checking the predicate. The mutex is atomically released during wait, allowing the notifier to acquire it.

## OCaml Approach

OCaml's `Condition.t` provides the same primitive: `Condition.wait cond mutex` atomically releases the mutex and waits; `Condition.signal cond` wakes one waiter; `Condition.broadcast cond` wakes all. The pattern `let (mutex, cond) = ...` and `while not predicate do Condition.wait cond mutex done` handles spurious wakeups. OCaml 4.x's `Thread.create` + `Mutex` + `Condition` is the standard synchronization toolkit.

## Key Differences

1. **Pair pattern**: Rust idiomatically pairs `(Mutex<T>, Condvar)` as a tuple; OCaml typically uses separate `mutex` and `condition` values.
2. **wait_while**: Rust's `Condvar::wait_while` handles the spurious wakeup loop automatically; OCaml requires explicit `while` loop.
3. **Guard reacquisition**: Rust's `wait` returns the `MutexGuard` when woken; OCaml requires re-locking explicitly after `Condition.wait`.
4. **Parking**: Rust's `thread::park` + `unpark` is a simpler one-shot condition variable; OCaml has no equivalent.

## Exercises

1. **Bounded producer-consumer**: Implement a `BoundedQueue<T>` using `Arc<(Mutex<VecDeque<T>>, Condvar, Condvar)>` — one `Condvar` for "not empty" (unblocks consumers) and one for "not full" (unblocks producers). Test with 4 producers and 4 consumers.
2. **Event broadcast**: Create an `EventBroadcaster` using `Condvar::notify_all`. When `broadcast()` is called, all waiting threads receive the event simultaneously. Verify with 10 waiting threads that all unblock on a single broadcast.
3. **Timeout wait**: Use `Condvar::wait_timeout_while` to implement a `try_recv_timeout(timeout: Duration) -> Option<T>` that waits for an item but gives up after the timeout. Verify it returns `None` when no producer sends within the timeout.
