# 447: Work-Stealing Pattern

**Difficulty:** 3  **Level:** Intermediate

A concurrency scheduling pattern where idle threads steal work from busy ones — minimizing contention and maximizing CPU utilization.

## The Problem This Solves

Naive thread pools distribute work upfront — each thread gets a fixed share. When one thread finishes early and others still have a backlog, the early thread idles while work waits. Load balancing by reassignment requires locking a shared queue, creating a bottleneck as all threads compete for the same mutex.

Work-stealing solves both problems with a clever data structure: each thread has its own deque (double-ended queue). Threads take work from the *front* of their own deque. When idle, they steal from the *back* of another thread's deque. This halves contention — owners and thieves access opposite ends, reducing lock conflicts — and makes utilization near-optimal: no thread idles while work exists anywhere in the system.

This is how Rayon, Go's goroutine scheduler, and Java's `ForkJoinPool` work. Understanding it gives you the mental model for why parallel iterators like `par_iter()` scale so well.

## The Intuition

Each worker owns a queue; idle workers steal from the back of busy workers' queues — owners and stealers access opposite ends to minimize lock contention.

## How It Works in Rust

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

type Queue = Arc<Mutex<VecDeque<u32>>>;

fn worker(id: usize, own: Queue, others: Vec<Queue>) {
    loop {
        // Try own work first (from the front)
        if let Some(job) = own.lock().unwrap().pop_front() {
            println!("worker {} owns job {}", id, job);
            // ... do work
            continue;
        }

        // Idle: try stealing from back of others
        let mut stole = false;
        for q in &others {
            if let Ok(mut guard) = q.try_lock() {  // non-blocking!
                if let Some(job) = guard.pop_back() {
                    println!("worker {} STOLE job {}", id, job);
                    drop(guard);  // release lock before working
                    stole = true;
                    break;
                }
            }
        }

        if !stole { break; }  // all queues empty — done
    }
}

// Setup: load all work into first worker
let queues: Vec<Queue> = (0..4)
    .map(|_| Arc::new(Mutex::new(VecDeque::new())))
    .collect();

for job in 0..20u32 { queues[0].lock().unwrap().push_back(job); }

let handles: Vec<_> = (0..4).map(|i| {
    let own = Arc::clone(&queues[i]);
    let others: Vec<_> = queues.iter().enumerate()
        .filter(|&(j, _)| j != i)
        .map(|(_, q)| Arc::clone(q))
        .collect();
    thread::spawn(move || worker(i, own, others))
}).collect();

for h in handles { h.join().unwrap(); }
```

1. Each thread owns an `Arc<Mutex<VecDeque<T>>>`.
2. Own work: `pop_front()` — the productive path.
3. Steal: `try_lock()` (non-blocking!) on other queues, then `pop_back()`.
4. `try_lock` is critical: a blocking lock would cause deadlocks when all threads try to steal simultaneously.

## What This Unlocks

- **Near-optimal CPU utilization**: Work naturally migrates to idle cores without centralized coordination.
- **Low contention**: Owners and stealers access opposite ends — the common case (owner consuming own work) is contention-free.
- **Foundation for Rayon**: `par_iter()` uses work-stealing internally — now you know why it scales.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread pool | `Domain` + `Effect` (OCaml 5) | `std::thread` + `Arc<Mutex<...>>` |
| Shared mutable queue | `Mutex` from `Thread` module | `Arc<Mutex<VecDeque<T>>>` |
| Non-blocking lock attempt | `Mutex.try_lock` | `.try_lock()` returns `TryLockResult` |
| Work-stealing library | Domainslib (OCaml 5) | `crossbeam-deque` (production), Rayon |
| Production-grade | Domainslib's task pool | `crossbeam::deque::Worker` / `Stealer` |
