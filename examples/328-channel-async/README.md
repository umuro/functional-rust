📖 **[View on hightechmind.io →](https://hightechmind.io/rust/328-channel-async)**

---

# 328: Async Channels (mpsc)

## Problem Statement

Shared mutable state across threads requires locks — error-prone and contended. Channels provide an alternative: communicate via message passing rather than sharing state. Go popularized "don't communicate by sharing memory; share memory by communicating." Rust's `std::sync::mpsc` (multi-producer, single-consumer) channels implement this pattern with type-safe, backpressure-supporting message queues. This is the foundation for actor-based and pipeline architectures.

## Learning Outcomes

- Use `mpsc::channel()` for unbounded and `mpsc::sync_channel()` for bounded channels
- Implement fan-in: multiple producers sending to one consumer
- Handle channel closure: `Sender` drop signals end of stream to `Receiver`
- Understand backpressure: bounded channels block producers when the buffer is full

## Rust Application

Fan-in with multiple producers and one consumer:

```rust
pub fn create_producer(tx: Sender<String>, label: &'static str, count: usize) -> JoinHandle<()> {
    thread::spawn(move || {
        for i in 1..=count {
            tx.send(format!("{}-{}", label, i)).unwrap();
        }
        // tx drops here — if last sender, receiver knows stream ended
    })
}

// Drop the original tx so receiver ends when all producers finish
let (tx, rx) = mpsc::channel();
create_producer(tx.clone(), "a", 3);
create_producer(tx.clone(), "b", 3);
drop(tx);  // Critical: drop original so rx detects closure

let messages: Vec<_> = rx.into_iter().collect();
```

## OCaml Approach

OCaml uses `Event` channels or the `Domainslib` library for channel-based concurrency. Lwt provides `Lwt_stream` for async streams:

```ocaml
(* Lwt_stream: lazy push-based stream *)
let (stream, push) = Lwt_stream.create ()
let () = push (Some "message1"); push (Some "message2"); push None
let* messages = Lwt_stream.to_list stream
```

## Key Differences

1. **MPSC vs MPMC**: Rust's standard `mpsc` is multi-producer single-consumer; Tokio's `broadcast` and `watch` channels provide multi-consumer.
2. **Backpressure**: `sync_channel(capacity)` blocks the sender when full; unbounded `channel()` never blocks but may allocate unboundedly.
3. **Closure detection**: The receiver detects channel closure when all senders are dropped — `recv()` returns `Err` or `into_iter()` terminates.
4. **async channels**: `tokio::sync::mpsc` is the async-aware version — `send().await` yields instead of blocking the thread.

## Exercises

1. Implement a pipeline with three stages connected by channels: generator → transformer → aggregator.
2. Use a bounded `sync_channel` to implement backpressure: the producer should slow down when the consumer can't keep up.
3. Implement a work-stealing queue where multiple workers receive tasks from a shared channel and report results back to a collector.
