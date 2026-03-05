# OCaml vs Rust: Enriched Categories

## Preorder (Bool-enriched)

### OCaml
```ocaml
type 'a t = { elements: 'a list; leq: 'a -> 'a -> bool }
let is_related pre a b = pre.leq a b
```

### Rust
```rust
struct Preorder<T> {
    elements: Vec<T>,
    leq: Box<dyn Fn(&T, &T) -> bool>,
}
```

## Metric ([0,∞]-enriched)

### OCaml
```ocaml
type 'a metric = 'a -> 'a -> float
```

### Rust
```rust
struct Metric<T> {
    dist: Box<dyn Fn(&T, &T) -> f64>,
}
```

## Key Insight

Both languages naturally express:
- Preorders via comparison functions
- Metrics via distance functions
- Cost graphs via edge weight maps
