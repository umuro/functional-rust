# 461: Producer-Consumer Pattern

**Difficulty:** 3  **Level:** Intermediate

Decouple data production from consumption using a bounded channel with automatic backpressure.

## The Problem This Solves

In many real systems — log ingestion, image processing pipelines, network packet handlers — data arrives faster than it can be processed. Without coordination, producers flood memory with unprocessed work until the system crashes or OOMs. You need a way for producers to slow down automatically when consumers fall behind.

The naive fix is an unbounded queue. It "works" until your server's RAM fills up under load spikes. What you really want is backpressure: the producer blocks until there's room, naturally throttling the overall system to the consumer's speed.

Beyond throttling, you also need clean shutdown. A producer that finishes its work needs to signal "no more items" without a sentinel value that every consumer must check for. In Rust, dropping all senders is that signal — the channel itself becomes the lifecycle manager.

## The Intuition

A bounded `sync_channel(N)` is a concurrent, backpressure-aware queue: when it's full, `send` blocks the producer; when it's empty, `recv` blocks the consumer; and when all senders drop, `recv` returns `Err` — clean shutdown with no extra code. The core trade-off is throughput vs. memory: smaller buffer = less memory but more blocking; larger buffer = smoother throughput but more latency under load.

## How It Works in Rust

```rust
use std::sync::mpsc;
use std::thread;

// Bounded channel: at most 4 items in flight at once
let (tx, rx) = mpsc::sync_channel::<i32>(4);

// Producer thread — blocks on send when buffer is full
let producer = thread::spawn(move || {
    for i in 0..20 {
        tx.send(i).unwrap();   // blocks if channel is full
    }
    // tx drops here → channel closes → consumers see Err
});

// Consumer thread — blocks on recv when buffer is empty
let consumer = thread::spawn(move || {
    for item in rx {           // iterator ends when all senders drop
        println!("consumed {}", item);
    }
});

producer.join().unwrap();
consumer.join().unwrap();
```

For multiple consumers, wrap `rx` in `Arc<Mutex<Receiver>>` so threads compete for items.

## What This Unlocks

- **Log/event pipelines**: collectors produce at burst speed; processors consume at sustained speed — bounded channel absorbs spikes.
- **Work-stealing thread pools**: distribute tasks to N workers without a dispatcher loop.
- **Rate-limited writes**: disk/network writer consumes at its max speed; producer auto-throttles.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bounded queue | `Queue.t` + `Mutex` + `Condition` | `mpsc::sync_channel(cap)` |
| Backpressure | Manual capacity check + wait | `SyncSender::send` blocks automatically |
| Multi-consumer | `Arc<Mutex<Queue>>` | `Arc<Mutex<Receiver>>` |
| Shutdown signal | Sentinel `None` value | Drop all `Sender`s → `recv` returns `Err` |
| Channel type | User-built | `std::sync::mpsc` (stdlib) |
