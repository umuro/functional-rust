📖 **[View on hightechmind.io →](https://hightechmind.io/rust/445-mpsc-channel)**

---

# 445: MPSC Channels — Message Passing Between Threads

**Difficulty:** 3  **Level:** Intermediate

Send values across threads with `std::sync::mpsc` — multiple producers, one consumer, with automatic shutdown when all senders drop.

## The Problem This Solves

Shared mutable state (`Arc<Mutex<T>>`) is one concurrency model, but it requires every thread to coordinate on access. It scales poorly when threads have different roles: producers that generate work and a consumer that processes it. Shared state forces both sides to synchronise on every operation, creating contention.

The alternative is **message passing**: producers don't share data with the consumer — they send owned values through a channel. No locks, no shared memory, no coordination beyond the channel itself. The consumer processes messages one at a time in a clean sequential loop. This is the model Go popularised with goroutines and channels, and it maps directly to actor systems (Erlang, Akka).

The critical operational question is: when does the consumer stop? With shared state you need a sentinel value or an external flag. With `mpsc`, the answer is elegant: when all `Sender` clones are dropped, the channel closes and `recv()` returns `Err`. The consumer loop exits naturally. No sentinel, no flag, no race on "was the last message sent?".

## The Intuition

A `mpsc` channel is a thread-safe queue. The `Sender` end can be cloned and given to as many threads as you like — they all push values in. The `Receiver` end is unique — only one consumer. Values arrive in FIFO order (though producers interleave non-deterministically). `recv()` blocks; `try_recv()` and `try_iter()` don't.

In Python: `queue.Queue()` with `put`/`get`. In Go: `ch := make(chan T)` with `ch <- v` and `<-ch`. The Rust version gives you type safety (the channel carries a specific `T`) and automatic close signaling via drop.

## How It Works in Rust

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel::<String>();

// Multiple producers — clone the Sender for each thread
let handles: Vec<_> = (0..3).map(|id| {
    let tx = tx.clone(); // clone increments sender count
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("p{}-msg{}", id, i)).unwrap();
        }
        // tx drops here — sender count decremented
    })
}).collect();

// Drop the original tx — channel closes when ALL clones drop
drop(tx);

// for-loop on Receiver: iterates until channel closes
for msg in rx {
    println!("got: {}", msg);
}
// Loop exits when last Sender drops — no sentinel needed

for h in handles { h.join().unwrap(); }

// Non-blocking drain — collect all buffered messages
let (tx2, rx2) = mpsc::channel::<u32>();
for i in 0..5 { tx2.send(i).unwrap(); }
drop(tx2);
let all: Vec<u32> = rx2.try_iter().collect(); // non-blocking
```

The crucial line is `drop(tx)` after cloning. If you forget it, the channel never closes — `for msg in rx` loops forever waiting for the original sender that will never send.

## What This Unlocks

- **Producer-consumer pipelines** — worker threads generate results; a single collector thread aggregates them without any shared mutable state.
- **Fan-out work queues** — main thread sends jobs to a `Receiver` shared via `Arc<Mutex<Receiver<Job>>>` among a pool of workers (see example 446).
- **Event-driven loops** — a background thread sends events to the main thread's `rx` loop, enabling clean separation of concerns.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Channel creation | `Queue.create ()` + manual Mutex + Condvar | `let (tx, rx) = mpsc::channel()` |
| Send | push + signal | `tx.send(v).unwrap()` |
| Receive (blocking) | `Condition.wait` | `rx.recv().unwrap()` |
| Receive (non-blocking) | manual `try` | `rx.try_recv()` or `rx.try_iter()` |
| Shutdown signal | sentinel `None` or external flag | drop all `Sender` clones — `recv()` returns `Err` |
| Multiple producers | manual clone/synchronise | `tx.clone()` — built-in |
