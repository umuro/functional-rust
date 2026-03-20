📖 **[View on hightechmind.io →](https://hightechmind.io/rust/354-binary-heap-priority)**

---

# 354: BinaryHeap Priority Queue

## Problem Statement

Many algorithms need to repeatedly extract the element with the highest (or lowest) priority: Dijkstra's shortest path, A* search, Huffman coding, task schedulers, and event-driven simulations. A priority queue with O(log n) insert and O(log n) extract-max is the standard data structure for these needs. Rust's `BinaryHeap` implements a max-heap — the largest element is always at the top. The `Reverse<T>` wrapper flips the comparison, turning the max-heap into a min-heap. This covers both "top N largest" and "top N smallest" queries efficiently.

## Learning Outcomes

- Build a `BinaryHeap<T>` from a slice and extract elements with `heap.pop()` (always max)
- Use `Reverse<T>` to create a min-heap without a separate data structure
- Use `into_sorted_vec()` for O(n log n) heap sort producing ascending order
- Understand that building a heap from `n` elements is O(n) via heapify
- Recognize when `BinaryHeap` is appropriate vs `BTreeMap`/`sort`
- Use `BinaryHeap` as a k-way merge structure for merging sorted streams

## Rust Application

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn top_n<T: Ord + Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut heap: BinaryHeap<_> = items.iter().cloned().collect(); // O(n)
    (0..n).filter_map(|_| heap.pop()).collect() // O(n log n)
}

pub fn bottom_n<T: Ord + Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut heap: BinaryHeap<Reverse<T>> = items.iter().cloned().map(Reverse).collect();
    (0..n).filter_map(|_| heap.pop().map(|Reverse(x)| x)).collect()
}

pub fn heap_sort<T: Ord>(items: Vec<T>) -> Vec<T> {
    let heap: BinaryHeap<_> = items.into_iter().collect();
    heap.into_sorted_vec() // ascending order
}
```

`BinaryHeap::into_sorted_vec()` returns elements in ascending order (smallest first) even though the heap is a max-heap internally — it sorts the backing array in-place. For `top_n`, popping from a max-heap gives the largest elements in descending order.

## OCaml Approach

OCaml's standard library lacks a binary heap; functional priority queues use a leftist tree or pairing heap:

```ocaml
(* Using the heap module from the containers library *)
module H = CCHeap.Make(struct type t = int let leq a b = a <= b end)

let top_n items n =
  let h = List.fold_left (fun h x -> H.add h x) H.empty items in
  let rec pop h acc = function
    | 0 -> List.rev acc
    | k -> match H.pop h with
      | None -> List.rev acc
      | Some (x, h') -> pop h' (x :: acc) (k - 1)
  in
  pop h [] n
```

For imperative use, `Array`-based heap sort is common in competitive programming. The `psq` (priority search queue) package provides both priority and key-based access.

## Key Differences

| Aspect | Rust `BinaryHeap` | OCaml (functional heap) |
|--------|------------------|-------------------------|
| Default order | Max-heap | Depends on `leq` comparator |
| Min-heap | `Reverse<T>` wrapper | Flip `leq` |
| Sorted output | `into_sorted_vec()` ascending | `fold` with pop |
| Build cost | O(n) heapify | O(n log n) for functional |
| Mutability | In-place | Persistent (new structure per op) |

## Exercises

1. **K-way merge**: Merge 3 sorted `Vec<i32>` streams efficiently using `BinaryHeap<(i32, usize)>` where the `usize` is the stream index; produce one merged sorted output.
2. **Task scheduler**: Implement a priority task queue where tasks have `priority: u8` and `name: String`; process tasks in priority order (highest first) using `BinaryHeap<(u8, String)>`.
3. **Running median**: Maintain two heaps (max-heap for lower half, min-heap for upper half via `Reverse`) to compute the running median as each number is inserted; the median is always accessible in O(1).
