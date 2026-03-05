# Priority Queue — Comparison

## Core Insight
A binary heap stores elements in array positions such that `data[i] <= data[2i+1]` and `data[i] <= data[2i+2]` (min-heap). Push appends and sifts up; pop swaps root with last, removes last, and sifts down. Both languages implement this identically. Rust also ships `BinaryHeap` (max-heap) in std; `Reverse<T>` flips ordering for min-heap use.

## OCaml Approach
- Generic via `compare: 'a -> 'a -> int` higher-order function
- `Obj.magic 0` initializes array slots (unsafe placeholder)
- Mutable record with `mutable data` and `mutable size`
- Array resizing via `Array.blit` when capacity exceeded
- `sift_up`/`sift_down` with `ref` for mutable loop index

## Rust Approach
- Generic via `T: Ord` trait bound (compiler-enforced ordering)
- `Vec<T>` grows automatically — no manual resize needed
- `data.swap(i, j)` for in-place swap
- `data.first()` for peek; `data.pop()` for removal (O(1))
- Separate `MinHeap<T>` struct and `std::collections::BinaryHeap` with `Reverse<T>`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Generics | `compare: 'a -> 'a -> int` arg | `T: Ord` trait bound |
| Storage | `'a array` (fixed, manually resized) | `Vec<T>` (auto-growing) |
| Initialize | `Obj.magic 0` (placeholder) | `Vec::new()` (empty) |
| Swap | Manual via tmp variable | `data.swap(i, j)` |
| Peek | `Some data.(0)` | `data.first()` |
| Pop remove | Manual `data.(h.size)` | `data.pop()` (Vec O(1)) |
| Sift index | `ref i` in while loop | `let mut i` in loop |
| Std heap | n/a | `BinaryHeap<T>` (max) + `Reverse<T>` |
