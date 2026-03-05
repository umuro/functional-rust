# Queue Using VecDeque — Comparison

## Core Insight
FIFO queues need efficient operations at both ends. Rust's `VecDeque` (ring buffer) provides O(1) for both. OCaml offers a mutable `Queue` module or the functional two-list queue with amortized O(1) dequeue.

## OCaml Approach
- `Queue` module: mutable, `push`/`pop`/`peek`
- Functional alternative: `{ inbox; outbox }` two-list queue
- Amortized O(1) via lazy reversal when outbox empties
- BFS naturally uses `Queue.push`/`Queue.pop`

## Rust Approach
- `VecDeque<T>`: `push_back` (enqueue), `pop_front` (dequeue)
- `front()` for peek without removal
- Ring buffer — true O(1), not amortized
- Optional wrapper struct for semantic clarity
- No functional queue in std — use `im` crate for persistent queues

## Comparison Table

| Feature | OCaml (`Queue`) | Rust (`VecDeque`) |
|---|---|---|
| Enqueue | `Queue.push` | `push_back` |
| Dequeue | `Queue.pop` | `pop_front` |
| Peek | `Queue.peek` | `front()` |
| Complexity | O(1) | O(1) |
| Mutability | Mutable | Mutable |
| Implementation | Doubly-linked | Ring buffer |
| Functional alt | Two-list queue | None in std |
