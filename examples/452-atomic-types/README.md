📖 **[View on hightechmind.io →](https://hightechmind.io/rust/452-atomic-types)**

---

# 452: Atomic Types — Lock-Free Operations on Single Values

**Difficulty:** 3  **Level:** Intermediate

Use `AtomicUsize`, `AtomicBool`, and friends for thread-safe counters and flags without any mutex — single CPU instructions, no kernel involvement.

## The Problem This Solves

`Arc<Mutex<u64>>` for a counter is correct but heavyweight. Every increment takes a lock: a compare-and-swap to acquire, the increment, another to release. On a busy system this means threads queuing, kernel involvement, and cache line bouncing. For a simple counter or a shutdown flag, that's enormous overhead for what could be a single instruction.

The subtler problem is contention. With a mutex-protected counter shared by 16 threads, most threads spend their time waiting. Throughput plateaus. The mutex becomes the bottleneck. Atomic operations fix this: `fetch_add` is a single `LOCK XADD` x86 instruction. Hardware handles the mutual exclusion at the CPU level — faster than any software lock, no context switching.

Atomic types also prevent a subtle class of bugs: torn reads and writes. On a 64-bit system, reading a `u64` that's being written by another thread without synchronisation can produce a value where the high 32 bits are the old value and the low 32 bits are the new value. This is undefined behavior in C/C++. Rust's `AtomicU64` makes such reads safe by using the platform's atomic load instruction.

## The Intuition

An atomic operation is indivisible from every other thread's perspective. No thread can observe a half-completed fetch-add. The CPU's memory bus or cache coherency protocol handles this. Atomics are the lowest-level synchronisation primitive — everything else (mutexes, channels, condvars) is built on top of them.

In Java: `AtomicInteger`, `AtomicBoolean`. In Python: integers are protected by the GIL, but that disappears with true parallelism. In Go: `sync/atomic` package. Rust's `std::sync::atomic` is a direct mapping to CPU atomic instructions, with the ordering argument (more on that in example 453) controlling cross-CPU visibility.

## How It Works in Rust

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// Lock-free counter — no Mutex needed
let counter = Arc::new(AtomicUsize::new(0));

let handles: Vec<_> = (0..4).map(|_| {
    let c = Arc::clone(&counter);
    thread::spawn(move || {
        for _ in 0..1000 {
            c.fetch_add(1, Ordering::Relaxed); // single LOCK XADD instruction
        }
    })
}).collect();
for h in handles { h.join().unwrap(); }
println!("{}", counter.load(Ordering::SeqCst)); // 4000 — correct

// Shutdown flag — avoid spinning on a Mutex
let running = Arc::new(AtomicBool::new(true));
let r = Arc::clone(&running);
let worker = thread::spawn(move || {
    while r.load(Ordering::Relaxed) {  // cheap: just a load instruction
        // do work
    }
});
running.store(false, Ordering::Relaxed); // signal shutdown
worker.join().unwrap();

// fetch_add returns the OLD value
let a = AtomicUsize::new(10);
let old = a.fetch_add(5, Ordering::SeqCst);
println!("old={} new={}", old, a.load(Ordering::SeqCst)); // old=10 new=15
```

The `Ordering` argument controls memory visibility across CPUs — not correctness of the atomic operation itself. For independent counters, `Relaxed` is correct and fastest. For operations that need to synchronise other memory (like the shutdown flag above), use `Release`/`Acquire` or `SeqCst`. See example 453.

## What This Unlocks

- **High-performance counters** — request counts, bytes transferred, cache hits — updated by many threads without locking.
- **Shutdown and cancellation flags** — a background thread checks `AtomicBool::load(Relaxed)` in its loop; the main thread signals stop with `store(false, Relaxed)`.
- **Lock-free algorithms** — the foundation for compare-exchange loops, lock-free stacks, and reference counting (what `Arc` uses internally).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Atomic int | `Atomic.make 0` (OCaml 5+) | `AtomicUsize::new(0)` |
| Fetch-add | `Atomic.fetch_and_add a 1` | `a.fetch_add(1, Ordering::Relaxed)` |
| Returns old value | yes | yes |
| Stop flag | `Atomic.make false` | `AtomicBool::new(false)` |
| Ordering | implicit (sequential) | explicit `Relaxed` / `Acquire` / `Release` / `SeqCst` |
| Available types | `int`, `bool`, `float` | `AtomicU8/16/32/64/Usize`, `AtomicI*`, `AtomicBool`, `AtomicPtr<T>` |
