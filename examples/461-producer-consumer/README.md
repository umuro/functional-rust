📖 **[View on hightechmind.io →](https://hightechmind.io/rust/461-producer-consumer)**

---

# 461: Producer-Consumer Pattern

## Problem Statement

The producer-consumer pattern separates work generation from work processing: one or more producer threads generate items at their own rate; one or more consumer threads process items at their own rate. A bounded buffer between them provides decoupling and backpressure. This is the most common inter-thread communication pattern — used in web servers (request producers, handler consumers), data pipelines (readers produce, transformers consume), and I/O systems (network receivers produce, parsers consume).

Producer-consumer appears in every concurrent system: OS kernel I/O buffers, database connection pools, message queue brokers (Kafka, RabbitMQ), and async runtime task queues.

## Learning Outcomes

- Understand the producer-consumer pattern as decoupled work generation and processing
- Learn how `mpsc::sync_channel(n)` creates a bounded buffer with backpressure
- See how multiple producers use `tx.clone()` to send to the same channel
- Understand how `drop(tx)` after spawning producers signals consumer shutdown
- Learn the `Arc<Mutex<Receiver>>` pattern for sharing a receiver across multiple consumers

## Rust Application

In `src/lib.rs`, two producer threads each send 5 items via cloned `tx`. The original `tx` is dropped after spawning to allow channel close when all producers finish. A consumer thread wraps the receiver in `Arc<Mutex<Receiver>>` for exclusive access per receive. `rx.lock().unwrap().iter().count()` collects until channel close. The test verifies all 10 items (2 producers × 5 items) are consumed.

## OCaml Approach

OCaml implements producer-consumer with `Queue.t` + `Mutex.t` + `Condition.t`: producers enqueue under lock and signal the "not empty" condition; consumers wait on the condition, dequeue under lock, and signal "not full". `Domainslib.Chan.make_bounded n` provides a ready-made bounded channel for OCaml 5.x. The pattern is the same; the implementation details differ.

## Key Differences

1. **Channel close**: Rust channels close when all senders drop — natural producer shutdown; OCaml requires sentinel values or explicit close flags.
2. **Multiple consumers**: Rust's `mpsc` supports multiple consumers via `Arc<Mutex<Receiver>>`; `crossbeam::channel` supports native MPMC.
3. **Backpressure**: `sync_channel(n)` blocks producers when buffer full; OCaml's unbounded `Queue` has no built-in backpressure.
4. **Type safety**: Rust's channel is typed (`mpsc::channel::<T>()`); OCaml's `Queue.t` is polymorphic `'a Queue.t`.

## Exercises

1. **Rate-limited producer**: Add a rate limiter to the producer: it can only send at most N items per second. Implement using `thread::sleep` and `Instant::now()`. Verify the consumer processes items at the limited rate.
2. **Multiple consumers benchmark**: Compare throughput of 1, 2, 4, 8 consumer threads processing CPU-bound work from a shared `Arc<Mutex<Receiver>>`. Plot throughput vs. consumer count to find the optimal number for your CPU.
3. **Poison pill shutdown**: Instead of relying on channel close, implement graceful shutdown with a "poison pill" sentinel: producers send a special `None` item when done; consumers exit on receiving `None` and propagate it.
