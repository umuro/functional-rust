**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[priority-queue on hightechmind.io](https://hightechmind.io/posts/functional-rust/priority-queue)

---

## Problem Statement

Implement a min-heap priority queue from scratch and compare it with Rust's standard `BinaryHeap` (max-heap). The manual min-heap uses sift-up on push and sift-down on pop. Also demonstrate the `Reverse<T>` wrapper pattern for converting the standard max-heap into a min-heap.

## Learning Outcomes

- Implement `MinHeap<T: Ord>` using a `Vec<T>` with 0-indexed heap invariant
- Implement `sift_up`: while parent > current, swap and move up
- Implement `sift_down`: compare with both children, swap with smaller, repeat
- Use `BinaryHeap::push(Reverse(x))` for a min-heap with the standard library
- Understand why `BinaryHeap` is a max-heap by default and when `Reverse` is appropriate

## Rust Application

```rust
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self { MinHeap { data: Vec::new() } }
    pub fn size(&self) -> usize { self.data.len() }
    pub fn peek(&self) -> Option<&T> { self.data.first() }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() { return None; }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let top = self.data.pop();
        if !self.data.is_empty() { self.sift_down(0); }
        top
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else { break; }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        loop {
            let (left, right) = (2 * i + 1, 2 * i + 2);
            let mut smallest = i;
            if left < self.data.len() && self.data[left] < self.data[smallest] { smallest = left; }
            if right < self.data.len() && self.data[right] < self.data[smallest] { smallest = right; }
            if smallest == i { break; }
            self.data.swap(i, smallest);
            i = smallest;
        }
    }
}
```

Heap indexing: root at index 0, children of node `i` at `2*i+1` (left) and `2*i+2` (right), parent of `i` at `(i-1)/2`. `sift_up` restores the heap property after `push`; `sift_down` restores it after `pop` (which swaps root with last element).

`BinaryHeap` with `Reverse`: `heap.push(Reverse(x))` stores `Reverse(x)` which implements `Ord` in reverse order, so the standard max-heap pops the minimum value of `x` first. Extract with `heap.pop().map(|Reverse(x)| x)`.

## OCaml Approach

```ocaml
(* OCaml stdlib: module Heap does not exist; use a sorted list or custom *)
(* Simple min-heap as an array *)
let create () = { data = Array.make 64 0; size = 0 }

let parent i = (i - 1) / 2
let left i = 2 * i + 1
let right i = 2 * i + 2

let swap h i j =
  let tmp = h.data.(i) in
  h.data.(i) <- h.data.(j);
  h.data.(j) <- tmp

let rec sift_up h i =
  if i > 0 && h.data.(i) < h.data.(parent i) then begin
    swap h i (parent i);
    sift_up h (parent i)
  end

(* Standard library alternative *)
(* module PQ = Set.Make(Int) — sorted set acts like priority queue *)
```

OCaml's standard library has `Set.Make(M)` which provides O(log n) add and min/max removal — equivalent to a priority queue but without duplicate support. For duplicates, `Map.Make(Int)` with count values works.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Standard PQ | `BinaryHeap` (max-heap) | No built-in; `Set.Make` for unique elements |
| Min-heap standard | `Reverse<T>` wrapper | `Set.Make` with reversed compare |
| Array access | `self.data[i]` | `h.data.(i)` |
| Index arithmetic | Same 0-indexed formula | Same |

## Exercises

1. Implement `heapify(vec: Vec<T>) -> MinHeap<T>` that builds the heap in O(n) using Floyd's algorithm (sift-down from `n/2-1` to `0`).
2. Implement heap sort: build a max-heap, repeatedly pop to produce sorted output.
3. Use `BinaryHeap<Reverse<(i32, usize)>>` to implement Dijkstra's shortest path algorithm.
4. Implement `decrease_key` — update the priority of an existing element without removing and re-inserting.
5. Implement a `k`-way merge using a min-heap: merge `k` sorted arrays into one sorted array in O(n log k).
