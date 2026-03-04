# 345: Work Stealing

**Difficulty:** 5  **Level:** Expert

Each worker maintains its own local job queue; idle workers steal from busy ones — better CPU utilization than a single shared queue.

## The Problem This Solves

A simple thread pool with one shared queue has a bottleneck: every job dispatch and pickup requires locking the shared queue. Under high load with many small tasks, threads spend significant time waiting for the lock, not doing work. You've paid for N cores but can't fully use them.

A worse problem: uneven work distribution. If task 1 takes 1ms but task 2 takes 1 second, a simple round-robin or FIFO pool leaves workers idle while one thread grinds through the long task. No rebalancing happens.

Work stealing fixes both: each thread has its own queue (no contention on the hot path), and idle threads steal from the back of busy threads' queues. Short tasks finish fast on their local thread; long tasks get split when others are idle. This is how `rayon` and `tokio` achieve near-linear CPU utilization.

## The Intuition

Imagine a team of chefs preparing dishes. One approach: all dishes go in a single pile, each chef takes from it (simple pool — lock contention). Better approach: each chef has their own pile. When a chef finishes their pile, they walk over to the busiest chef and take some work off their stack.

The "steal from the back" part is deliberate: the current worker takes from the *front* (LIFO — most recently added work is hottest in cache), while stealers take from the *back* (FIFO — oldest work, so they don't interfere with the owner's cache-hot work).

Work stealing was popularized by the Cilk project (MIT), then adopted by Java's `ForkJoinPool`, Go's goroutine scheduler, and Rust's `rayon`. It's the standard approach for parallel runtime schedulers.

```
Worker 1 queue (front):  [job5] [job4] [job3] [job2] [job1] (back)
  - Worker 1 takes job5 (front)
  - Worker 3 (idle) steals job1 (back) — least recently added, least cache-hot
```

This example simulates work stealing using `mpsc` channels and thread coordination — the structure without the lock-free deque optimization that production implementations use.

## How It Works in Rust

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn work_stealing_pool(num_workers: usize, jobs: Vec<Job>) {
    // Each worker has its own queue (VecDeque for deque operations)
    let queues: Vec<Arc<Mutex<VecDeque<Job>>>> = (0..num_workers)
        .map(|_| Arc::new(Mutex::new(VecDeque::new())))
        .collect();

    // Distribute initial work round-robin across worker queues
    for (i, job) in jobs.into_iter().enumerate() {
        queues[i % num_workers].lock().unwrap().push_back(job);
    }

    let queues = Arc::new(queues);
    let (done_tx, done_rx) = mpsc::channel::<()>();

    let handles: Vec<_> = (0..num_workers).map(|id| {
        let queues = Arc::clone(&queues);
        let done_tx = done_tx.clone();
        thread::spawn(move || {
            loop {
                // Try local queue first (pop from front — most recently added)
                let job = queues[id].lock().unwrap().pop_front();

                let job = job.or_else(|| {
                    // Local queue empty — try to steal from another worker's back
                    for other_id in 0..queues.len() {
                        if other_id == id { continue; }
                        if let Some(stolen) = queues[other_id].lock().unwrap().pop_back() {
                            return Some(stolen);  // steal!
                        }
                    }
                    None
                });

                match job {
                    Some(job) => job(),              // run the job
                    None => { let _ = done_tx.send(()); break; }  // no work anywhere
                }
            }
        })
    }).collect();

    drop(done_tx);
    // Wait for all workers to signal done
    for _ in 0..num_workers { done_rx.recv().ok(); }
    for h in handles { h.join().unwrap(); }
}
```

Production work-stealing (rayon, tokio) uses lock-free deques (`crossbeam-deque`) — the owner uses push/pop (no locking), stealers use a CAS-based steal operation. The Mutex version here is conceptually correct but wouldn't scale to thousands of tiny tasks.

## What This Unlocks

- **CPU-bound parallel algorithms**: rayon's `par_iter()` uses work stealing internally — parallel map/filter/fold automatically load-balance across all cores.
- **Recursive divide-and-conquer**: Split a problem, spawn sub-tasks; work stealing ensures all cores stay busy even with uneven splits.
- **Runtime scheduler foundation**: tokio's multi-thread scheduler is work-stealing — tasks spawned on one thread can run on another if it's idle.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Work stealing | Domainslib `Task.pool` (OCaml 5) | `rayon::ThreadPool` (crossbeam-deque) |
| Per-worker queue | manual implementation | `crossbeam-deque::Worker<T>` |
| Steal operation | N/A | `crossbeam-deque::Stealer::steal()` |
| Lock-free deque | N/A | `crossbeam-deque` crate |
| Parallel iterators | `Parmap` | `rayon::par_iter()` — work-stealing backed |
