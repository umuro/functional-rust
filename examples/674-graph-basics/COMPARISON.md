# OCaml vs Rust: Graph Basics

## Adjacency List

### Rust
```rust
HashMap<V, Vec<V>>
```

### OCaml
```ocaml
Map.Make(V) with lists
```

Both use hash-based or tree-based maps for vertex lookup.
