# OCaml vs Rust: Borrow Checker

## OCaml
```ocaml
(* No borrow checking — ref cells for mutation *)
let v = ref [1; 2; 3]
let r1 = !v
let r2 = !v
(* No restrictions *)
```

## Rust
```rust
// Rule 1: Multiple & OR one &mut, not both
let mut v = vec![1, 2, 3];
let r1 = &v;  // shared borrow
let r2 = &v;  // OK: multiple shared
// v.push(4); // ERROR: can't mutate while borrowed

// After r1, r2 last use:
v.push(4);    // OK: borrows ended
```

## Key Differences

1. **OCaml**: ref cells provide interior mutability, GC safety
2. **Rust**: Borrow checker enforces at compile time
3. **Rust**: Either N readers OR 1 writer
4. **Rust**: Prevents data races by design
5. Both: Memory-safe, different enforcement
