# 468: Lock-Free Queue

**Difficulty:** 3  **Level:** Intermediate

Enqueue and dequeue without mutexes — using compare-and-swap loops on atomic pointers.

## The Problem This Solves

A `Mutex<VecDeque>` queue serialises every enqueue and dequeue. Under high throughput — millions of messages per second — threads spend more time waiting for the lock than doing real work. The kernel context-switch to park a thread costs microseconds; lock-free queues avoid that entirely.

A lock-free queue lets multiple threads enqueue and dequeue concurrently using CPU atomic instructions (CAS — compare-and-swap). If two threads race, one wins and the other retries. No thread ever blocks. In the uncontended case (the common case) the fast path is a handful of atomic operations.

This is the pattern behind `std::sync::mpsc` and the core data path in async runtimes.

## The Intuition

Two people sharing a whiteboard. Instead of taking turns with a lock on the whiteboard, each one writes their message in a specific spot and uses a sticky note to say "next message is over there." If two people try to update the sticky note simultaneously, only one succeeds — the other looks again and tries a different spot. Nobody waits; they just retry.

## How It Works in Rust

The Michael-Scott queue is the classic algorithm:

1. **Node structure** — each node holds a value and an `AtomicPtr` to the next node.
2. **Head and tail** — the queue maintains `AtomicPtr<Node>` for both. Head is a sentinel (dummy) node.
3. **Enqueue** — allocate a new node, then CAS the current tail's `next` pointer from null to the new node. Swing the tail pointer forward.
   ```rust
   loop {
       let tail = self.tail.load(Acquire);
       let next = (*tail).next.load(Acquire);
       if next.is_null() {
           if (*tail).next.compare_exchange(null_mut(), node, Release, Relaxed).is_ok() {
               self.tail.compare_exchange(tail, node, Release, Relaxed).ok();
               return;
           }
       } else {
           // help advance the tail
           self.tail.compare_exchange(tail, next, Release, Relaxed).ok();
       }
   }
   ```
4. **Dequeue** — CAS the head sentinel forward; the old sentinel's value becomes the returned item.
5. **Memory safety** — use `crossbeam-epoch` to defer-free unlinked nodes safely (see example 467).

## What This Unlocks

- **High-throughput messaging** — uncontended paths hit L1 cache; no system calls, no context switches.
- **Progress guarantees** — lock-free means at least one thread always makes forward progress even if others stall.
- **Foundation for channels** — production MPSC queues (like `tokio::sync::mpsc` internals) build on this.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lock-free queue | Not available in stdlib | `AtomicPtr` + CAS loop |
| Memory reclamation | GC | Epoch-based (`crossbeam-epoch`) |
| Ordering guarantees | Sequential consistency (GC barrier) | Explicit `Acquire`/`Release` |
| Practical crate | — | `crossbeam-queue::SegQueue` |
