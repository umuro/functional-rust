📖 **[View on hightechmind.io →](https://hightechmind.io/rust/335-waker-context)**

---

# 335: Waker and Context

## Problem Statement

When a future returns `Poll::Pending`, the executor must know when to re-poll it. Without this signaling mechanism, the executor would either poll continuously (busy-waiting, wasting CPU) or miss the completion event. The `Waker` mechanism solves this: the future stores the waker from `Context`, then calls `waker.wake()` when it's ready to be re-polled. This is the fundamental scheduling mechanism behind all async runtimes.

## Learning Outcomes

- Understand `Context<'_>` as the channel through which the executor gives the future a `Waker`
- Implement a future that stores the `Waker` and signals it when external state changes
- Recognize that `wake()` schedules the future for re-polling — not immediate execution
- Understand the shared state pattern: `Arc<Mutex<SharedState<T>>>` between future and resolver

## Rust Application

`ExternalFuture<T>` completes when a separate `Resolver` provides a value:

```rust
pub struct SharedState<T> { value: Option<T>, waker: Option<Waker> }

impl<T: Clone> Future for ExternalFuture<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut state = self.state.lock().unwrap();
        if let Some(value) = state.value.take() {
            Poll::Ready(value)
        } else {
            state.waker = Some(cx.waker().clone()); // Store for later wake
            Poll::Pending
        }
    }
}
// Resolver::resolve(value) sets value and calls waker.wake()
```

## OCaml Approach

OCaml's Lwt uses a resolver pattern (`Lwt.wait()` returns a promise + resolver). The internal mechanism uses callbacks rather than poll-based wakers:

```ocaml
let (promise, resolver) = Lwt.wait ()
(* Elsewhere: Lwt.wakeup resolver value *)
(* The promise completes when wakeup is called *)
```

## Key Differences

1. **Pull vs push**: Rust's Waker is pull-based (executor polls); OCaml's Lwt callbacks are push-based (completion triggers continuation).
2. **Clone requirement**: `Waker` implements `Clone` so multiple contexts can hold it; the executor uses a `RawWaker` vtable for dynamic dispatch.
3. **Wake = schedule**: `waker.wake()` doesn't immediately execute the future; it just queues it for the executor's next scheduling cycle.
4. **Thread safety**: `Waker` is `Send + Sync` — it can be stored in any thread-safe context and woken from any thread.

## Exercises

1. Implement a `ChannelFuture<T>` that wraps an `mpsc::Receiver<T>` and uses a `Waker` to be notified when a message arrives.
2. Show the difference between `wake_by_ref()` (keeps the waker alive) and `wake()` (consumes the waker).
3. Build a simple timer future that stores the waker and spawns a thread that sleeps, then calls `wake()` after the delay.
