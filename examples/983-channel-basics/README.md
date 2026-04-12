**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[channel-basics on hightechmind.io](https://hightechmind.io/posts/functional-rust/channel-basics)

---

## Problem Statement

Introduce Rust's `std::sync::mpsc` (Multi-Producer Single Consumer) channels for message-passing between threads. Implement a single-producer/consumer pair, a multi-producer/single-consumer pattern using `tx.clone()`, and a bounded channel using `mpsc::sync_channel`. Channels enforce ownership transfer — the sender gives up ownership of each sent value.

## Learning Outcomes

- Create `(Sender<T>, Receiver<T>)` pairs with `mpsc::channel()`
- Spawn a producer thread with `move` closure that owns the `Sender` end
- Consume messages with `rx.iter()` which loops until all senders are dropped
- Clone `Sender` for multiple producers: `let tx2 = tx.clone()`
- Use `mpsc::sync_channel(capacity)` for a bounded channel with backpressure

## Rust Application

```rust
fn single_producer_consumer() -> Vec<i32> {
    let (tx, rx) = mpsc::channel::<i32>();

    let producer = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
        // tx drops here — closes the channel
    });

    let results: Vec<i32> = rx.iter().collect();
    producer.join().unwrap();
    results
}

fn multi_producer_consumer() -> Vec<i32> {
    let (tx, rx) = mpsc::channel::<i32>();

    let handles: Vec<_> = (0..3)
        .map(|batch| {
            let tx = tx.clone();
            thread::spawn(move || {
                let start = batch * 10 + 1;
                for i in start..=start + 2 {
                    tx.send(i).unwrap();
                }
            })
        })
        .collect();

    drop(tx);  // drop the original tx so rx.iter() terminates
    let mut results: Vec<i32> = rx.iter().collect();
    results.sort();
    handles.into_iter().for_each(|h| h.join().unwrap());
    results
}
```

`rx.iter()` blocks and returns one item per iteration, terminating when all `Sender` clones are dropped. In the multi-producer case, the original `tx` must be explicitly `drop`ped because only producer threads' `tx` clones drive the iteration — the original `tx` in the main thread holds the channel open otherwise.

Channels transfer ownership: `tx.send(value)` moves `value` into the channel. The consumer receives an owned `T`. This prevents data races without additional synchronization.

## OCaml Approach

```ocaml
(* OCaml: Event module (stdlib) for synchronous channels *)
let ch = Event.new_channel ()

let producer () =
  List.iter (fun i ->
    Event.sync (Event.send ch i)
  ) [1;2;3;4;5]

let consumer () =
  let rec loop acc =
    let v = Event.sync (Event.receive ch) in
    loop (v :: acc)
  in
  (* OCaml Event is synchronous — no buffer *)
  loop []

(* Practical: use Lwt_stream or Domainslib.Chan for async/parallel *)
let (stream, push) = Lwt_stream.create ()
let push_items () =
  List.iter (fun i -> push (Some i)) [1;2;3;4;5];
  push None
```

OCaml's standard `Event` module provides synchronous channels (rendezvous — no buffer). For buffered async channels, `Lwt_stream` or `Domainslib.Chan` are the practical choices.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Channel type | `mpsc` — multi-producer, single-consumer | `Event.channel` — synchronous rendezvous |
| Buffer | Unbounded (`channel`) or bounded (`sync_channel`) | Zero-buffer (synchronous) |
| Multiple senders | `tx.clone()` — reference counted `Sender` | `Event.channel` is already shared |
| Channel close | All senders dropped | No built-in close; use sentinel value |
| Ownership transfer | Moved into channel | Shared via GC |

`mpsc` channels are a safe, efficient alternative to shared-memory concurrency. They enforce a clear ownership model: each value has exactly one owner at any time, moving from producer to consumer via the channel.

## Exercises

1. Implement a producer that sends `Option<T>` and a consumer that stops on `None` — a sentinel-based close signal.
2. Use `mpsc::sync_channel(8)` to implement bounded backpressure and observe how the producer blocks when the buffer is full.
3. Implement a fan-out: one producer, multiple consumers each with their own `Receiver` (requires cloning messages with `Arc<T>`).
4. Implement a pipeline with three stages connected by two channels: `Stage1 -> chan1 -> Stage2 -> chan2 -> Stage3`.
5. Benchmark `mpsc` throughput vs `Mutex<VecDeque<T>>` for 1,000,000 messages.
