📖 **[View on hightechmind.io →](https://hightechmind.io/rust/336-executor-basics)**

---

# 336: Executor Basics

## Problem Statement

`async fn` and `Future` trait implementations don't run themselves — they need an executor to drive them to completion by calling `poll()` repeatedly until `Poll::Ready`. Understanding how an executor works explains the behavior of `tokio`, `async-std`, and other runtimes. This example builds a minimal single-threaded executor from scratch, demonstrating task queueing, waker implementation, and the poll loop.

## Learning Outcomes

- Understand that an executor drives futures by polling them until `Poll::Ready`
- Implement a minimal task queue using `mpsc::sync_channel` for the ready queue
- Build a `Waker` using the `RawWaker` vtable API for custom wake behavior
- Recognize the event loop: poll task → if Pending, park; on wake, re-queue and re-poll

## Rust Application

A minimal executor with a channel-based task queue:

```rust
pub struct Executor {
    ready_queue: mpsc::Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = make_waker(Arc::clone(&task));
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future); // Not done yet, put it back
                }
            }
        }
    }
}
```

## OCaml Approach

OCaml's Lwt has a similar event loop internally, but it uses a cooperative scheduling model based on callbacks. The "scheduler" in Lwt processes ready callbacks in a queue:

```ocaml
(* Lwt's internal loop, simplified: *)
let () =
  while not (Queue.is_empty ready_callbacks) do
    let callback = Queue.pop ready_callbacks in
    callback ()
  done
```

## Key Differences

1. **Poll vs callback**: Rust's executor poll-based model is more explicit about state; OCaml's Lwt uses implicit callback registration.
2. **Thread model**: A single-threaded executor processes futures sequentially; Tokio's multi-threaded executor uses work-stealing.
3. **RawWaker vtable**: The `RawWaker` API requires unsafe code for custom wakers — production code uses the `waker_fn` or `futures::task::noop_waker` helpers.
4. **Production runtimes**: Tokio, async-std, and smol all implement this loop with I/O multiplexing (epoll/kqueue/IOCP) for true async I/O.

## Exercises

1. Extend the minimal executor to support spawning new tasks from within futures.
2. Add a counter to the executor that tracks how many times each task was polled — useful for performance debugging.
3. Implement a `block_on(future)` function that runs a single future to completion using the minimal executor.
