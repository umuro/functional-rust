# 350: Oneshot Channel

**Difficulty:** 2  **Level:** Intermediate

Send exactly one value from one task to another — a future you resolve from outside.

## The Problem This Solves

You spawn a background task to do some work and you need to get its result back. `mpsc` is overkill — you only need to send one value. What you want is a single-use channel: one sender, one receiver, one message. After the message is sent, the channel is done.

This is the request-response pattern in async systems: the caller sends a request along with a `OneshotSender`; the handler processes the request and calls `sender.send(response)`. The caller had been `await`ing the `OneshotReceiver`, and now it unblocks with the result. This avoids shared mutable state entirely — the response travels through the channel rather than being written to a shared variable.

`tokio::sync::oneshot` is the production version. It integrates with async/await: the receiver is a `Future` that resolves when the sender fires.

## The Intuition

Like a JavaScript `Promise` with an external `resolve`:
```js
let resolve;
const promise = new Promise(r => resolve = r);
// later, from anywhere:
resolve(42);
// awaiting promise now gets 42
```

The `OneshotSender` is `resolve`. The `OneshotReceiver` is the `promise`. The key property: the sender is consumed on send — it can't be called twice.

## How It Works in Rust

```rust
fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let state = Arc::new((Mutex::new(None), Condvar::new()));
    (OneshotSender { state: Arc::clone(&state) }, OneshotReceiver { state })
}

impl<T> OneshotSender<T> {
    fn send(self, value: T) {  // `self` is consumed — can only send once
        let (lock, cvar) = &*self.state;
        *lock.lock().unwrap() = Some(value);
        cvar.notify_one();  // wake the receiver
    }
}

impl<T> OneshotReceiver<T> {
    fn recv(self) -> T {  // blocks until sender fires
        let (lock, cvar) = &*self.state;
        let mut guard = lock.lock().unwrap();
        while guard.is_none() {
            guard = cvar.wait(guard).unwrap();
        }
        guard.take().unwrap()
    }
}
```

The sender takes `self` by value — consuming it prevents sending twice. This is the Rust type system enforcing "exactly one send" at compile time, not runtime.

In async Rust:
```rust
let (tx, rx) = tokio::sync::oneshot::channel::<i32>();
tokio::spawn(async move { tx.send(42).unwrap(); });
let value = rx.await.unwrap();
```

## What This Unlocks

- **Request-response actors** — send a request with an embedded `OneshotSender`; the actor sends the reply through it.
- **Parallel task results** — spawn N tasks each with their own oneshot; collect all results independently.
- **Async barrier** — use a oneshot as a ready signal: "wait here until this initialization is complete."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Oneshot channel | `Lwt.wait ()` → `(promise, resolver)` pair | `tokio::sync::oneshot::channel()` |
| Send once | Convention — resolver can be called multiple times | Type system: `send(self)` consumes the sender |
| Waiting | `Lwt.bind promise (fun v -> ...)` | `rx.await` or `rx.recv()` |
| Cross-thread | `Mutex` + `Condition` | Built into `tokio::sync::oneshot` |
