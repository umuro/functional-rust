📖 **[View on hightechmind.io →](https://hightechmind.io/rust/445-mpsc-channel)**

---

# 445: MPSC Channels — Message Passing Between Threads
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Shared mutable state with locks is error-prone: deadlocks, priority inversion, and complex lock ordering. The alternative is message passing: threads communicate by sending values through channels, with no shared state. `std::sync::mpsc` (Multiple Producer, Single Consumer) provides channels for this pattern. Producers send messages; the consumer receives them. When all senders drop, the receiver's iteration automatically ends — a natural shutdown mechanism.

MPSC channels power the actor model, pipeline processing, result aggregation from worker threads, and the "channel as work queue" pattern used in thread pools.

## Learning Outcomes

- Understand the MPSC channel contract: multiple senders, one receiver, bounded or unbounded
- Learn how `tx.clone()` creates additional senders for multiple producer threads
- See how `drop(tx)` signals shutdown — the channel closes when all senders drop
- Understand `rx.iter()` for collecting all messages until channel close
- Learn the relationship between `mpsc` and Go's channels (Go's are MPMC with select)

## Rust Application

In `src/lib.rs`, `multi_producer_single_consumer` creates a channel with `mpsc::channel()`. Each producer thread clones the `tx` sender, sends messages, then drops its clone when done. Crucially, `drop(tx)` drops the original sender — the channel only closes when all senders (original + clones) are dropped. `rx.iter()` blocks until the channel closes, collecting all messages. The test verifies the correct total message count.

## OCaml Approach

OCaml's `Event` module provides synchronous channels: `let ch = Event.new_channel()`, `Event.sync (Event.send ch v)` blocks until a receiver is ready. The `Thread_safe_queue` from `Core` provides asynchronous buffered queues. OCaml 5.x's `Domainslib.Chan` provides a task pool with channels. Unlike Rust's `mpsc`, OCaml's built-in channel primitives are more primitive and require more assembly for complex patterns.

## Key Differences

1. **MPSC vs. MPMC**: Rust's `std::sync::mpsc` is multiple-producer, single-consumer; Go's channels are MPMC. For MPMC in Rust, use `crossbeam::channel`.
2. **Shutdown signal**: Rust channels close when all senders drop — automatic shutdown; OCaml requires explicit sentinel values or condition variables.
3. **Bounded vs. unbounded**: `mpsc::channel()` is unbounded (back-pressure requires explicit management); `mpsc::sync_channel(n)` creates bounded channels.
4. **Select**: Rust's `mpsc` has no select; `crossbeam::channel` + `crossbeam::select!` enable multi-channel receive.

## Exercises

1. **Pipeline with channels**: Build a three-stage pipeline: stage 1 produces numbers 1-1000, stage 2 squares them, stage 3 filters evens. Connect stages with `mpsc` channels. Verify the final output.
2. **Fan-out and fan-in**: Create a work distributor: one sender distributes work items to N workers via N channels. Each worker sends results back via a single results channel. Verify all items are processed.
3. **Backpressure with sync_channel**: Replace `mpsc::channel()` with `mpsc::sync_channel(10)`. Producer threads will block when the buffer is full. Verify the producer is throttled when the consumer is slow (add a `thread::sleep` in the consumer).
