# OCaml vs Rust: Async Mutex

## Basic Mutex

**OCaml:**
```ocaml
let m = Mutex.create () in
Mutex.lock m;
(* critical section *)
Mutex.unlock m
```

**Rust:**
```rust
let m = Mutex::new(0);
{
    let mut guard = m.lock().unwrap();
    *guard += 1;
} // guard drops, lock released
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Lock/unlock | Explicit methods | RAII guard |
| Poison | Not possible | `PoisonError` on panic |
| Data association | Separate from mutex | Mutex wraps data |
| Async version | `Lwt_mutex` | `tokio::sync::Mutex` |
