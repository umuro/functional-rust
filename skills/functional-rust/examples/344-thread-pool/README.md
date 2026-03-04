# 344: Thread Pool

**Difficulty:** 4  **Level:** Expert

Spawn N threads once, reuse them for many tasks — eliminate thread creation overhead and bound total resource usage.

## The Problem This Solves

Spawning an OS thread for every task sounds reasonable until you're handling 10,000 requests per second. Thread creation costs 10–100µs and ~1–8MB of stack. Spawning a new thread per task becomes the bottleneck — you spend more time creating and destroying threads than doing actual work.

Thread pools fix this by amortizing thread creation: spawn N threads at startup, keep them alive forever, send work to them via a channel. A thread finishes one task and immediately starts the next. No creation overhead, predictable memory usage, controllable parallelism.

The pool size is usually tied to CPU count (`num_cpus::get()` or `std::thread::available_parallelism()`). More threads than cores doesn't help for CPU-bound work — you'd just be context switching. For I/O-bound work, you can go higher.

## The Intuition

Thread pools exist in every language's standard library or ecosystem because this pattern is so fundamental:

- Python: `concurrent.futures.ThreadPoolExecutor(max_workers=4)`
- Java: `Executors.newFixedThreadPool(4)`
- Go: doesn't need this because goroutines are cheap; `GOMAXPROCS` controls parallelism
- Rust std: no built-in pool, but `rayon` provides one; or build your own with ~20 lines

The Rust manual implementation is instructive because it shows exactly how thread pools work: a channel is the job queue, each thread is a loop pulling from that queue. The "pool" is just N threads all reading from the same channel.

```
Main thread:    [job1] [job2] [job3] [job4] [job5] [job6]
                   ↓ channel ↓
Worker 1:     job1 ──── job4 ──────────
Worker 2:     job2 ─── job5 ──────────  (run in parallel)
Worker 3:     job3 ──────── job6 ──────
```

## How It Works in Rust

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        // Wrap receiver in Arc<Mutex> so all workers can share it
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size).map(|id| {
            let rx = Arc::clone(&receiver);
            thread::spawn(move || {
                loop {
                    // Lock to receive a job, then release lock before running it
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => { println!("worker {id} running job"); job(); }
                        Err(_) => break,  // sender dropped — shut down
                    }
                }
            })
        }).collect();

        Self { workers, sender }
    }

    fn execute(&self, job: impl FnOnce() + Send + 'static) {
        self.sender.send(Box::new(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // sender is dropped here → workers receive Err → break out of loop
    }
}
```

The `Arc<Mutex<Receiver>>` is the key insight: multiple threads share one receiver, but the Mutex ensures only one thread calls `recv()` at a time. Whichever thread gets the job runs it; the others wait for the next one.

Releasing the lock *before* running the job (`let job = rx.lock().unwrap().recv()`) is important — if you held the lock during execution, only one worker could run at a time, defeating the purpose.

## What This Unlocks

- **Web servers**: rayon-style parallel request processing with bounded resource usage.
- **Batch computation**: Process a large dataset in parallel — split into chunks, send to pool, collect results.
- **Async runtime foundation**: tokio's thread pool is this pattern extended with work-stealing for better load balancing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread pool | `Domain_pool` (OCaml 5) or Domainslib | manual or `rayon::ThreadPool` |
| Shared work queue | `Mutex` + `Queue.t` | `Arc<Mutex<Receiver<Job>>>` |
| Job type | `unit -> unit` function | `Box<dyn FnOnce() + Send + 'static>` |
| Graceful shutdown | manual signal | drop `Sender` → workers exit loop |
| Parallel iterators | Parmap | `rayon::par_iter()` (uses thread pool) |
