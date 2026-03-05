# 322: The Future Trait and Poll

**Difficulty:** 4  **Level:** Expert

A Future is just one method: `poll`. The runtime calls it. If the work is done, return `Ready`. If not, return `Pending` and arrange to be woken later.

## The Problem This Solves

When you write `async fn` and `.await`, the compiler generates state machine code for you. But what *is* a Future, actually? Understanding the `Future` trait answers this — and it matters when you need to implement your own async primitive, integrate non-async code into an async runtime, or debug why a future is never waking up.

Without understanding `poll`, async code feels like magic. You don't know why `await` suspends, how the runtime knows when to resume, or what the `Waker` is for. This leads to subtle bugs: futures that never wake, busy-polling that wastes CPU, or deadlocks from holding locks across `.await`.

## The Intuition

Imagine a restaurant. You order food (create a future). The waiter doesn't stand next to the kitchen watching — they go serve other tables. The kitchen calls the waiter when the order is ready (the `Waker`). The waiter comes back and delivers (returns `Poll::Ready`).

The `poll` method is: "Is the food ready?" The answer is either `Ready(food)` or `Pending` (with a promise to call you when it is). The runtime keeps a list of pending tasks and polls them when they signal readiness.

```
poll() → Poll::Ready(value)   ← work done, here's the result
poll() → Poll::Pending        ← not done yet, we'll wake you when ready
```

## How It Works in Rust

```rust
impl Future for DelayedValue {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready(self.value)         // work is done
        } else {
            self.remaining -= 1;
            cx.waker().wake_by_ref();       // tell the runtime: try again soon
            Poll::Pending                   // not ready yet
        }
    }
}
```

`Pin<&mut Self>` prevents the future from being moved in memory while it's being polled — important for self-referential state machines.

`cx.waker().wake_by_ref()` is how you tell the runtime "I'll be ready soon, poll me again."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Core async abstraction | `Lwt.t` (promise/thread) | `Future` trait with `poll` |
| State transition | implicit in Lwt machinery | explicit `Poll::Ready` / `Poll::Pending` |
| Wakeup mechanism | callback registered on promise | `Waker::wake()` via `Context` |
| Pinning | not needed | `Pin<&mut Self>` prevents moves |
