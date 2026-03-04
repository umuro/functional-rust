# 341: MPSC Channel

**Difficulty:** 3  **Level:** Advanced

Multi-producer, single-consumer channel — the standard, safe way to communicate between threads without sharing memory.

## The Problem This Solves

Multiple threads need to send results to a single collector. The naive approach — a `Vec` behind a `Mutex` — works, but every push and pop requires locking. Under high contention, threads spend more time waiting for the lock than doing actual work. And getting the teardown right (when is the Vec "full"?) requires extra coordination.

Channels solve both problems. The channel itself handles synchronization internally. Teardown is automatic: when all senders are dropped, the receiver knows the channel is exhausted and stops blocking. No extra state, no shutdown flags, no spurious wakeups.

The "multi-producer, single-consumer" design is intentional. Real-world patterns almost always have this shape: many workers generating results, one aggregator collecting them. `mpsc` captures this asymmetry in the type system — `Sender<T>` is `Clone`, `Receiver<T>` is not.

## The Intuition

Think of it as a post office. Many people (producers) can drop letters in the mailbox. One postal worker (consumer) collects and processes them. The producers don't need to coordinate with each other — they just drop the letter and leave. The consumer processes each letter as it arrives.

Go channels are the famous version of this pattern — Go made channels a language primitive. Rust's `mpsc` is a library implementation with the same concept but explicit about roles:

```go
// Go: channels are bidirectional
ch := make(chan int, 10)  // anyone can send or receive
```

```rust
// Rust: split into typed producer/consumer
let (tx, rx) = mpsc::channel::<i32>();
// tx: Sender — clone it for each producer
// rx: Receiver — only one, only one consumer
```

The Rust version makes the ownership structure explicit. In Go, you can accidentally have two goroutines reading the same channel; in Rust, the type system prevents it.

## How It Works in Rust

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn multiple producers — each gets a clone of the sender
    for i in 0..4 {
        let tx = tx.clone();  // clone before spawning
        thread::spawn(move || {
            tx.send(format!("message from worker {i}")).unwrap();
            // tx dropped here when thread finishes
        });
    }

    // Drop the original sender — otherwise rx never closes
    drop(tx);

    // Collect all messages — blocks until all senders dropped
    for msg in rx {  // rx implements IntoIterator
        println!("{msg}");
    }
}
```

`for msg in rx` is idiomatic — iterating the receiver blocks until a message arrives, and the loop ends when all senders are dropped. No manual `recv()` loop, no EOF checking.

Bounded channel: `mpsc::sync_channel(capacity)` creates a bounded channel where `send` blocks when full — useful for backpressure.

## What This Unlocks

- **Fan-in aggregation**: N workers each producing results, funneled through one channel into a single collector.
- **Decoupled pipeline stages**: Producer and consumer can run at different speeds; the channel buffers between them.
- **Clean shutdown**: Drop all senders → receiver loop terminates naturally. No shutdown flags or sentinel values needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Channel creation | `Event.new_channel ()` | `mpsc::channel()` → `(Sender<T>, Receiver<T>)` |
| Multiple producers | manual Event.send per thread | `tx.clone()` — idiomatic and safe |
| Receiver iteration | manual loop with recv | `for msg in rx` — implements `Iterator` |
| Channel closed signal | N/A | all `Sender` dropped → `recv()` returns `Err` |
| Bounded channel | `Queue.create ~capacity:n` | `mpsc::sync_channel(n)` |
