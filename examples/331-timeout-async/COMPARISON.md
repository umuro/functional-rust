# OCaml vs Rust: Timeout Async

## Timeout Pattern

**OCaml (Lwt):**
```ocaml
let with_timeout timeout f =
  Lwt.pick [
    f ();
    Lwt_unix.sleep timeout >>= fun () -> Lwt.fail Timeout
  ]
```

**Rust:**
```rust
fn with_timeout<T>(timeout: Duration, f: impl FnOnce() -> Result<T, E>) -> Result<T, TimeoutError<E>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    rx.recv_timeout(timeout)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Async timeout | `Lwt.pick` with sleep | `tokio::time::timeout` |
| Sync timeout | Busy-wait loop | `recv_timeout` |
| Error type | Exception | Enum variant |
| Cancellation | `Lwt.cancel` | Future dropped |
