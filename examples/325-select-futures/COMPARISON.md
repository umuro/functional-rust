# OCaml vs Rust: Select/Race Futures

## Racing Tasks

**OCaml:**
```ocaml
let race tasks =
  let ch = Event.new_channel () in
  List.iter (fun f ->
    ignore (Thread.create (fun () -> Event.sync (Event.send ch (f ()))) ())
  ) tasks;
  Event.sync (Event.receive ch)
```

**Rust:**
```rust
fn race<T: Send + 'static>(
    tasks: Vec<(Box<dyn FnOnce()->T+Send>, &'static str)>
) -> (&'static str, T) {
    let (tx, rx) = mpsc::channel();
    for (f, label) in tasks {
        let tx = tx.clone();
        thread::spawn(move || { let _ = tx.send((label, f())); });
    }
    rx.recv().unwrap()
}
```

## Timeout Pattern

**OCaml (with Lwt):**
```ocaml
Lwt_unix.with_timeout 5.0 (fun () -> slow_operation ())
```

**Rust:**
```rust
fn with_timeout<T: Send + 'static>(f: Box<dyn FnOnce()->T+Send>, ms: u64) -> Option<T> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    rx.recv_timeout(Duration::from_millis(ms)).ok()
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Channel type | `Event.channel` | `mpsc::channel` |
| Receive first | `Event.sync (Event.receive ch)` | `rx.recv()` |
| Timeout | `Lwt_unix.with_timeout` | `recv_timeout` |
| Loser cleanup | GC | Threads continue (can be ignored) |
