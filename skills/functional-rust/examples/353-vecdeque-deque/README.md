# 353: VecDeque — Double-Ended Queue

**Difficulty:** 2  **Level:** Intermediate

O(1) push and pop from both ends — a ring buffer that replaces Vec when you need a queue or sliding window.

## The Problem This Solves

You're implementing BFS and use a `Vec` as your queue. `push` (enqueue) goes to the back — O(1). `remove(0)` (dequeue) goes from the front — O(n), because every remaining element shifts left. With a million-node graph, that shift dominates your runtime.

The same issue bites sliding windows: add a new element to the right, remove the oldest from the left. With `Vec`, every left-removal is O(n). With `VecDeque`, both ends are O(1) because it uses a ring buffer — a circular array where the "start" and "end" pointers wrap around without moving data.

A third case: maintaining a fixed-size history buffer. Add new entries to the back and pop the oldest from the front as the buffer fills. `VecDeque` is exactly this, and it doesn't allocate extra memory for the rotation.

## The Intuition

Python's `collections.deque` is the direct equivalent. Both are ring buffers: a contiguous memory block where the logical start and end float around, so push/pop at either end just moves a pointer instead of shifting data.

The tradeoff vs `Vec`: random indexing (`deque[i]`) is still O(1) but has a small constant overhead (two-part index calculation). If you never touch the front, use `Vec`. If you ever need O(1) front operations, use `VecDeque`.

## How It Works in Rust

```rust
use std::collections::VecDeque;

let mut dq: VecDeque<i32> = VecDeque::new();

// Push to back (like Vec::push)
dq.push_back(1);
dq.push_back(2);
dq.push_back(3);

// Push to front — O(1), impossible with Vec in O(1)
dq.push_front(0);

// Pop from front — the queue operation (FIFO)
while let Some(val) = dq.pop_front() {
    println!("{val}"); // 0, 1, 2, 3
}

// BFS pattern
let mut queue: VecDeque<usize> = VecDeque::new();
queue.push_back(start_node);
while let Some(node) = queue.pop_front() {   // O(1) dequeue
    for &neighbor in &graph[node] {
        queue.push_back(neighbor);           // O(1) enqueue
    }
}

// Sliding window of size k
let mut window: VecDeque<i32> = VecDeque::new();
for &x in &data {
    window.push_back(x);
    if window.len() > k {
        window.pop_front(); // evict oldest element, O(1)
    }
}

// Convert from Vec when you realize you need front-ops
let v = vec![1, 2, 3];
let dq: VecDeque<i32> = v.into_iter().collect();
```

## What This Unlocks

- **BFS / level-order traversal**: queue semantics (push back, pop front) in O(1) — the canonical use case.
- **Sliding window algorithms**: add right, remove left in O(1); no index arithmetic, no shifting.
- **Work-stealing queues**: one thread pushes to the back, another steals from the front, both O(1).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Double-ended queue | `Queue` module (linked list) | `VecDeque<T>` (ring buffer) |
| Push to front | O(1) for linked lists | O(1) `.push_front()` |
| Pop from front | O(1) for linked lists | O(1) `.pop_front()` |
| Random access | O(n) for linked lists | O(1) indexing |
| Memory layout | pointer-chased nodes | contiguous ring buffer |
| Python equivalent | `collections.deque` | `VecDeque<T>` |
