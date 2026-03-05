# Flat Binary Tree in Vec — Comparison

## Core Insight
A flat tree represents a complete binary tree in an array: node `i` has children at `2i+1` and `2i+2`, parent at `(i-1)/2`. No pointers, no allocations — just arithmetic. Both languages use this identically; the difference is just array vs Vec syntax.

## OCaml Approach
- Array-based: `tree.(i)`, `tree.(2*i+1)`, etc.
- Mutable array for heapify
- `swap` and `sift_down` with imperative loops
- Level-order: iterate with doubling level sizes

## Rust Approach
- `Vec<T>` with index arithmetic
- `data.swap(i, j)` built-in
- `sift_down` with loop (no recursion needed)
- `data.get(i)` returns `Option` for safe access
- Same indexing formulas as any language

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Storage | `'a array` | `Vec<T>` |
| Access | `arr.(i)` | `vec[i]` / `vec.get(i)` |
| Swap | Manual temp var | `vec.swap(i, j)` |
| Bounds check | Runtime exception | Panic or `get()` → Option |
| Heapify | Imperative loop | Imperative loop |
| Cache locality | Good (array) | Good (Vec) |
| Use case | Same: heaps, segment trees | Same: heaps, segment trees |
