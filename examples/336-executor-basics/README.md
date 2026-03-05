# 336: Executor Basics

**Difficulty:** 5  **Level:** Master

A minimal async executor — the engine that drives futures to completion by polling them.

## The Problem This Solves

`async fn` compiles into state machines that implement `Future`. But futures are inert — something must call `poll()` to drive them. That something is an *executor*.

This example builds a real executor from scratch: task queue, waker that reschedules tasks, and a run loop.

## The Intuition

Imagine a to-do list manager:
1. Add tasks to a queue
2. Pick a task and run it until it says "I'm blocked" (`Poll::Pending`)
3. When unblocked, the task re-adds itself to the queue
4. Keep working until the queue is empty

## How It Works in Rust

```rust
struct Task {
    future: Mutex<Option<BoxFuture>>,
    sender: mpsc::SyncSender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.sender.send(Arc::clone(self));
    }
}

impl SimpleExecutor {
    fn run(self) {
        drop(self.tx);  // when no more tasks, recv() ends
        while let Ok(task) = self.rx.recv() {
            let mut slot = task.future.lock().unwrap();
            if let Some(mut f) = slot.take() {
                let w = make_waker(Arc::clone(&task));
                if f.as_mut().poll(&mut Context::from_waker(&w)) == Poll::Pending {
                    *slot = Some(f);
                }
            }
        }
    }
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Executor | Lwt scheduler (implicit) | Explicit `block_on` or `run` |
| Task scheduling | Internal Lwt queue | `mpsc::SyncSender<Arc<Task>>` |
| Wakeup | Lwt resolver | `Waker::wake()` |
