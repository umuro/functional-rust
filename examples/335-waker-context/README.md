📖 **[View on hightechmind.io →](https://hightechmind.io/rust/335-waker-context)**

---

# 335: Waker and Context

**Difficulty:** 5  **Level:** Master

How the executor knows to re-poll a future — `cx.waker().wake()` schedules a re-poll.

## The Problem This Solves

The executor polls futures until complete. But polling continuously wastes CPU. The `Waker` is the notification mechanism: when the future returns `Pending`, it stores the waker. When the external event happens, calling `waker.wake()` tells the executor to re-poll.

## The Intuition

It's like a JavaScript `Promise` resolver:
```js
new Promise((resolve, reject) => {
  setTimeout(() => resolve(42), 100);
});
```

In Rust, `waker.wake()` plays the role of `resolve` — the signal that the future is ready.

## How It Works in Rust

```rust
impl Future for ExternalFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        let mut s = self.state.lock().unwrap();
        if let Some(v) = s.value {
            Poll::Ready(v)
        } else {
            s.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Resolver {
    fn fulfill(self, value: i32) {
        let mut s = self.state.lock().unwrap();
        s.value = Some(value);
        if let Some(w) = s.waker.take() { w.wake(); }
    }
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Completion signal | `Lwt.wakeup resolver value` | `waker.wake()` |
| Suspension | `Lwt.wait ()` | `Poll::Pending` + stored `Waker` |
| Shared state | `ref` / `Mutex` | `Arc<Mutex<SharedState>>` |
