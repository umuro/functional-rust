# 330: Async Sink

**Difficulty:** 4  **Level:** Expert

A destination that accepts values and flushes them in batches — the write side of a stream.

## The Problem This Solves

If a `Stream` is a source you pull from, a `Sink` is a destination you push into. In real async systems you constantly need to write to something that can't accept items one-by-one at full speed: a database that prefers bulk inserts, a log aggregator that batches messages, a network socket that benefits from coalescing writes, or a metrics collector that flushes every N samples.

Writing directly to these sinks on every item is expensive. The `Sink` pattern buffers incoming values and flushes when the buffer is full (or on explicit flush), absorbing bursts without hammering the downstream. In async Rust, `futures::Sink` gives you `send(item).await` and `flush().await` — the sink decides internally when to actually write.

This example implements the same pattern synchronously using `VecDeque` as a buffer, showing the core logic without a runtime dependency.

## The Intuition

Think of it like a batched database writer in Node.js:
```js
// Don't do one INSERT per row — buffer and flush
buffer.push(item);
if (buffer.length >= BATCH_SIZE) await db.insertMany(buffer.splice(0));
```

Rust's `BatchSink` is the same idea: `send()` buffers, flush triggers when capacity is reached. The async version (from `futures::SinkExt`) wraps `send` in an async call so the flush — which might wait for I/O — doesn't block the executor.

## How It Works in Rust

```rust
struct BatchSink<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    flushed_batches: Vec<Vec<T>>,
}

impl<T: Clone> BatchSink<T> {
    fn send(&mut self, item: T) -> Result<(), String> {
        self.buffer.push_back(item);
        // Auto-flush when buffer reaches capacity
        if self.buffer.len() >= self.capacity { self.flush()?; }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        if !self.buffer.is_empty() {
            // drain() moves items out without cloning
            let batch: Vec<T> = self.buffer.drain(..).collect();
            self.flushed_batches.push(batch);
        }
        Ok(())
    }
}
```

Key: `VecDeque::drain(..)` empties the buffer in one move. Always call `flush()` explicitly at the end — the last partial batch won't auto-flush if `len < capacity`.

## What This Unlocks

- **Batched database writes** — collect rows and `INSERT` in bulk, cutting round-trips by 10–100×.
- **Log aggregation** — buffer log lines and send to Elasticsearch / Loki in batches.
- **Backpressure integration** — pair with a `Stream` to build a full push-through pipeline with flow control.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable buffer | `mutable buf: 'a list` (reversed for O(1) prepend) | `VecDeque<T>` (O(1) both ends) |
| Flush trigger | Manual check on `List.length` | `buffer.len() >= capacity` |
| Drain | `s.buf <- []` (GC frees old list) | `buffer.drain(..)` (moves into new `Vec`) |
| Error handling | Unit / exception | `Result<(), String>` |
