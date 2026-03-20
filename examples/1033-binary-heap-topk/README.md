📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1033-binary-heap-topk)**

---

# 1033-binary-heap-topk — Top-K with BinaryHeap

## Problem Statement

Finding the k largest (or smallest) elements from a large stream is a common problem in data processing: top-10 most visited pages from a billion-row log, top-k nearest neighbors in machine learning, k most recent events. Sorting the entire dataset is O(n log n), but maintaining a heap of size k gives O(n log k) — much better when k is small relative to n.

Rust's `BinaryHeap` is a max-heap by default. For top-k (keeping the k largest), use a min-heap of size k with `Reverse<T>`: when a new element is larger than the heap minimum, evict the minimum and insert the new element.

## Learning Outcomes

- Understand max-heap vs min-heap semantics in Rust
- Use `Reverse<T>` to turn `BinaryHeap` into a min-heap
- Implement the top-k algorithm with O(n log k) time complexity
- Use `BinaryHeap` as a priority queue for task scheduling
- Compare sorting vs heap approaches for small k vs large k

## Rust Application

`src/lib.rs` implements `top_k` using a `BinaryHeap<Reverse<i32>>` of exactly size k. For each new element, push it and then pop the minimum if the heap exceeds size k. After processing all elements, the heap contains exactly the k largest. The result is sorted descending for a clean output. `priority_queue_demo` shows `BinaryHeap` as a priority queue: elements are popped in descending order regardless of insertion order.

Priority queues built on `BinaryHeap` are used in Dijkstra's algorithm, A* search, and task schedulers.

## OCaml Approach

OCaml lacks a standard heap. The algorithmic pattern uses sorting or a functional priority queue:

```ocaml
(* Simple O(n log n) approach via sort *)
let top_k k data =
  List.sort (fun a b -> compare b a) data
  |> (fun sorted -> List.filteri (fun i _ -> i < k) sorted)
```

The `containers` library provides `CCHeap` and `CCFQueue` (functional priority queues) for the O(n log k) approach.

## Key Differences

1. **Max vs min**: Rust's `BinaryHeap` is a max-heap by default; a min-heap requires `Reverse<T>`. OCaml's `CCHeap` takes a custom comparison function.
2. **Mutable vs functional**: Rust's `BinaryHeap` is mutable; OCaml's functional priority queues (`CCFQueue`) are persistent.
3. **Standard library**: `BinaryHeap` is in Rust's std; OCaml requires a third-party crate for heap structures.
4. **`Reverse` wrapper**: Rust's `Reverse<T>` inverts the `Ord` comparison without allocating; OCaml inverts comparison via a custom functor argument.

## Exercises

1. Implement `bottom_k(k: usize, data: &[i32]) -> Vec<i32>` (k smallest elements) using a max-heap instead of a min-heap.
2. Write a `kth_largest(k: usize, data: &[i32]) -> Option<i32>` function that returns only the kth largest element without collecting all k.
3. Implement a task scheduler using `BinaryHeap<(Priority, Task)>` where higher priority numbers are processed first.
