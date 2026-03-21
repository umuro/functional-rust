📖 **[View on hightechmind.io →](https://hightechmind.io/rust/349-broadcast-channel)**

---

# 349: Broadcast Channel
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Publish-subscribe systems need one sender to deliver the same message to multiple independent receivers — event buses, real-time dashboards, multi-client notification systems. Standard MPSC channels route each message to exactly one receiver. A broadcast channel delivers every message to every subscriber. This pattern powers WebSocket fan-out (one server event → all connected clients), log streaming (one log source → multiple sinks), and reactive frameworks (one state change → all observers). Tokio provides `tokio::sync::broadcast` natively; this example shows the mechanics of building one from primitives.

## Learning Outcomes

- Implement a broadcast sender that maintains a list of per-subscriber `SyncSender<T>`
- Protect the subscriber list with `Arc<Mutex<Vec<SyncSender<T>>>>`
- Clone the message to each subscriber (requires `T: Clone`)
- Use bounded `sync_channel` per subscriber to apply per-subscriber backpressure
- Handle slow subscribers without blocking the sender (use `try_send`, drop on lag)
- Recognize the difference between fan-out (broadcast) and fan-in (mpsc aggregation)

## Rust Application

```rust
use std::sync::{mpsc, Arc, Mutex};

pub struct BroadcastSender<T: Clone + Send + 'static> {
    subscribers: Arc<Mutex<Vec<mpsc::SyncSender<T>>>>,
}

impl<T: Clone + Send + 'static> BroadcastSender<T> {
    pub fn new() -> Self {
        Self { subscribers: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn subscribe(&self, buf: usize) -> BroadcastReceiver<T> {
        let (tx, rx) = mpsc::sync_channel(buf);
        self.subscribers.lock().unwrap().push(tx);
        BroadcastReceiver { rx }
    }

    pub fn send(&self, msg: T) {
        let subs = self.subscribers.lock().unwrap();
        for sub in subs.iter() {
            let _ = sub.try_send(msg.clone()); // drop silently if receiver is full
        }
    }
}
```

`try_send` instead of `send` ensures a slow subscriber doesn't block the broadcaster — it silently drops messages if the subscriber's buffer is full (lag policy). Tokio's `broadcast::Sender` uses a circular ring buffer and tracks the "lag" for each receiver explicitly, returning `RecvError::Lagged(n)` when messages are missed.

## OCaml Approach

OCaml's event-driven approach uses a mutable list of callbacks:

```ocaml
type 'a broadcaster = {
  mutable subscribers: ('a -> unit) list;
  mutex: Mutex.t;
}

let broadcast b msg =
  Mutex.lock b.mutex;
  let subs = b.subscribers in
  Mutex.unlock b.mutex;
  List.iter (fun f -> f msg) subs

let subscribe b f =
  Mutex.lock b.mutex;
  b.subscribers <- f :: b.subscribers;
  Mutex.unlock b.mutex
```

This callback model is common in OCaml GUI frameworks (LablGTK signals) and Lwt (reactive streams). The functional approach avoids channels entirely — each subscriber is a callback closure.

## Key Differences

| Aspect | Rust channel broadcast | OCaml callback broadcast |
|--------|----------------------|--------------------------|
| Message delivery | Queued in per-subscriber channel | Immediate callback invocation |
| Backpressure | Per-subscriber buffer limit | None (callbacks run synchronously) |
| Lag handling | Drop messages or `RecvError::Lagged` | No buffering — caller's responsibility |
| Type safety | `T: Clone + Send + 'static` | Polymorphic callback `'a -> unit` |
| Unsubscribe | Remove dead senders from list | Remove callback from list |

## Exercises

1. **Dead subscriber cleanup**: After each `send`, remove subscribers whose `try_send` returned `Err(Full)` or `Err(Disconnected)` — prune the subscriber list to prevent unbounded growth.
2. **Tokio broadcast**: Replace the manual implementation with `tokio::sync::broadcast::channel(16)`; test that 3 subscribers each receive 5 messages in order, and observe `RecvError::Lagged` when a slow receiver falls behind.
3. **Filtered subscription**: Add a `subscribe_filtered(buf, predicate)` method that only delivers messages matching a condition to that subscriber; implement by wrapping the sender in a `move |msg| if predicate(&msg) { ... }` closure.
