# 329: Async Streams

**Difficulty:** 4  **Level:** Expert

An iterator that yields values one by one — with the ability to pause and resume between each item.

## The Problem This Solves

In synchronous code, you pull values from an iterator instantly: `iter.next()` returns immediately. But what if each value requires waiting — for a database row, a network packet, or an event from a message queue? Calling a blocking `next()` on an async executor thread stalls the entire thread, starving all other tasks.

Async streams solve this by making the "yield the next value" step an async operation. Each poll either returns a ready value or suspends the stream, freeing the executor to do other work. This is the backbone of streaming APIs, server-sent events, log tailing, and real-time data pipelines.

The `futures::Stream` trait (and `tokio_stream`) is Rust's answer to this. In practice you consume streams with `StreamExt::next().await` in a loop — identical to an iterator but with await points between items.

## The Intuition

Think of JavaScript's `AsyncIterator` or Python's `async for`. In JavaScript:
```js
for await (const item of asyncStream) { ... }
```
Rust's equivalent (with a real async runtime) looks like:
```rust
while let Some(item) = stream.next().await { ... }
```

This example simulates the stream concept using `Iterator` and a stateful `ChunkedStream` — since the standard library has no native async stream trait (that lives in the `futures` crate). The pattern is the same: a struct holds state, and `next()` / `next_chunk()` advances it.

## How It Works in Rust

```rust
// Lazy range: generates values on demand, no allocation
struct RangeStream { current: i64, end: i64 }

impl Iterator for RangeStream {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end { None }
        else { let v = self.current; self.current += 1; Some(v) }
    }
}

// Stateful chunked stream: buffers in fixed-size windows
enum ChunkedStream { Active { data: Vec<i32>, pos: usize, sz: usize }, Done }

impl ChunkedStream {
    fn next_chunk(&mut self) -> Option<Vec<i32>> {
        match self {
            Self::Done => None,
            Self::Active { data, pos, sz } => {
                if *pos >= data.len() { *self = Self::Done; return None; }
                let end = (*pos + *sz).min(data.len());
                let chunk = data[*pos..end].to_vec();
                *pos = end;
                Some(chunk)
            }
        }
    }
}
```

The `Iterator` adapter chain — `.filter().map().take().collect()` — works lazily: no values are computed until `collect()` pulls them. In async Rust, `Stream` works identically but each `.next()` is an `.await`.

## What This Unlocks

- **Backpressure-aware data pipelines** — process only as fast as the consumer can handle, never loading the whole dataset into memory.
- **Streaming HTTP / WebSocket responses** — yield each chunk as it arrives instead of buffering the full body.
- **Infinite sequences** — model event streams, time series, or sensor data that never "ends."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy sequence type | `'a stream = Empty \| Cons of 'a * (unit -> 'a stream)` | `impl Iterator<Item=T>` (built-in trait) |
| Async iteration | `Lwt_stream.next` (Lwt) | `StreamExt::next().await` (futures crate) |
| Chunking | Recursive `take` / `drop` | `ChunkedStream` struct with owned index |
| Ownership of buffer | Shared via closures | Owned by enum state machine |
