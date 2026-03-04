# 458: Barrier — Rendezvous All Threads Before Proceeding

**Difficulty:** 3  **Level:** Intermediate

Synchronise N threads at a checkpoint — all must arrive before any can proceed, with one elected leader per phase for coordination work.

## The Problem This Solves

Parallel algorithms often have phases: initialise in parallel, then all threads must finish before computing in parallel, then all must finish before aggregating. If thread 3 starts phase 2 before thread 1 finishes phase 1, it operates on incomplete data — a race condition with no obvious error message, just silently wrong results.

The naive fix is for the main thread to `join` all workers, then re-spawn them for the next phase. This works but is expensive: thread creation cost multiplied by number of phases. A `Barrier` solves this cleanly: threads reach the barrier and block until the last one arrives, then all proceed simultaneously. No re-spawning, no separate coordination thread.

The leader mechanism addresses a common need: one thread should print phase headers, accumulate partial results, or reset state between phases. Electing a leader via a mutex or channel is boilerplate. `Barrier::wait()` returns a `BarrierWaitResult` where exactly one thread gets `is_leader() == true` per phase — built-in, no extra code.

## The Intuition

A `Barrier::new(n)` is a gate that only opens when `n` threads are waiting at it. The last thread to arrive swings the gate open and all threads proceed. Then the barrier resets for the next use. It's the parallel equivalent of waiting for everyone to sit down before starting dinner — you can use the same dinner table (barrier) for multiple meals (phases).

In Java: `CyclicBarrier`. In Python: `threading.Barrier`. In Go: no built-in — you'd build one from a `WaitGroup` per phase (not reusable like `Barrier`). Rust's `std::sync::Barrier` is reusable across phases automatically.

## How It Works in Rust

```rust
use std::sync::{Arc, Barrier};
use std::thread;

let n = 4;
let barrier = Arc::new(Barrier::new(n)); // all n threads must wait

let handles: Vec<_> = (0..n).map(|id| {
    let b = Arc::clone(&barrier);
    thread::spawn(move || {
        for phase in 1..=3 {
            // Do phase work (different amounts per thread — fast or slow)
            do_phase_work(id, phase);

            // Block here until all n threads arrive
            let result = b.wait();

            // Exactly one thread per phase gets is_leader() == true
            if result.is_leader() {
                println!("=== phase {} complete — all threads ready ===", phase);
                // Safe to aggregate results, log, reset shared state, etc.
            }
            // All threads continue here simultaneously
        }
    })
}).collect();

for h in handles { h.join().unwrap(); }
```

The barrier automatically resets after each `wait()` cycle — no `reset()` call needed. This makes it directly reusable for multi-phase algorithms without rebuilding it.

## What This Unlocks

- **Multi-phase parallel algorithms** — matrix multiplication passes, simulation timesteps, parallel sort phases — any algorithm where phase N+1 must see all of phase N's writes.
- **Parallel test frameworks** — all test threads set up fixtures, barrier, run tests, barrier, verify results.
- **Coordinated startup** — N worker threads initialize their local state, then all start processing simultaneously at the barrier to avoid a thundering herd.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Barrier | manual `Mutex` + `Condvar` + counter | `Barrier::new(n)` |
| Wait | block until count == n, then reset | `barrier.wait()` → `BarrierWaitResult` |
| Leader election | manual (first to unlock, or external) | `result.is_leader()` — exactly one per phase, built-in |
| Reuse across phases | new barrier or manual reset | automatic — same `Barrier` reused across all phases |
| Java equivalent | `CyclicBarrier` | `std::sync::Barrier` |
