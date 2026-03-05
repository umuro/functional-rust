# Top-K Elements with BinaryHeap — Comparison

## Core Insight
The top-K problem is a classic priority queue application. Maintain a min-heap of size K; for each element, push it and evict the smallest if heap exceeds K. Result: only the K largest remain. Rust has `BinaryHeap` in std; OCaml has no built-in heap.

## OCaml Approach
- No stdlib heap — must use sorted list or manual implementation
- Sort-and-take: O(n log n) — simple but not optimal
- Bounded sorted list: maintain sorted list of size K, insert with binary-ish placement
- Functorized priority queue libraries exist but aren't in stdlib

## Rust Approach
- `BinaryHeap<T>` — max-heap by default
- `BinaryHeap<Reverse<T>>` — min-heap via newtype wrapper
- `push` + `pop` for bounded heap pattern
- `into_sorted_vec()` for sorted extraction
- `peek()` for O(1) max/min access

## Comparison Table

| Feature | OCaml | Rust (`BinaryHeap`) |
|---|---|---|
| Stdlib heap | No | Yes |
| Default order | N/A | Max-heap |
| Min-heap | N/A | `Reverse<T>` wrapper |
| Top-K complexity | O(n log n) sort | O(n log k) heap |
| Heap sort | Manual | `into_sorted_vec()` |
| Peek | N/A | `peek()` O(1) |
| Custom ordering | N/A | Implement `Ord` |
