📖 **[View on hightechmind.io →](https://hightechmind.io/rust/341-mpsc-channel)**

---

# 341: MPSC Channel

## Problem Statement

Threads need to communicate without sharing memory unsafely. The MPSC (multi-producer, single-consumer) channel pattern solves thread communication by providing a typed message queue where many threads can send and one thread receives. Channels originate from Hoare's Communicating Sequential Processes (1978) and Dijkstra's work on process communication — the philosophy: don't communicate by sharing memory; share memory by communicating. Channels make data flow explicit, eliminating the need for locks around communication points and preventing entire classes of race conditions.

## Learning Outcomes

- Use `std::sync::mpsc::channel()` to create unbounded channels
- Use `mpsc::sync_channel(capacity)` for bounded/backpressure channels
- Clone `Sender<T>` to create multiple producers for one receiver
- Drop the original sender so `rx` closes when all senders are gone
- Iterate `rx.into_iter()` to collect all messages until channel closes
- Implement fan-in patterns where many threads feed one aggregator

## Rust Application

```rust
use std::sync::mpsc;
use std::thread;

pub fn fan_in<T: Send + 'static>(
    producers: Vec<Box<dyn FnOnce(Sender<T>) + Send>>
) -> Vec<T> {
    let (tx, rx) = mpsc::channel();
    for producer in producers {
        let tx = tx.clone();
        thread::spawn(move || producer(tx));
    }
    drop(tx); // drop original so rx closes when all clones finish
    rx.into_iter().collect()
}

// Bounded channel applies backpressure: sender blocks when full
pub fn bounded(capacity: usize, items: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::sync_channel::<i32>(capacity);
    thread::spawn(move || {
        for item in items { tx.send(item).unwrap(); }
    });
    rx.into_iter().collect()
}
```

`Sender<T>` is `Clone + Send`, `Receiver<T>` is `Send` but not `Clone` — the type system enforces the single-consumer constraint. Dropping all senders causes `recv()` to return `Err`, which `into_iter()` translates to `None`, cleanly terminating iteration.

## OCaml Approach

OCaml's `Event` module provides synchronous channels from CML (Concurrent ML). `Event.channel()` creates a typed channel; `Event.send` and `Event.receive` create events that `Event.sync` commits:

```ocaml
let ch = Event.channel () in
let _ = Thread.create (fun () -> Event.sync (Event.send ch 42)) () in
let v = Event.sync (Event.receive ch)
```

For async message passing, `Thread`+`Mutex`+`Queue` combinations are common, or the `Domainslib` library in OCaml 5 provides `Task.async`/`Task.await` with channels between domains.

## Key Differences

| Aspect | Rust `mpsc` | OCaml `Event` |
|--------|-------------|---------------|
| Producer count | Many (`Sender` is `Clone`) | Many (send to same channel) |
| Consumer count | One (`Receiver` not `Clone`) | One sync'd receive at a time |
| Default behavior | Asynchronous (unbounded) | Synchronous (rendezvous) |
| Bounded variant | `sync_channel(n)` | No built-in; use `Domainslib` |
| Type safety | `T: Send + 'static` | Polymorphic channel `'a Event.channel` |

## Exercises

1. **Aggregator pipeline**: Build a pipeline where 4 worker threads each transform a slice of data and send results to an aggregator thread that merges and sorts them.
2. **Backpressure demo**: Use `sync_channel(2)` with a slow consumer (`thread::sleep`) and a fast producer — observe that the producer blocks automatically.
3. **Result fan-in**: Modify `fan_in` to handle `Result<T, E>` messages, collecting successes and errors separately, without panicking on any individual failure.
