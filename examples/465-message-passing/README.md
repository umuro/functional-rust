📖 **[View on hightechmind.io →](https://hightechmind.io/rust/465-message-passing)**

---

# 465: Message Passing vs. Shared Memory
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Concurrent programming has two fundamental coordination models. Shared memory: threads share data structures, protected by locks. Message passing: threads communicate by sending values, with no shared state. Shared memory is faster for fine-grained updates (no serialization); message passing is safer, scales better, and avoids deadlocks. The word-count example demonstrates both: message passing (each thread computes a local count, sends the whole map) vs. shared memory (all threads write into a shared `HashMap` under a mutex).

This comparison is central to Go's mantra "don't communicate by sharing memory, share memory by communicating" and Erlang's everything-is-messages model.

## Learning Outcomes

- Understand the message passing model: no shared state, pass computed results
- Learn the shared memory model: protect shared data structures with locks
- See how the word-count problem demonstrates both approaches
- Understand the performance trade-off: message passing avoids lock contention but serializes merges
- Learn when each model is appropriate (read-heavy vs. write-heavy workloads)

## Rust Application

In `src/lib.rs`, `msg_passing` spawns N threads each computing a local `HashMap` and sending it via channel. The main thread merges all maps sequentially — no locks needed during computation. `shared_memory` uses `Arc<Mutex<HashMap>>` where all threads lock and update the shared map for each word. The message passing version has lower contention during computation but a merge bottleneck; the shared memory version has high lock contention but no final merge.

## OCaml Approach

OCaml's message passing uses channels: `let ch = Event.new_channel()` with `Thread.create (fun () -> Event.sync (Event.send ch result)) ()`. Shared memory uses `Hashtbl.t` + `Mutex.t`. Functional OCaml naturally favors message passing (immutable local maps merged at end) since persistent data structures share structure efficiently. The `Parallel` library provides higher-level abstractions for both models.

## Key Differences

1. **Contention**: Message passing has no lock contention during computation; shared memory locks on every write.
2. **Memory**: Message passing creates N copies of partial results; shared memory uses one structure (but serializes writes).
3. **Correctness**: Message passing is easier to reason about (no shared mutable state); shared memory requires careful lock discipline.
4. **Scale**: Message passing scales better to more threads; shared memory becomes a bottleneck as thread count increases.

## Exercises

1. **Benchmark comparison**: Implement both approaches for counting words in 100 documents of 10,000 words each. Benchmark with 1, 2, 4, 8, and 16 threads. Plot throughput and identify where each approach breaks down.
2. **Hybrid approach**: Combine both: use message passing to compute partial results in parallel (no lock contention), then merge partial results using a parallel fan-in tree (multiple levels of merging) to avoid sequential merge bottleneck.
3. **Histogram computation**: Apply both patterns to computing a histogram of a billion integer samples. Message passing: each thread counts local histogram, sends to main for merge. Shared memory: `Arc<Vec<AtomicUsize>>` for per-bucket atomic increment. Compare performance.
