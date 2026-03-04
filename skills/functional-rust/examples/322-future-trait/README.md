# 322: The Future Trait and Poll

**Difficulty:** 4  **Level:** Expert

A Future is just one method: `poll`. The runtime calls it. If the work is done, return `Ready`. If not, return `Pending` and arrange to be woken later.

## The Problem This Solves

When you write `async fn` and `.await`, the compiler generates state machine code for you. But what *is* a Future, actually? Understanding the `Future` trait answers this — and it matters when you need to implement your own async primitive, integrate non-async code into an async runtime, or debug why a future is never waking up.

Without understanding `poll`, async code feels like magic. You don't know why `await` suspends, how the runtime knows when to resume, or what the `Waker` is for. This leads to subtle bugs: futures that never wake, busy-polling that wastes CPU, or deadlocks from holding locks across `.await`.

Implementing `Future` manually also reveals that the entire async machinery is surprisingly simple — one method, two return values, one callback mechanism.

## The Intuition

Imagine a restaurant. You order food (create a future). The waiter doesn't stand next to the kitchen watching — they go serve other tables. The kitchen calls the waiter when the order is ready (the `Waker`). The waiter comes back and delivers (returns `Poll::Ready`).

The `poll` method is: "Is the food ready?" The answer is either `Ready(food)` or `Pending` (with a promise to call you when it is). The runtime keeps a list of pending tasks and polls them when they signal readiness.

In Python, `asyncio` hides this behind coroutines. In JavaScript, Promises chain callbacks. Rust exposes the mechanism directly — which gives you full control and zero runtime overhead.

```
poll() → Poll::Ready(value)   ← work done, here's the result
poll() → Poll::Pending        ← not done yet, we'll wake you when ready
```

## How It Works in Rust

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct DelayedValue { value: i32, remaining: u32 }

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

`Pin<&mut Self>` prevents the future from being moved in memory while it's being polled — important for self-referential state machines that the compiler generates from `async fn`.

`cx.waker().wake_by_ref()` is how you tell the runtime "I'll be ready soon, poll me again." In real I/O, the OS (via epoll/kqueue) calls the waker when a socket becomes readable.

The `block_on` in this example is a minimal hand-rolled executor — it just loops calling `poll` until `Ready`. Real runtimes (tokio, async-std) are far more sophisticated but implement the same interface.

## What This Unlocks

- **Custom async primitives**: Integrate timers, file I/O, or OS events directly into the async ecosystem.
- **Zero-cost abstractions**: No heap allocations, no virtual dispatch — just a state machine in a struct.
- **Debugging async code**: When a task hangs, you know to look at whether `wake()` is being called.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Core async abstraction | `Lwt.t` (promise/thread) | `Future` trait with `poll` |
| State transition | implicit in Lwt machinery | explicit `Poll::Ready` / `Poll::Pending` |
| Wakeup mechanism | callback registered on promise | `Waker::wake()` via `Context` |
| Pinning | not needed | `Pin<&mut Self>` prevents moves |
| Executor | Lwt main loop | any runtime that calls `poll` |
