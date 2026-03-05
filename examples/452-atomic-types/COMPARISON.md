# OCaml vs Rust: Atomic Types

## Atomic Counter

### OCaml 5
```ocaml
let counter = Atomic.make 0
let () = Atomic.fetch_and_add counter 1
```

### Rust
```rust
let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Type | `'a Atomic.t` | `Atomic{Bool,Usize,Ptr,...}` |
| Ordering | Implicit SeqCst | Explicit parameter |
| Operations | Limited set | Full RMW suite |
