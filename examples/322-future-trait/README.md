📖 **[View on hightechmind.io →](https://hightechmind.io/rust/322-future-trait)**

---

# 322: The Future Trait and Poll

## Problem Statement

The `async`/`.await` syntax in Rust is syntactic sugar over the `Future` trait. A `Future` is a state machine with a single `poll()` method that either returns `Poll::Ready(output)` or `Poll::Pending`. Understanding the underlying `Future` trait is essential for implementing custom async primitives, debugging async code, and understanding why `.await` cannot be used in non-async contexts. This is the foundation that all async Rust is built on.

## Learning Outcomes

- Understand `Future::poll()` as returning `Poll::Ready(T)` or `Poll::Pending`
- Implement a custom `Future` manually to understand the state machine model
- Recognize that `async fn` generates a `Future` impl automatically
- Understand the role of `Waker` in signaling the executor to re-poll

## Rust Application

The `DelayedValue` future demonstrates manual `Future` implementation:

```rust
pub struct DelayedValue {
    value: i32,
    remaining_polls: u32,
}

impl Future for DelayedValue {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        if self.remaining_polls == 0 {
            Poll::Ready(self.value)
        } else {
            self.remaining_polls -= 1;
            cx.waker().wake_by_ref();  // Tell executor to re-poll
            Poll::Pending
        }
    }
}
```

## OCaml Approach

OCaml's `Lwt` uses continuations (callbacks) rather than a poll-based model. A Lwt "promise" is fulfilled when a callback is registered:

```ocaml
(* Lwt: promise-based rather than poll-based *)
let delayed_value n =
  let p, r = Lwt.wait () in
  Lwt.on_success (Lwt_unix.sleep 0.1) (fun () -> Lwt.wakeup r n);
  p
```

OCaml 5's `Effect` system provides even lower-level primitives for custom async runtimes.

## Key Differences

1. **Poll vs callback**: Rust's `Future` is pull-based (executor calls `poll`); OCaml's `Lwt` is push-based (completion triggers callbacks).
2. **Zero-cost**: Rust's state machine generation produces zero-allocation futures (often); Lwt uses heap-allocated closures for continuations.
3. **Waker contract**: Rust requires the future to call `waker().wake()` when it can make progress — without this, the executor won't re-poll.
4. **Composability**: Both models compose well for concurrent execution; Rust's model allows more compiler optimization due to its static nature.

## Exercises

1. Implement a `ReadyFuture<T>` that always returns `Poll::Ready(value)` immediately without ever returning `Pending`.
2. Implement a `CountdownFuture` that returns `Pending` exactly N times before returning `Poll::Ready(())`.
3. Write a simple single-threaded executor that drives a `Future` to completion by calling `poll` repeatedly until `Ready`.
