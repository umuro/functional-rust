📖 **[View on hightechmind.io →](https://hightechmind.io/rust/452-atomic-types)**

---

# 452: Atomic Types — Lock-Free Concurrent Primitives

## Problem Statement

Mutex-based synchronization has overhead: kernel entry, thread scheduling, potential contention. For simple operations on single values — incrementing a counter, setting a flag, tracking a maximum — atomic hardware instructions provide the same guarantee without a lock. Modern CPUs support atomic read-modify-write operations (fetch_add, compare_and_swap) that execute atomically from all other cores' perspectives. Rust's `std::sync::atomic` module exposes these directly.

Atomics power reference counting (`Arc`'s inner counter), lock-free data structures, metrics counters, progress tracking, and any concurrent primitive where one CPU instruction is sufficient.

## Learning Outcomes

- Understand atomic operations: fetch_add, fetch_sub, load, store, compare_and_swap
- Learn the `Ordering` parameter: `SeqCst`, `Acquire`, `Release`, `Relaxed` and when each is appropriate
- See how `AtomicCounter`, `AtomicBool`, and `AtomicUsize` enable lock-free concurrent primitives
- Understand why `Arc`'s reference counting uses `AtomicUsize` not `Mutex<usize>`
- Learn the performance advantage: no kernel calls, no lock contention, no scheduling

## Rust Application

In `src/lib.rs`, `AtomicCounter` wraps `AtomicUsize` with `fetch_add(1, Ordering::SeqCst)` for increment and `load(Ordering::SeqCst)` for read. `AtomicBool` provides a flag for shutdown signaling. Multiple threads increment the counter concurrently without any locking. `SeqCst` (sequentially consistent) ordering is used throughout — the safest but most conservative choice; `Relaxed` ordering would be sufficient for independent counters.

## OCaml Approach

OCaml 5.x provides `Atomic.make`, `Atomic.get`, `Atomic.set`, `Atomic.compare_and_set`, and `Atomic.fetch_and_add` for atomic operations. OCaml 4.x doesn't need atomics for reference counting since the GIL handles it. The `Atomic.t` type in OCaml 5.x is similar to Rust's `AtomicT` types but without explicit ordering specification — OCaml uses sequentially consistent semantics by default.

## Key Differences

1. **Ordering control**: Rust exposes `Ordering` explicitly enabling optimization; OCaml 5.x's atomics use sequential consistency without user control.
2. **Type variety**: Rust has `AtomicBool`, `AtomicI8`/`U8` through `AtomicI64`/`U64`, `AtomicIsize`/`Usize`, `AtomicPtr`; OCaml has a single `'a Atomic.t`.
3. **Performance model**: Rust's `Relaxed` ordering is the cheapest; OCaml's fixed SeqCst has consistent but potentially higher cost.
4. **Arc counter**: Rust's `Arc` uses `AtomicUsize` for reference counting directly; OCaml's GC handles reference counting transparently.

## Exercises

1. **Lock-free max**: Implement `fn update_max(current: &AtomicUsize, value: usize)` that atomically updates the stored maximum. Use `compare_exchange` in a loop. Test with 16 threads each trying to set the max.
2. **Ordering experiment**: Write a test demonstrating that `Relaxed` ordering on two independent counters can produce results that would be impossible with sequential consistency. (Hint: requires specific CPU architectures — document why x86 may not show the effect.)
3. **Arc from scratch**: Implement a simplified `MyArc<T>` using a raw pointer to `(T, AtomicUsize)`. Implement `Clone` (increment count with `fetch_add`) and `Drop` (decrement with `fetch_sub`, free if zero). Verify correctness with 4 threads sharing the same value.
