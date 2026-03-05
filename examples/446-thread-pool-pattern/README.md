📖 **[View on hightechmind.io →](https://hightechmind.io/rust/446-thread-pool-pattern)**

---

# 446: Thread Pool — Reuse Threads for Amortised Concurrency

**Difficulty:** 3  **Level:** Intermediate

Pre-spawn N worker threads that drain a shared job queue — eliminate per-task thread creation overhead and cap total concurrency.

## The Problem This Solves

Spawning a thread for each unit of work has real cost: OS kernel call, stack allocation (typically 2–8 MB reserved), scheduler registration. For thousands of short tasks, this overhead dominates. A server spawning a thread per HTTP request will exhaust memory and kernel thread limits under load.

The solution is a thread pool: create N threads once at startup, then reuse them. Work items (closures) are sent through a channel; workers dequeue and execute them. When a job finishes, the thread doesn't exit — it loops back and takes the next job. Total thread count is bounded; jobs queue when all workers are busy.

The tricky part in Rust is sharing the `Receiver` among N workers. A `Receiver` is not `Clone` — `mpsc` is single-consumer by design. The solution is `Arc<Mutex<Receiver<Job>>>`: wrap the receiver so workers compete for jobs via a mutex. Exactly one worker dequeues each job; the OS scheduler naturally load-balances.

## The Intuition

A thread pool is a restaurant kitchen. Instead of hiring a chef per order (expensive, chaotic), you hire N chefs at opening and give them a ticket system. Orders go on the rail; any free chef takes the next ticket. When all chefs are busy, tickets queue. When the restaurant closes, you wait for current orders to finish, then send the chefs home.

In Java: `Executors.newFixedThreadPool(n)`. In Python: `ThreadPoolExecutor(max_workers=n)`. In Go: a buffered channel of goroutines. Rust's standard library doesn't include a thread pool, but building one from `mpsc` + `Arc<Mutex<>>` in ~30 lines illustrates the primitives cleanly. Production code uses `rayon` or `tokio`.

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    tx: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();
        // Wrap Receiver so N workers can share it
        let rx = Arc::new(Mutex::new(rx));

        let workers = (0..n).map(|_| {
            let rx = Arc::clone(&rx);
            thread::spawn(move || loop {
                // Acquire lock, dequeue one job, release lock, execute job
                match rx.lock().unwrap().recv() {
                    Ok(job) => job(),        // run the closure
                    Err(_)  => break,        // channel closed — exit
                }
            })
        }).collect();

        ThreadPool { workers, tx: Some(tx) }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.tx.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take()); // close channel — workers see Err and exit
        for w in self.workers.drain(..) {
            w.join().unwrap(); // wait for clean shutdown
        }
    }
}
```

The `Drop` impl is the shutdown protocol: drop the `Sender`, which closes the channel, which causes all `recv()` calls to return `Err`, which causes all workers to `break` their loop. Then `join()` waits for all of them. Dropping the pool blocks until all queued jobs complete.

## What This Unlocks

- **Bounded concurrency** — cap the number of simultaneous threads regardless of how many tasks are submitted.
- **Amortised thread cost** — create threads once, reuse indefinitely; ideal for high-throughput servers and batch processors.
- **Graceful shutdown** — the `Drop` + channel-close pattern ensures queued work completes and threads exit cleanly on pool destruction.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Job type | `unit -> unit` | `Box<dyn FnOnce() + Send + 'static>` |
| Shared queue | `Queue.t` + `Mutex` + `Condvar` | `mpsc::Receiver` in `Arc<Mutex<...>>` |
| Shutdown | sentinel `None` or flag | drop `Sender` — workers see `Err` on `recv` |
| Backpressure | manual queue capacity check | `mpsc::sync_channel(bound)` for bounded queue |
| Production use | `Domainslib` | `rayon`, `tokio::task::spawn_blocking` |
