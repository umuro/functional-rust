# OCaml vs Rust: Async Channels

## Producer Pattern

**OCaml:**
```ocaml
let prod label n =
  Thread.create (fun () ->
    for i = 1 to n do
      Event.sync (Event.send ch (Printf.sprintf "%s-%d" label i))
    done
  ) ()
```

**Rust:**
```rust
fn producer(tx: mpsc::Sender<String>, label: &'static str, n: usize) {
    thread::spawn(move || {
        for i in 1..=n {
            tx.send(format!("{label}-{i}")).unwrap();
        }
    });
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Channel creation | `Event.new_channel ()` | `mpsc::channel()` |
| Send operation | `Event.sync (Event.send ch x)` | `tx.send(x)` |
| Clone sender | N/A | `tx.clone()` |
| Close signal | Manual | All senders dropped |
| Error handling | Exceptions | `Result` type |
