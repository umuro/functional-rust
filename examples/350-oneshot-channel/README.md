📖 **[View on hightechmind.io →](https://hightechmind.io/rust/350-oneshot-channel)**

---

# 350: Oneshot Channel
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Many concurrent operations need exactly one result delivered back: spawning a task and waiting for its answer, implementing a request-response protocol, or delivering a computation result from a worker thread. MPSC channels are overkill here — they're designed for streams of messages. A oneshot channel is optimized for single-value delivery: the sender can only send once (consuming itself), the receiver can only receive once, and both are guaranteed by the type system. Tokio's `tokio::sync::oneshot` implements this; the stdlib provides no equivalent, making it an instructive primitive to build from scratch.

## Learning Outcomes

- Build a oneshot channel using `Arc<(Mutex<Option<T>>, Condvar)>`
- Consume the `OneshotSender` on `send()` to enforce single-send at the type level
- Use `Condvar` to block the receiver until the value is ready
- Implement non-blocking `try_recv` alongside blocking `recv`
- Understand why `Sender` being consumed (not `&mut self`) prevents double-send
- Recognize oneshot channels as the basis for async futures (Promise/Future pattern)

## Rust Application

```rust
use std::sync::{Arc, Condvar, Mutex};

pub struct OneshotSender<T> {
    state: Arc<(Mutex<Option<T>>, Condvar)>,
}
pub struct OneshotReceiver<T> {
    state: Arc<(Mutex<Option<T>>, Condvar)>,
}

pub fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let state = Arc::new((Mutex::new(None), Condvar::new()));
    (OneshotSender { state: Arc::clone(&state) }, OneshotReceiver { state })
}

impl<T> OneshotSender<T> {
    pub fn send(self, value: T) { // consumes self: can only send once
        let (lock, cvar) = &*self.state;
        *lock.lock().unwrap() = Some(value);
        cvar.notify_one();
    }
}

impl<T> OneshotReceiver<T> {
    pub fn recv(self) -> T {
        let (lock, cvar) = &*self.state;
        let mut guard = lock.lock().unwrap();
        while guard.is_none() {
            guard = cvar.wait(guard).unwrap();
        }
        guard.take().unwrap()
    }
    pub fn try_recv(&self) -> Option<T> {
        self.state.0.lock().unwrap().take()
    }
}
```

`send(self)` takes ownership, preventing reuse — the type system enforces "send exactly once." The `Condvar` prevents busy-waiting: the receiver thread sleeps until the sender wakes it. This is exactly how a Promise works: set the value once, signal all waiters.

## OCaml Approach

OCaml's `Lwt` uses `Lwt.task()` for oneshot semantics:

```ocaml
let (promise, resolver) = Lwt.task () in
(* in another fiber: *)
Lwt.wakeup_later resolver 42;
(* back in caller: *)
let%lwt value = promise in
Printf.printf "%d\n" value
```

`Lwt.task()` returns a `(promise, resolver)` pair. The resolver can only be used once (`wakeup_later` is idempotent after first call). This is the direct Lwt equivalent of a oneshot channel.

## Key Differences

| Aspect | Rust oneshot channel | OCaml `Lwt.task` |
|--------|---------------------|------------------|
| Single-send enforcement | `send(self)` consumes sender | Idempotent `wakeup_later` |
| Blocking wait | `Condvar::wait` | `let%lwt` suspends fiber |
| Non-blocking check | `try_recv()` | `Lwt.state promise` |
| Dropped sender | Receiver blocks forever (deadlock) | `Lwt.cancel` can resolve |
| Async version | `tokio::sync::oneshot` | Built into Lwt |

## Exercises

1. **Timeout recv**: Wrap `recv()` with a timeout by spawning a thread that sleeps then drops the sender; return `Result<T, Timeout>` — use `try_recv` polling in a loop to detect the timeout.
2. **Request-response**: Use a pair of oneshot channels to implement a synchronous request-response: the requester sends a `(payload, OneshotSender<Response>)` on an mpsc channel; the worker processes it and sends back on the oneshot.
3. **Tokio oneshot**: Rewrite the implementation using `tokio::sync::oneshot`; demonstrate cancellation by dropping the receiver before the sender sends, and observe `SendError` on the sender side.
