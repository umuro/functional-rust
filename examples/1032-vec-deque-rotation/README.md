📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1032-vec-deque-rotation)**

---

# 1032-vec-deque-rotation — VecDeque Rotation
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A double-ended queue (deque) supports efficient insertion and removal at both ends. Ring buffers — fixed-size deques used for circular logs, producer-consumer queues, and audio sample buffers — are a classic application. Rotation (shifting all elements left or right by k positions) requires O(1) with a proper deque but O(n) with a plain `Vec`.

Rust's `VecDeque` is a ring-buffer backed deque providing O(1) amortized push and pop at both ends. Its `rotate_left` and `rotate_right` methods are O(min(k, n-k)), making rotation efficient.

## Learning Outcomes

- Use `VecDeque` for O(1) push and pop at both ends
- Apply `rotate_left` and `rotate_right` for efficient rotation
- Implement a sliding window using `VecDeque` as a fixed-size buffer
- Use `VecDeque` as a BFS queue
- Convert between `Vec` and `VecDeque`

## Rust Application

`src/lib.rs` covers four operations. `basic_deque` shows push/pop at both ends. `rotation` demonstrates `rotate_left(2)` and `rotate_right(2)` as inverse operations. `sliding_window` maintains a fixed-size window by pushing to the back and popping from the front — the classic ring buffer pattern. `deque_as_queue` shows BFS-style processing where elements are enqueued at the back and dequeued from the front.

The sliding window pattern appears in network packet processing, time-series smoothing, and audio/video buffering.

## OCaml Approach

OCaml's standard library lacks a deque, but `Queue` provides a FIFO queue with O(1) add and take:

```ocaml
let sliding_window data window_size =
  let q = Queue.create () in
  List.map (fun x ->
    Queue.add x q;
    if Queue.length q > window_size then ignore (Queue.pop q);
    Queue.fold (+) 0 q  (* sum of window *)
  ) data
```

The `Base.Deque` module and `containers` library provide full double-ended queues with rotation.

## Key Differences

1. **Rotation**: Rust's `VecDeque::rotate_left` is a built-in method; OCaml requires manual element movement or a custom implementation.
2. **Ring buffer**: `VecDeque` is backed by a ring buffer with a start offset; OCaml's `Queue` uses a linked list.
3. **Index access**: `VecDeque` supports `O(1)` index access via `deque[i]`; OCaml's `Queue` is sequential-access only.
4. **Conversion**: Rust provides `VecDeque::from(vec)` and `vec.into()` for zero-copy conversion; OCaml requires explicit iteration.

## Exercises

1. Implement a `RateLimiter` that tracks requests using a `VecDeque<Instant>`, sliding the window to count requests in the last N seconds.
2. Write `rotate_string(s: &str, k: usize) -> String` using `VecDeque<char>` to rotate a string's characters.
3. Implement a fixed-capacity circular log buffer `CircularLog<T>` backed by `VecDeque<T>` that keeps only the last N entries.
