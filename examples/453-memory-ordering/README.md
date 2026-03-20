📖 **[View on hightechmind.io →](https://hightechmind.io/rust/453-memory-ordering)**

---

# 453: Memory Ordering

## Problem Statement

Modern CPUs and compilers reorder instructions for performance. On a multi-core system, one thread's operations may appear in a different order to another thread. Memory ordering specifies the synchronization guarantees: `Relaxed` (no ordering guarantees), `Acquire`/`Release` (synchronized handoff between writer and reader), `AcqRel` (both acquire and release), `SeqCst` (total global order). Choosing the wrong ordering causes data races or needless performance loss. The Release-Acquire pair is the key idiom: a `Release` store "publishes" writes; an `Acquire` load "subscribes" to them.

Memory ordering is foundational to all lock-free programming, `Arc`'s reference counting, message passing channel internals, and spinlock implementations.

## Learning Outcomes

- Understand the five memory ordering modes: `Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst`
- Learn the Release-Acquire pattern: `store(..., Release)` and `load(..., Acquire)` form a happens-before edge
- See how `Relaxed` is sufficient for independent counters where ordering doesn't matter
- Understand why `SeqCst` is the safest default but has the highest cost
- Learn the C++11/20 memory model that Rust's atomics are based on

## Rust Application

In `src/lib.rs`, the `test_release_acquire` test demonstrates the pattern: thread 1 stores data with `Relaxed` and the flag with `Release`. The main thread loads the flag with `Acquire` — if it sees `true`, the happens-before edge guarantees it will also see `42` for data. The `Release` store on the flag synchronizes with the `Acquire` load, making all preceding `Relaxed` writes visible. This is the standard producer-consumer handoff pattern.

## OCaml Approach

OCaml 5.x's `Atomic` module uses sequential consistency for all operations — there is no explicit ordering control. The simplicity reduces bug potential but prevents optimizations that weaker orderings enable. OCaml's memory model is based on the "OCaml Memory Model" paper (2022), which is weaker than C11's sequentially consistent model in some edge cases involving non-atomic accesses.

## Key Differences

1. **Explicit control**: Rust exposes all five ordering modes; OCaml's atomics are always SeqCst.
2. **Complexity**: Rust's ordering flexibility enables optimization but requires expertise; OCaml's simplicity trades performance for safety.
3. **C11 correspondence**: Rust's orderings map directly to C11/C++11 orderings; OCaml has its own memory model.
4. **Non-atomic accesses**: Rust's non-atomic accesses are data races if unsynchronized; OCaml's GC values have special rules in OCaml 5.x.

## Exercises

1. **Spinlock**: Implement a spinlock using `AtomicBool` with `compare_exchange(false, true, Acquire, Relaxed)` for lock and `store(false, Release)` for unlock. Explain in a comment why these orderings are sufficient.
2. **Seqlock**: Implement a sequence lock (seqlock) — a writer increments a counter (odd = writing), copies data, increments again (even = done). A reader reads the counter (must be even and `Acquire`), reads data, reads counter again, retries if different. Use correct orderings.
3. **Ordering violation**: Write a test that demonstrates what can go wrong with `Relaxed` on a flag without the Release-Acquire pattern: have one thread write data then set a `Relaxed` flag, another spin on the `Relaxed` flag then read data. Document what incorrect result the reader might observe on weakly-ordered CPUs (ARM/POWER).
