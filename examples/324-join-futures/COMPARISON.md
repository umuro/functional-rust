# OCaml vs Rust: Join Futures

## Parallel Execution

**OCaml:**
```ocaml
let parallel tasks =
  let threads = List.map (fun f -> Thread.create f ()) tasks in
  List.iter Thread.join threads
```

**Rust:**
```rust
fn join_all<T: Send + 'static>(tasks: Vec<Box<dyn FnOnce()->T+Send>>) -> Vec<T> {
    tasks.into_iter()
        .map(|f| thread::spawn(f))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Return values | None (unit) | `Vec<T>` collected |
| Thread creation | `Thread.create f ()` | `thread::spawn(f)` |
| Waiting | `Thread.join` | `handle.join()` |
| Type constraints | None | `Send + 'static` |
| Error handling | Exceptions | `Result` from join |
| Result order | N/A | Preserved |
