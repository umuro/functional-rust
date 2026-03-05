# OCaml vs Rust: Channel Select

## Select from Multiple Channels

### OCaml (Event module)
```ocaml
let msg = Event.sync (Event.choose [
  Event.receive ch1;
  Event.receive ch2
])
```

### Rust (Polling)
```rust
loop {
    if let Ok(v) = rx1.try_recv() { return First(v); }
    if let Ok(v) = rx2.try_recv() { return Second(v); }
    thread::sleep(Duration::from_millis(1));
}
```

## Key Differences

| Feature | OCaml | Rust (std) |
|---------|-------|------------|
| Select primitive | `Event.choose` | Poll loop or crossbeam-channel |
| Blocking select | Yes | No (use crossbeam) |
| Timeout | Manual | `recv_timeout` |
