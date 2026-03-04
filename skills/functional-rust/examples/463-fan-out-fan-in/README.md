# 463: Fan-Out / Fan-In

**Difficulty:** 3  **Level:** Intermediate

Distribute work to N parallel workers (fan-out) and collect their results into one channel (fan-in) — the map-reduce pattern.

## The Problem This Solves

You have a list of slow operations: HTTP requests, database lookups, image resizes. Running them sequentially is wasteful — your CPU and network sit idle most of the time. You want to run them concurrently, but with a bounded number of workers, not one goroutine per item.

The temptation is a `thread::spawn` for every item. That works for 100 items; it crashes or thrashes for 100,000. You need a worker pool: fixed N threads, each pulling items from a shared queue, with results flowing into a single collector. This is fan-out (one task stream → N workers) + fan-in (N result streams → one receiver).

The subtle part is fan-in: `mpsc` is naturally built for it. Each worker gets a *clone* of the same `Sender` — they all funnel results into one `Receiver`. When all workers finish (all `Sender` clones drop), the receiver's loop ends cleanly.

## The Intuition

Fan-out shares work by giving N workers access to the same work queue; fan-in collects results by giving all workers clones of one `Sender` — `mpsc` is naturally multi-producer, so fan-in is free. The trade-off: more workers = more parallelism but more scheduling overhead; tune N to your bottleneck (CPU-bound → N=cores, I/O-bound → N can be much larger).

## How It Works in Rust

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

let items: Vec<i32> = (0..20).collect();

// Fan-out: wrap work queue in Arc<Mutex> so workers compete for items
let work = Arc::new(Mutex::new(items.into_iter()));

// Fan-in: single receiver collects all results
let (tx, rx) = mpsc::channel::<i32>();

let handles: Vec<_> = (0..4).map(|_| {
    let work = Arc::clone(&work);
    let tx = tx.clone();          // each worker gets its own Sender clone
    thread::spawn(move || {
        loop {
            let item = work.lock().unwrap().next();
            match item {
                Some(n) => tx.send(n * n).unwrap(),   // process and send result
                None => break,                         // no more work, exit
            }
        }
        // tx drops here — one fewer sender
    })
}).collect();

drop(tx); // drop the original sender so rx ends when all workers finish

// Collect all results
let results: Vec<i32> = rx.iter().collect();

for h in handles { h.join().unwrap(); }
```

## What This Unlocks

- **Parallel HTTP requests**: N workers each fetching a URL, results merged into one stream.
- **MapReduce**: fan-out maps over partitions; fan-in collects partial results for a final reduce.
- **Bounded thread pools**: process M items with N workers without spawning M threads.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fan-out distribution | Shared `Queue.t` + N threads | `Arc<Mutex<Iterator>>` — workers compete |
| Fan-in collection | N threads → shared result queue | N `Sender` clones → one `Receiver` |
| Result ordering | Manual index tracking | `(id, result)` tuples |
| Worker count | User-managed | `(0..N).map(...)` |
| Shutdown detection | Sentinel value | All `Sender` clones drop → `recv` returns `Err` |
