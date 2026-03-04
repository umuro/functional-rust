# 462: Pipeline Concurrency

**Difficulty:** 3  **Level:** Intermediate

Chain N processing stages with channels so each stage runs concurrently in its own thread.

## The Problem This Solves

Multi-step data transformation is common: parse raw bytes, validate, enrich from a database, serialize to JSON. If you run these steps sequentially on each item, you're wasting CPU — stage 2 sits idle while stage 1 processes the next item. You want all stages running simultaneously, with each working on a different item in the stream.

A naive solution spawns a thread pool and runs all stages in it. But that loses ordering guarantees and makes error propagation complicated. What you want is a *pipeline*: each stage has a dedicated thread, reads from an input channel, does its work, and writes to an output channel.

The elegant property of this design is automatic shutdown propagation. When the source closes, stage 1 sees `Err` on `recv` and drops its output sender. Stage 2's `recv` then fails. The shutdown signal travels through the entire pipeline without any extra signaling code.

## The Intuition

A pipeline is N threads, each owning an `rx`/`tx` pair, chained so one thread's output is the next thread's input — the channels *are* the pipeline, and dropping propagates end-of-stream automatically. The trade-off: each stage adds latency (buffering), but the pipeline runs all stages in parallel, so throughput matches your slowest stage.

## How It Works in Rust

```rust
use std::sync::mpsc;
use std::thread;

// Stage 1: produce numbers
let (tx1, rx1) = mpsc::channel::<i32>();
thread::spawn(move || {
    for i in 0..10 { tx1.send(i).unwrap(); }
    // dropping tx1 signals end of stream to stage 2
});

// Stage 2: double each value
let (tx2, rx2) = mpsc::channel::<i32>();
thread::spawn(move || {
    for item in rx1 {          // ends when tx1 drops
        tx2.send(item * 2).unwrap();
    }
    // dropping tx2 propagates end-of-stream to stage 3
});

// Stage 3: consume and print
let handle = thread::spawn(move || {
    for item in rx2 {          // ends when tx2 drops
        println!("{}", item);
    }
});

handle.join().unwrap();
```

Use `sync_channel(N)` instead of `channel()` for bounded stages with backpressure — stage 3 being slow will throttle stage 2, which throttles stage 1.

## What This Unlocks

- **ETL pipelines**: parse → validate → transform → write, all concurrent, with natural flow control.
- **Streaming media processing**: decode → filter → encode, each stage overlapping with the next.
- **Compiler passes**: lex → parse → typecheck → codegen, stages run concurrently on different tokens/ASTs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Stage thread | Thread + two shared queues | `thread::spawn` with owned `rx` + `tx` |
| Stage connection | Shared `Queue.t` ref | `mpsc::channel` pair between stages |
| Backpressure | Manual capacity check | `sync_channel(N)` blocks automatically |
| Shutdown propagation | Sentinel value or flag | Drop final `Sender` → `Err` cascades |
| Ownership | Shared by reference | Each stage owns its channels exclusively |
