# 341: MPSC Channel

**Difficulty:** 3  **Level:** Advanced

Multi-producer, single-consumer channel — the standard, safe way to communicate between threads.

## The Problem This Solves

Multiple threads need to send results to a single collector. Channels handle synchronization and teardown automatically. When all senders are dropped, the receiver knows the channel is exhausted.

## How It Works in Rust

```rust
let (tx, rx) = mpsc::channel();

for i in 0..4 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(format!("message from {i}")).unwrap();
    });
}

drop(tx);  // Drop original so rx closes

for msg in rx {
    println!("{msg}");
}
```

`for msg in rx` iterates until all senders are dropped.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Channel | `Event.new_channel()` | `mpsc::channel()` |
| Multiple producers | Manual send per thread | `tx.clone()` |
| Channel closed | N/A | All senders dropped |
