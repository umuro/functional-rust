📖 **[View on hightechmind.io →](https://hightechmind.io/rust/330-async-sink)**

---

# 330: Async Sink

**Difficulty:** 4  **Level:** Expert

A destination that accepts values and flushes them in batches — the write side of a stream.

## The Problem This Solves

If a `Stream` is a source you pull from, a `Sink` is a destination you push into. Real async systems constantly need to write to things that can't accept items one-by-one: databases preferring bulk inserts, log aggregators batching messages, or network sockets benefiting from coalescing writes.

The `Sink` pattern buffers incoming values and flushes when full, absorbing bursts efficiently.

## The Intuition

Think of batched database writing:
```js
buffer.push(item);
if (buffer.length >= BATCH_SIZE) await db.insertMany(buffer.splice(0));
```

Rust's `BatchSink` is the same: `send()` buffers, flush triggers when capacity is reached.

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
        if self.buffer.len() >= self.capacity { self.flush()?; }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        if !self.buffer.is_empty() {
            let batch: Vec<T> = self.buffer.drain(..).collect();
            self.flushed_batches.push(batch);
        }
        Ok(())
    }
}
```

Key: `VecDeque::drain(..)` empties the buffer in one move. Always call `flush()` at the end.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable buffer | `mutable buf: 'a list` | `VecDeque<T>` |
| Flush trigger | Manual `List.length` | `buffer.len() >= capacity` |
| Drain | `s.buf <- []` | `buffer.drain(..)` |
