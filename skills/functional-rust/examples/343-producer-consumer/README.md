# 343: Producer-Consumer Pattern

**Difficulty:** 3  **Level:** Advanced

Producers push work onto a shared channel; consumers pull and process it — decoupled, concurrent, and naturally backpressured with a bounded channel.

## The Problem This Solves

You're processing a stream of incoming work: web requests, log entries, uploaded files, sensor readings. The work arrives faster than you can process it, and you have two choices: drop items (bad), or buffer them and process concurrently (good).

The naive approach — process each item inline as it arrives — limits throughput to what one thread can do. The other extreme — spawn a new thread per item — works until you have thousands of items queued and thousands of threads consuming memory and scheduler time.

Producer-consumer with a bounded channel is the middle path: N worker threads processing items, a channel buffering between arrival and processing. When the buffer fills, producers naturally slow down (backpressure). Workers run continuously at full speed. The system self-regulates.

## The Intuition

This is one of the oldest patterns in concurrent programming, and every language has it:

```python
# Python
import queue, threading
q = queue.Queue(maxsize=10)
def producer(): q.put(item)
def consumer(): item = q.get(); process(item)
```

```rust
// Rust — almost identical, but type-safe and compile-checked
let (tx, rx) = mpsc::sync_channel(10);  // bounded: backpressure built in
// producer thread: tx.send(item)
// consumer thread: for item in rx { process(item) }
```

The Rust version gives you one extra guarantee for free: the compiler ensures only one thread owns the `Receiver`. No accidental dual-consumer races — a common bug in Python/Java implementations where two threads both call `queue.get()`.

In Go, this is channels with goroutines. The shape is identical; Go just has syntactic sugar for channels (`<-` operator) while Rust uses method calls.

## How It Works in Rust

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::sync_channel::<i32>(10);  // bounded buffer of 10

    // Producers: 3 threads each send 5 items
    let producers: Vec<_> = (0..3).map(|id| {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..5 {
                let item = id * 100 + i;
                tx.send(item).unwrap();  // blocks if buffer full (backpressure!)
                println!("produced: {item}");
            }
        })
    }).collect();

    // Drop the original tx — so rx closes when all producer clones drop
    drop(tx);

    // Consumer: single thread processes all items
    let consumer = thread::spawn(move || {
        let mut count = 0;
        for item in rx {  // blocks waiting for items, ends when all senders drop
            println!("consumed: {item}");
            count += 1;
        }
        count
    });

    for p in producers { p.join().unwrap(); }
    let total = consumer.join().unwrap();
    println!("Processed {total} items");
}
```

`mpsc::sync_channel(10)` creates a bounded channel — `send` blocks when the buffer has 10 unprocessed items. This is backpressure: fast producers can't overwhelm slow consumers, they just wait.

`mpsc::channel()` (unbounded) never blocks on send but uses unlimited memory if consumers fall behind. Use bounded channels in production.

## What This Unlocks

- **Async job queues**: Web server spawns one producer per request, N consumer workers process jobs — scale workers independently of request rate.
- **Streaming ETL pipelines**: Read CSV (producer) → transform (consumer) → write to DB (next consumer) — each stage decoupled.
- **Rate-limited processing**: Bounded channel naturally limits how far ahead producers can run, giving you flow control without explicit rate limiters.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Channel type | `Queue.t` + `Mutex` | `mpsc::sync_channel(n)` (built-in bounded) |
| Backpressure | manual with semaphore | `sync_channel(n)` — `send` blocks when full |
| Multiple producers | share `Queue.t` via mutex | `tx.clone()` per producer |
| Consumer loop | while loop with `Queue.pop` | `for item in rx` — iterator protocol |
| Channel close | sentinel value | drop all `Sender`s — receiver loop ends |
