# OCaml vs Rust: Dijkstra

## Priority Queue

### Rust
```rust
BinaryHeap<Reverse<(i32, usize)>> // min-heap via Reverse
```

### OCaml
```ocaml
module PQ = Set.Make(...) // Set as priority queue
```

Both use greedy selection of minimum distance vertex.
