📖 **[View on hightechmind.io →](https://hightechmind.io/rust/450-crossbeam-channel)**

---

# 450: Crossbeam Channels — Advanced Channel Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

`std::sync::mpsc` provides basic MPSC (multiple producer, single consumer) channels but lacks bounded backpressure, multiple consumers, and select across channels. `crossbeam::channel` provides both bounded (`crossbeam::channel::bounded(n)`) and unbounded channels with MPMC (multiple producer, multiple consumer) semantics. Bounded channels implement backpressure — the producer blocks when the buffer is full — preventing fast producers from overwhelming slow consumers with unbounded memory growth.

Crossbeam channels are used in high-performance task queues, trading systems, event brokers, and any pipeline where multiple workers process from the same channel.

## Learning Outcomes

- Understand bounded channels: `sync_channel(n)` blocks sender when buffer is full
- Learn the backpressure mechanism: slow consumers throttle fast producers
- See how `mpsc::sync_channel` simulates crossbeam bounded channels
- Understand MPMC vs. MPSC: multiple consumers enable worker pool patterns
- Learn how channel capacity affects latency, throughput, and memory usage

## Rust Application

In `src/lib.rs`, `bounded_channel_demo` uses `mpsc::sync_channel(capacity)` — Rust's standard bounded channel. The producer sends all messages, blocking when the buffer is full. The consumer thread receives until the channel closes. This demonstrates the backpressure mechanism: if the consumer is slower than the producer, the producer is throttled to `capacity` items ahead. `crossbeam::channel::bounded` would enable multiple consumers on the same channel (MPMC).

## OCaml Approach

OCaml's `Event` module provides synchronous (zero-buffer) channels — the send always blocks until a receiver is ready. `Domainslib.Chan.make_bounded n` creates bounded channels for OCaml 5.x domains. `Async.Pipe` and `Lwt_stream` provide bounded buffered streams for async OCaml. The backpressure semantics are similar but integrated with their respective runtimes.

## Key Differences

1. **MPMC**: `crossbeam::channel` supports multiple consumers; `std::sync::mpsc` does not (MPSC only).
2. **Select**: `crossbeam::select!` enables waiting on multiple channels; `std::sync::mpsc` has no select.
3. **Bounded semantics**: Both `mpsc::sync_channel` and `crossbeam::bounded` block the sender when full; they differ in MPMC support.
4. **Performance**: `crossbeam::channel` uses more sophisticated lock-free algorithms vs. `mpsc`'s mutex-based bounded channel.

## Exercises

1. **Worker pool with bounded channel**: Create a bounded work queue (`bounded(100)`) and spawn 4 worker threads, all consuming from the same channel. Verify that all work items are processed exactly once.
2. **Backpressure test**: Measure how bounded vs. unbounded channels affect peak memory usage when a producer sends 1M large messages and a consumer processes one every millisecond.
3. **Timeout send**: Using `crossbeam::channel` or a custom wrapper, implement `try_send_timeout(val, timeout)` that attempts to send but returns `Err(val)` if the channel doesn't have capacity within the timeout.
