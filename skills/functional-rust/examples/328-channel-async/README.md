# 328: Async Channels (mpsc)

**Difficulty:** 3  **Level:** Advanced

Multi-producer, single-consumer channels let multiple tasks send messages to one receiver — the safe, idiomatic way to communicate between concurrent workers.

## The Problem This Solves

You have multiple threads (or async tasks) producing data — log entries, events, computation results — and you need to funnel them all into one place for processing. Shared mutable state (`Arc<Mutex<Vec<T>>>`) works but requires locking on every access, which creates contention. Channels are the alternative: no shared state, no locks, just message passing.

The "multi-producer, single-consumer" design reflects real usage: many workers generate results, one aggregator collects them. The `Sender` is cheap to clone, so any number of producers can hold one. The `Receiver` is not cloneable — exactly one place in your code processes incoming messages.

When all `Sender`s are dropped, the channel closes and `recv()` returns an error, giving the consumer a clean signal to stop. This makes teardown easy and correct.

## The Intuition

Go channels (`chan T`) are the most famous version of this pattern. The Rust `mpsc` is similar but more explicit about roles: `Sender<T>` and `Receiver<T>` are distinct types, and the asymmetry (many senders, one receiver) is baked into the API.

```go
// Go: bidirectional channel, anyone can send or receive
ch := make(chan int)
go func() { ch <- 42 }()
val := <-ch
```

```rust
// Rust: explicit producer/consumer split
let (tx, rx) = mpsc::channel::<i32>();
let tx2 = tx.clone();  // clone for second producer
thread::spawn(move || tx.send(42).unwrap());
thread::spawn(move || tx2.send(99).unwrap());
let val = rx.recv().unwrap();  // only one receiver
```

In async Rust (tokio), `tokio::sync::mpsc` provides the awaitable version — same concept, non-blocking.

## How It Works in Rust

```rust
fn producer(tx: mpsc::Sender<String>, label: &'static str, n: usize, delay_ms: u64) {
    thread::spawn(move || {
        for i in 1..=n {
            thread::sleep(Duration::from_millis(delay_ms));
            tx.send(format!("{label}-{i}")).unwrap();  // send returns Err if receiver dropped
        }
        // tx dropped here — this producer is done
    });
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    producer(tx.clone(), "A", 3, 10);   // clone tx for each producer
    producer(tx.clone(), "B", 3, 15);
    drop(tx);  // drop the original — without this, rx.into_iter() would hang forever

    let msgs: Vec<String> = rx.into_iter().collect();  // blocks until all senders dropped
}
```

`drop(tx)` after cloning is the pattern. You clone before spawning each producer, then drop the original. When the last producer finishes and its cloned `tx` drops, the channel closes and `rx.into_iter()` completes.

## What This Unlocks

- **Work distribution**: Producers push jobs onto the channel; a consumer thread processes them in order without coordination.
- **Result aggregation**: Spawn N parallel workers, each sends results back through the channel; collect in arrival order.
- **Event buses**: Multiple sources emit events into one channel; a single handler processes them sequentially, avoiding race conditions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Create channel | `Event.new_channel ()` | `mpsc::channel()` returns `(Sender, Receiver)` |
| Send | `Event.send chan x` | `tx.send(val)` → `Result` (Err if receiver gone) |
| Receive | `Event.receive chan` | `rx.recv()` → `Result` (Err if all senders dropped) |
| Multiple producers | manual synchronization | `tx.clone()` — `Sender` is `Clone` |
| Channel close signal | N/A | all senders dropped → `recv()` returns `Err` |
