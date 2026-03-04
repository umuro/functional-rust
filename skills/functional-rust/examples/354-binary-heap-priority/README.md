# 354: BinaryHeap — Priority Queue

**Difficulty:** 2  **Level:** Intermediate

Always pops the largest element first. O(1) peek, O(log n) push/pop — the go-to for Dijkstra's, task scheduling, and top-K problems.

## The Problem This Solves

You have a set of tasks where each has a priority, and you always want to process the highest-priority task next. You could use a sorted `Vec` — but every insertion requires finding the right position (O(log n) search + O(n) shift). You could re-sort the whole `Vec` after every insertion — that's O(n log n) just to add one item.

`BinaryHeap` gives you a smarter answer: insertions are O(log n) and the maximum element is always at the top, accessible in O(1). You never sort — the heap property (every parent ≥ its children) is maintained automatically on every push and pop.

The second use case is top-K selection: given a stream of a billion numbers, find the 10 largest. A max-heap of size 10 lets you push each new number and pop the minimum if the heap overflows — at O(log 10) = O(1) effective cost per element.

## The Intuition

Python's `heapq` is a min-heap (smallest first). Rust's `BinaryHeap` is a max-heap (largest first). For min-heap behavior in Rust, wrap your type in `std::cmp::Reverse<T>` — it flips the comparison.

The mental model: imagine a queue where instead of "first in, first out", the rule is "highest priority out first". The heap data structure maintains this guarantee efficiently without sorting.

## How It Works in Rust

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Max-heap: largest comes out first
let mut heap: BinaryHeap<i32> = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);
heap.push(1);
heap.push(5);

// Peek without removing — O(1)
println!("{:?}", heap.peek()); // Some(5)

// Pop largest first — O(log n)
while let Some(val) = heap.pop() {
    println!("{val}"); // 5, 4, 3, 1, 1
}

// Min-heap: wrap in Reverse to flip ordering
let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(5));
min_heap.push(Reverse(1));
min_heap.push(Reverse(3));

while let Some(Reverse(val)) = min_heap.pop() {
    println!("{val}"); // 1, 3, 5
}

// Priority queue with custom priority
#[derive(Eq, PartialEq)]
struct Task { priority: i32, name: &'static str }

// Implement Ord so highest priority value comes out first
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

let mut tasks: BinaryHeap<Task> = BinaryHeap::new();
tasks.push(Task { priority: 1, name: "low" });
tasks.push(Task { priority: 10, name: "critical" });
tasks.push(Task { priority: 5, name: "normal" });

// Processes "critical" first
while let Some(task) = tasks.pop() {
    println!("processing: {} (priority {})", task.name, task.priority);
}
```

## What This Unlocks

- **Dijkstra's shortest path**: process the node with the smallest current distance next — use a min-heap (`Reverse<(distance, node)>`).
- **Task schedulers and event loops**: always run the highest-priority or earliest-deadline task without maintaining a sorted list.
- **Top-K selection**: maintain a heap of size K; push each new element, pop if over capacity — O(n log K) over a stream of n items.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Priority queue | not in stdlib | `BinaryHeap<T>` |
| Heap type | max or min depending on impl | max-heap by default |
| Min-heap | invert comparison manually | `BinaryHeap<Reverse<T>>` |
| Peek max | manual access | `.peek()` O(1) |
| Pop max | manual | `.pop()` O(log n) |
| Python equivalent | `heapq` (min-heap) | `BinaryHeap<Reverse<T>>` for min |
