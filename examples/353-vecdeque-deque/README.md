📖 **[View on hightechmind.io →](https://hightechmind.io/rust/353-vecdeque-deque)**

---

# 353: VecDeque Double-Ended Queue
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

`Vec` supports O(1) push/pop at the back but O(n) at the front (shifting all elements). When you need O(1) at both ends — sliding window algorithms, BFS queues, ring buffers, undo/redo stacks — `VecDeque` is the right tool. Implemented as a ring buffer (circular array), it maintains `head` and `tail` indices and wraps around, giving amortized O(1) for `push_front`, `push_back`, `pop_front`, and `pop_back`. This data structure dates back to Knuth (TAOCP Vol. 1) and is standard in every language: Python's `collections.deque`, Java's `ArrayDeque`, C++'s `std::deque`.

## Learning Outcomes

- Use `VecDeque::push_back` and `pop_front` for queue (FIFO) operations in O(1)
- Use `push_front` and `pop_back` for stack (LIFO) operations from the other end
- Implement a sliding window using `push_back` / `pop_front` on a deque
- Use `rotate_left` / `rotate_right` for efficient in-place rotation
- Convert between `VecDeque` and `Vec` with `.into_iter().collect()` or `VecDeque::from(vec)`
- Recognize `VecDeque` as the canonical BFS queue in graph traversal

## Rust Application

```rust
use std::collections::VecDeque;

pub fn sliding_window(data: &[i32], window_size: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut window: VecDeque<i32> = VecDeque::new();
    for &item in data {
        window.push_back(item);
        if window.len() > window_size {
            window.pop_front(); // O(1) — no element shifting
        }
        if window.len() == window_size {
            result.push(window.iter().cloned().collect());
        }
    }
    result
}

pub fn rotate_left<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut dq: VecDeque<_> = items.iter().cloned().collect();
    dq.rotate_left(n % items.len().max(1)); // O(min(n, len-n))
    dq.into_iter().collect()
}
```

The sliding window idiom is the canonical `VecDeque` use case: `push_back` adds the new element, `pop_front` removes the oldest. Both are O(1), making the overall algorithm O(n) regardless of window size — compared to O(n×w) if you used `Vec::remove(0)`.

## OCaml Approach

OCaml's standard library lacks a built-in deque; the `deque` package or a pair-of-lists functional deque (Hood-Melville, Okasaki) is used:

```ocaml
(* Functional deque using two lists: O(1) amortized *)
type 'a deque = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }
let push_back d x = { d with back = x :: d.back }
let pop_front d = match d.front with
  | [] -> (match List.rev d.back with
    | [] -> failwith "empty"
    | x :: rest -> (x, { front = rest; back = [] }))
  | x :: rest -> (x, { d with front = rest })
```

The two-list deque reverses the back list lazily when the front is exhausted — amortized O(1) per operation. For imperative code, `Buffer` or `Array` with manual index arithmetic mimics a ring buffer.

## Key Differences

| Aspect | Rust `VecDeque` | OCaml two-list deque |
|--------|----------------|---------------------|
| Implementation | Ring buffer (array) | Two lists |
| Cache locality | Excellent (contiguous memory) | Poor (linked list pointers) |
| All operations | O(1) amortized | O(1) amortized |
| Rotation | `rotate_left`/`rotate_right` built in | Manual via list operations |
| Index access | `dq[i]` O(1) | O(n) |

## Exercises

1. **BFS with VecDeque**: Implement breadth-first search on a graph represented as `Vec<Vec<usize>>` (adjacency list) using a `VecDeque` as the frontier queue; return the visited order.
2. **Sliding window maximum**: Given a `VecDeque<(usize, i32)>` (monotonic deque of index-value pairs), implement O(n) sliding window maximum without computing max over each window separately.
3. **Ring buffer**: Implement a fixed-capacity ring buffer using `VecDeque::with_capacity(n)` that overwrites the oldest entry when full; test with 5-element capacity and 10 insertions.
