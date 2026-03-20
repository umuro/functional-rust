📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1040-queue-via-deque)**

---

# 1040-queue-via-deque — Queue Using VecDeque

## Problem Statement

A queue (FIFO — first in, first out) is fundamental to breadth-first search, task scheduling, message passing, and producer-consumer patterns. Implementing a queue efficiently requires O(1) enqueue at one end and O(1) dequeue at the other.

`Vec<T>` with `push` at the back and `remove(0)` at the front is O(n) dequeue — each removal shifts all remaining elements. `VecDeque<T>`, backed by a ring buffer, provides O(1) at both ends and is the correct queue implementation in Rust.

## Learning Outcomes

- Understand why `Vec::remove(0)` is O(n) and unsuitable for queues
- Use `VecDeque::push_back` and `pop_front` for O(1) FIFO operations
- Implement a queue wrapper with ergonomic `enqueue`/`dequeue` methods
- Apply a queue to BFS traversal
- Know when to use `VecDeque` directly vs a wrapper

## Rust Application

`src/lib.rs` wraps `VecDeque<T>` in a `Queue<T>` struct with `enqueue` (push_back), `dequeue` (pop_front), `peek` (front), `is_empty`, and `len`. A BFS function demonstrates the queue in a realistic algorithm context: each discovered node's neighbors are enqueued, and nodes are processed FIFO to ensure shortest-path-first traversal.

In production Rust, `VecDeque` is used directly without a wrapper — the method names `push_back` and `pop_front` are self-documenting. A wrapper adds value in APIs where you want to enforce FIFO discipline and prevent indexed access.

## OCaml Approach

OCaml's standard `Queue` module provides a mutable FIFO queue backed by a doubly-linked list:

```ocaml
let q = Queue.create ()
Queue.add 1 q  (* enqueue *)
Queue.push 2 q  (* also enqueue — Queue.push is an alias *)
let x = Queue.pop q  (* dequeue, raises Empty if empty *)
let y = Queue.take_opt q  (* dequeue returning option *)
```

OCaml's `Queue.add` is O(1); `Queue.pop` is O(1). The linked-list backing means more memory overhead per element than Rust's ring buffer.

## Key Differences

1. **Backing structure**: Rust's `VecDeque` is a ring buffer (cache-friendly, contiguous memory); OCaml's `Queue` is a doubly-linked list.
2. **Method names**: Rust uses `push_back`/`pop_front`; OCaml uses `add`/`pop` or `push`/`take`.
3. **Capacity hints**: Rust's `VecDeque::with_capacity(n)` pre-allocates; OCaml's `Queue.create()` starts empty with no pre-allocation hint.
4. **Index access**: Rust's `VecDeque` supports `O(1)` index access; OCaml's `Queue` is sequential-access only.

## Exercises

1. Implement a bounded queue `BoundedQueue<T>` that blocks (returns `Err`) when full, suitable for backpressure in a pipeline.
2. Write a level-order tree traversal using the queue.
3. Implement `Queue::drain_all<F: Fn(T)>(&mut self, f: F)` that processes and removes all elements in FIFO order.
