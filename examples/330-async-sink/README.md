📖 **[View on hightechmind.io →](https://hightechmind.io/rust/330-async-sink)**

---

# 330: Async Sink — Buffered Writing

## Problem Statement

Writing individual items to disk, network, or databases one at a time is inefficient. Batching writes — accumulating items in a buffer and flushing when the buffer is full or a flush is explicitly requested — is the standard optimization. The `Sink` trait (in the `futures` crate) is the write-side complement to `Stream`: it accepts items and provides backpressure when the buffer is full. Understanding the buffering and flushing lifecycle is essential for high-throughput I/O.

## Learning Outcomes

- Understand a `Sink` as a destination that accepts items and controls backpressure
- Implement a `BatchSink` that buffers items and flushes in configurable-size batches
- Recognize the lifecycle: `send()` (add item), auto-flush when full, `flush()` for explicit drain
- Apply batching to reduce I/O overhead in database writes and network sends

## Rust Application

`BatchSink<T>` buffers items and flushes when capacity is reached:

```rust
pub struct BatchSink<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    flushed_batches: Vec<Vec<T>>,
}

impl<T: Clone> BatchSink<T> {
    pub fn send(&mut self, item: T) {
        self.buffer.push_back(item);
        if self.buffer.len() >= self.capacity {
            self.flush(); // Auto-flush when buffer is full
        }
    }
    pub fn flush(&mut self) {
        if !self.buffer.is_empty() {
            self.flushed_batches.push(self.buffer.drain(..).collect());
        }
    }
}
```

## OCaml Approach

OCaml's `Buffer` module provides in-memory buffering, and `Lwt_io.flush` drains IO buffers. For custom batch logic, a mutable `Queue.t` serves as the accumulator:

```ocaml
let batch_sink capacity =
  let buffer = Queue.create () in
  let flush () = (* send batch *) Queue.clear buffer in
  let send item =
    Queue.add item buffer;
    if Queue.length buffer >= capacity then flush ()
  in (send, flush)
```

## Key Differences

1. **Futures Sink trait**: `futures::Sink` provides `poll_ready`, `start_send`, `poll_flush`, `poll_close` — a four-phase protocol for async backpressure.
2. **Backpressure**: A sync `BatchSink` blocks the producer inline; an async sink uses `poll_ready` → `Pending` to signal "not ready" without blocking.
3. **Production use**: Kafka producers, Elasticsearch bulk indexers, and PostgreSQL batch inserters all use this buffering pattern.
4. **Flush on drop**: Sinks should flush remaining items when dropped — implement `Drop` to flush the residual buffer.

## Exercises

1. Add `Drop` to `BatchSink<T>` that flushes any remaining buffered items when the sink is dropped.
2. Implement a `LogSink` that batches log messages and writes them to a writer in configurable-size chunks.
3. Add a high-watermark threshold: when the buffer reaches 80% capacity, start emitting backpressure signals.
