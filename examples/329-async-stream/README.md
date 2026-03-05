# 329: Async Streams

**Difficulty:** 4  **Level:** Expert

An iterator that yields values one by one — with the ability to pause and resume between each item.

## The Problem This Solves

In synchronous code, `iter.next()` returns immediately. But what if each value requires waiting — for a database row, network packet, or event? Async streams make the "yield next value" step an async operation.

This is the backbone of streaming APIs, server-sent events, log tailing, and real-time data pipelines.

## The Intuition

Think of JavaScript's `AsyncIterator` or Python's `async for`:
```js
for await (const item of asyncStream) { ... }
```

Rust's equivalent:
```rust
while let Some(item) = stream.next().await { ... }
```

## How It Works in Rust

```rust
struct RangeStream { current: i64, end: i64 }

impl Iterator for RangeStream {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end { None }
        else { let v = self.current; self.current += 1; Some(v) }
    }
}

// Stateful chunked stream
enum ChunkedStream { Active { data: Vec<i32>, pos: usize, sz: usize }, Done }

impl ChunkedStream {
    fn next_chunk(&mut self) -> Option<Vec<i32>> {
        // ... yields chunks until done
    }
}
```

The iterator adapter chain — `.filter().map().take().collect()` — works lazily.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy sequence | `'a stream` ADT | `impl Iterator<Item=T>` |
| Async iteration | `Lwt_stream.next` | `StreamExt::next().await` |
| Chunking | Recursive functions | Stateful enum |
