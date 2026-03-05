# OCaml vs Rust: Spawning Tasks

## Spawn Worker

**OCaml:**
```ocaml
let spawn_worker id delay =
  Thread.create (fun () ->
    Thread.delay delay;
    Printf.sprintf "worker-%d done" id
  ) ()
```

**Rust:**
```rust
fn spawn_worker(id: usize, delay_ms: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        format!("worker-{id} done after {delay_ms}ms")
    })
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Return type | `Thread.t` | `JoinHandle<T>` |
| Thread creation | `Thread.create f ()` | `thread::spawn(closure)` |
| Getting result | N/A (thread returns unit) | `handle.join().unwrap()` |
| Move semantics | Implicit | Explicit `move` |
| Static lifetime | Not required | `'static` required |
