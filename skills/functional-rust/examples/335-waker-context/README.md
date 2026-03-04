# 335: Waker and Context

**Difficulty:** 5  **Level:** Master

How the executor knows to re-poll a future — `cx.waker().wake()` schedules a re-poll.

## The Problem This Solves

The async executor's job is to poll futures until they complete. But polling continuously (busy-waiting) wastes 100% of a CPU core. The executor needs a way to be *notified* when a future that previously returned `Poll::Pending` is now ready to make progress.

That notification mechanism is the `Waker`. When your future returns `Pending`, it stores the waker from the `Context` (`cx.waker().clone()`). When the external event happens — the timer fires, the I/O is ready, the channel receives a value — code somewhere calls `waker.wake()`, which tells the executor: "re-poll this future now." The executor then schedules the future for another poll, and this time it returns `Poll::Ready`.

Without `Waker`, every async runtime would have to busy-poll all pending futures. With it, the runtime can park idle and only wake up tasks that have actual work to do.

## The Intuition

It's like a JavaScript `Promise` resolver. When you create a Promise:
```js
new Promise((resolve, reject) => {
  setTimeout(() => resolve(42), 100);
});
```
The executor doesn't know the promise is ready until `resolve` is called. In Rust, `waker.wake()` plays the role of `resolve` — it's the signal from outside that tells the executor "this future is ready to continue."

`Context` is just the carrier for the `Waker`. Every time a future is polled, the executor passes a fresh `Context` containing the waker for that task.

## How It Works in Rust

```rust
struct ExternalFuture { state: Arc<Mutex<SharedState>> }

impl Future for ExternalFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        let mut s = self.state.lock().unwrap();
        if let Some(v) = s.value {
            Poll::Ready(v)        // value is available — done
        } else {
            // Store the waker so the resolver can wake us later
            s.waker = Some(cx.waker().clone());
            Poll::Pending         // not ready — suspend
        }
    }
}

struct Resolver { state: Arc<Mutex<SharedState>> }

impl Resolver {
    fn fulfill(self, value: i32) {
        let mut s = self.state.lock().unwrap();
        s.value = Some(value);
        // Wake the executor: "this future is ready to poll again"
        if let Some(w) = s.waker.take() { w.wake(); }
    }
}
```

The `block_on` executor in this example actually implements proper waker notifications using an `AtomicBool` — it parks when pending and resumes when `wake()` fires. This is a minimal but correct executor.

## What This Unlocks

- **Custom `Future` implementations** — any type that wraps OS I/O, a C callback, or an external event source needs to implement this pattern.
- **Bridging sync callbacks to async** — wrap a callback-based API (timers, sensors, UI events) into a proper `Future`.
- **Building executors** — understanding `Waker` is prerequisite to writing your own async runtime.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Completion signal | `Lwt.wakeup resolver value` (explicit resolver) | `waker.wake()` (executor re-polls the future) |
| Suspension | `Lwt.wait ()` returns a suspended thread + resolver | `Poll::Pending` + stored `Waker` |
| Executor notification | Internal to Lwt scheduler | Explicit: caller must call `waker.wake()` |
| Shared state | `ref` / `Mutex` | `Arc<Mutex<SharedState>>` |
