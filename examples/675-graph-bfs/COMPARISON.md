# OCaml vs Rust: BFS

## Rust (imperative)
```rust
let mut queue = VecDeque::from([start]);
while let Some(v) = queue.pop_front() { ... }
```

## OCaml (functional)
```ocaml
let rec loop visited queue acc = match queue with
  | [] -> List.rev acc
  | v :: rest -> ...
```

Key: Rust uses mutable collections, OCaml uses recursion with accumulator.
