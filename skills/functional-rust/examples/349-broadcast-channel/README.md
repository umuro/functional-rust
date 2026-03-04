# 349: Broadcast Channel

**Difficulty:** 3  **Level:** Advanced

One sender, many receivers — every subscriber gets a copy of every message. Pub/sub for async tasks.

## The Problem This Solves

Sometimes you need to send the same event to multiple independent consumers. A configuration change event should reach every worker. A system shutdown signal should notify all background tasks. A price update should be delivered to every connected WebSocket client. A plain `mpsc` channel has one receiver — the first one to call `recv()` gets the message; others miss it.

A broadcast channel (also called a fan-out channel) delivers each message to *all* active subscribers independently. Every subscriber has its own queue; the sender clones the message and pushes it to each queue. New subscribers join and receive messages from that point forward.

`tokio::sync::broadcast` is the production-ready version with FIFO ordering, lagged receiver detection, and configurable capacity. This example builds the same concept with `std::sync::mpsc`.

## The Intuition

Like Node.js `EventEmitter`:
```js
emitter.on('update', handler1);
emitter.on('update', handler2);
emitter.emit('update', payload);  // both handlers run
```

Or JavaScript's `BroadcastChannel` for cross-tab communication. In Go, you'd fan out by ranging over a slice of channels. The broadcast channel abstracts this: one `send`, all subscribers receive.

## How It Works in Rust

```rust
struct BroadcastSender<T: Clone> {
    subscribers: Arc<Mutex<Vec<mpsc::SyncSender<T>>>>,
}

impl<T: Clone + Send + 'static> BroadcastSender<T> {
    fn subscribe(&self, buf: usize) -> BroadcastReceiver<T> {
        let (tx, rx) = mpsc::sync_channel(buf);
        self.subscribers.lock().unwrap().push(tx);
        BroadcastReceiver { rx }
    }

    fn send(&self, msg: T) {
        let subs = self.subscribers.lock().unwrap();
        for sub in subs.iter() {
            // Clone message for each subscriber
            let _ = sub.try_send(msg.clone());
        }
    }
}
```

`T: Clone` is required — the sender clones the message once per subscriber. For expensive types, wrap in `Arc<T>` to make clones cheap (arc clone = reference count increment).

In async Rust:
```rust
let (tx, _rx) = tokio::sync::broadcast::channel::<String>(16);
let rx1 = tx.subscribe();
let rx2 = tx.subscribe();
tx.send("hello".into())?;
// rx1 and rx2 both receive "hello"
```

## What This Unlocks

- **Event bus** — broadcast config changes, feature flag updates, or schema refreshes to all workers.
- **WebSocket fanout** — each connected client subscribes; the server broadcasts price ticks or chat messages.
- **Graceful shutdown** — broadcast a shutdown signal to all background tasks simultaneously.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Broadcast | `Event.send` to list of channels | `BroadcastSender` with `Vec<SyncSender>` |
| Message delivery | `List.iter (Event.sync send)` | `try_send` to each subscriber's channel |
| Clone requirement | Values shared by GC reference | `T: Clone` — explicitly cloned per subscriber |
| Production crate | `Lwt_react` / custom | `tokio::sync::broadcast` |
