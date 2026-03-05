# OCaml vs Rust: MPSC Channel

## Channel Usage

**OCaml:**
```ocaml
let ch = Event.new_channel () in
Event.sync (Event.send ch value)
```

**Rust:**
```rust
let (tx, rx) = mpsc::channel();
tx.send(value).unwrap();
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Split sender/receiver | No | Yes (`tx`, `rx`) |
| Clone sender | N/A | `tx.clone()` |
| Iterate receiver | Manual loop | `for msg in rx` |
| Close detection | Manual | Automatic on sender drop |
