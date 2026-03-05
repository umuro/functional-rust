📖 **[View on hightechmind.io →](https://hightechmind.io/rust/328-channel-async)**

---

# 328: Async Channels (mpsc)

**Difficulty:** 3  **Level:** Advanced

Multi-producer, single-consumer channels let multiple tasks send messages to one receiver — the safe, idiomatic way to communicate between concurrent workers.

## The Problem This Solves

You have multiple threads producing data and need to funnel them all into one place. Shared mutable state requires locking on every access. Channels are the alternative: no shared state, no locks, just message passing.

When all `Sender`s are dropped, the channel closes and `recv()` returns an error, giving the consumer a clean signal to stop.

## The Intuition

Go channels (`chan T`) are the famous version of this pattern. Rust `mpsc` is similar but more explicit: `Sender<T>` and `Receiver<T>` are distinct types.

```rust
let (tx, rx) = mpsc::channel::<i32>();
let tx2 = tx.clone();  // clone for second producer
thread::spawn(move || tx.send(42).unwrap());
thread::spawn(move || tx2.send(99).unwrap());
let val = rx.recv().unwrap();
```

## How It Works in Rust

```rust
fn producer(tx: mpsc::Sender<String>, label: &'static str, n: usize) {
    thread::spawn(move || {
        for i in 1..=n {
            tx.send(format!("{label}-{i}")).unwrap();
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    producer(tx.clone(), "A", 3);
    producer(tx.clone(), "B", 3);
    drop(tx);  // important: drop original so rx knows when to stop
    let msgs: Vec<String> = rx.into_iter().collect();
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Create channel | `Event.new_channel ()` | `mpsc::channel()` |
| Send | `Event.send chan x` | `tx.send(val)` |
| Receive | `Event.receive chan` | `rx.recv()` |
| Multiple producers | manual sync | `tx.clone()` |
