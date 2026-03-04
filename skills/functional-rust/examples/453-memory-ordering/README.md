# 453: Memory Ordering — Acquire, Release, SeqCst

**Difficulty:** 3  **Level:** Intermediate

Control how atomic operations synchronise visibility across CPU cores — the difference between a correct lock-free algorithm and one that silently fails on ARM.

## The Problem This Solves

Atomic operations prevent torn reads and lost writes on a single memory location. But they don't, by themselves, say anything about other memory. If thread A writes data to a slice and then sets a flag with `store(true, Relaxed)`, thread B might see the flag as `true` but still see the old (pre-write) data. This isn't a bug in the atomic — it's the CPU reordering memory operations for performance. Modern CPUs (especially ARM) reorder reads and writes aggressively.

This breaks the pattern at the core of every lock-free algorithm: "write data first, then publish a flag; reader sees flag, then reads data." Without ordering constraints, the reader might observe the flag before the data writes are visible. On x86 this is rare because x86 has a relatively strong memory model. On ARM (your phone, your Raspberry Pi, AWS Graviton) it happens regularly.

The fix is not a Mutex — the whole point of atomics is to avoid locks. The fix is `Release`/`Acquire` ordering: `store(true, Ordering::Release)` says "all my writes before this store are visible to any thread that does `load(Ordering::Acquire)` on the same location and sees `true`". This pair is the fundamental synchronisation primitive underneath every lock in every language.

## The Intuition

Think of memory ordering as a contract between threads:

- **`Relaxed`** — just do the atomic op; no promises about other memory. Correct for isolated counters.
- **`Release`** (store) — "fence the past": everything I wrote before this store is visible to whoever picks up this value with `Acquire`.
- **`Acquire`** (load) — "fence the future": if I see a `Release`-stored value, all writes the storing thread made before that store are now visible to me.
- **`SeqCst`** — total global order: all threads agree on the sequence of all SeqCst operations. Slowest, safest, simplest to reason about.

A useful mental model: `Release` publishes. `Acquire` subscribes. Once you see the published value, you're guaranteed to see all the data that was prepared before publication.

## How It Works in Rust

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

let data: Arc<Vec<AtomicUsize>> =
    Arc::new((0..10).map(|_| AtomicUsize::new(0)).collect());
let ready = Arc::new(AtomicBool::new(false));

// Producer: write data, then publish with Release
let (dw, rw) = (Arc::clone(&data), Arc::clone(&ready));
let producer = thread::spawn(move || {
    for (i, cell) in dw.iter().enumerate() {
        cell.store(i * i, Ordering::Relaxed); // fine — data writes are Relaxed
    }
    // Release: all writes above are now visible to any Acquire load that sees true
    rw.store(true, Ordering::Release);
});

// Consumer: spin on Acquire, then read data
let (dr, rr) = (Arc::clone(&data), Arc::clone(&ready));
let consumer = thread::spawn(move || {
    while !rr.load(Ordering::Acquire) { // Acquire: if true, data is visible
        thread::yield_now();
    }
    // Safe to read — Release/Acquire pair guarantees data visibility
    let sum: usize = dr.iter().map(|c| c.load(Ordering::Relaxed)).sum();
    println!("sum = {}", sum);
});

producer.join().unwrap();
consumer.join().unwrap();
```

For most application-level code: use `SeqCst` everywhere and never think about ordering. The performance cost of `SeqCst` over `Release`/`Acquire` is small in practice and the correctness benefit is large. Switch to `Acquire`/`Release` only when profiling shows atomic contention as a bottleneck.

## What This Unlocks

- **Lock-free publish/subscribe** — a producer writes a batch of data and sets a `Release` flag; consumers wait on `Acquire` and read the data with `Relaxed` — correct and faster than a mutex.
- **Reference counting** — `Arc::clone` uses `Relaxed` fetch-add; `Arc::drop` uses `Release` decrement + `Acquire` fence before destructor. Understanding this lets you build custom reference-counted types.
- **Reasoning about lock implementations** — every mutex in every language is built on `Release` store (unlock) + `Acquire` load (lock). This is the foundation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Relaxed | no direct equivalent (OCaml 5 is sequentially consistent) | `Ordering::Relaxed` — fastest, no cross-thread guarantees |
| Release | implicit in OCaml 5 | `Ordering::Release` on stores — publishes prior writes |
| Acquire | implicit in OCaml 5 | `Ordering::Acquire` on loads — sees all writes before Release |
| SeqCst | OCaml 5 default | `Ordering::SeqCst` — total order, safest, ~same cost as Acquire on x86 |
| When to use Relaxed | N/A | isolated counters with no data dependency |
| When to use SeqCst | always | default for correctness; optimise to Acq/Rel only if needed |
