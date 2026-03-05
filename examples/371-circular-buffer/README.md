# 371: Circular Buffer / Ring Buffer

**Difficulty:** 3  **Level:** Advanced

Fixed-capacity FIFO queue where new writes overwrite the oldest data when full.

## The Problem This Solves

A sliding window of recent events: the last 1000 log lines, the last 60 seconds of sensor readings, the last N keystrokes for undo history. A `Vec` grows indefinitely and needs explicit truncation. A `VecDeque` works but doesn't enforce a capacity contract. What you want is a fixed-size buffer where pushing a new element automatically discards the oldest one — no allocation, no shifting, just a rotating write head.

Circular buffers are the classic solution for streaming data with bounded memory. They're used in network packet buffers, audio processing rings, real-time telemetry systems, and anywhere you need "most recent N" semantics without unbounded growth.

## The Intuition

A circular buffer is an array with two indices: `read_head` and `write_head`. Write advances `write_head % capacity`. Read advances `read_head % capacity`. When the buffer is full and you write, `write_head` laps `read_head`, overwriting the oldest slot. No allocation. No shifting. O(1) push and pop.

The tricky part: distinguishing "buffer is full" from "buffer is empty" — both have `read_head == write_head` naively. Solutions: track a count separately, or use `capacity + 1` slots and never fill the last one.

Rust's standard library provides `VecDeque` which is a heap-allocated ring buffer. For a fixed-capacity version that overwrites on overflow, use the `circular-buffer` crate or implement it yourself.

## How It Works in Rust

```rust
use std::collections::VecDeque;

struct RingBuffer<T> {
    buf: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self { buf: VecDeque::with_capacity(capacity), capacity }
    }

    fn push(&mut self, item: T) {
        if self.buf.len() == self.capacity {
            self.buf.pop_front(); // discard oldest
        }
        self.buf.push_back(item);
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.buf.iter()
    }
}

let mut ring = RingBuffer::new(3);
ring.push(1);
ring.push(2);
ring.push(3);
ring.push(4); // 1 is gone
// Contains: [2, 3, 4]
```

For a zero-allocation fixed-size variant on the stack:
```toml
circular-buffer = "0.1"
```

## What This Unlocks

- **Bounded log buffers** — keep last N log lines without ever allocating more.
- **Audio/video processing** — constant-size frame windows with no GC pressure.
- **Sliding window algorithms** — time-series averages, rate limiting, event debouncing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Queue | `Queue.t` (linked list, unbounded) | `VecDeque<T>` (ring buffer, growable) |
| Fixed-size queue | Manual array + indices | `RingBuffer` wrapper or `circular-buffer` crate |
| Overwrite-on-full | Manual logic | Encapsulated in `push()` method |
| Memory layout | Heap-allocated, GC-managed | `VecDeque` is heap; `[T; N]` ring is stack |
