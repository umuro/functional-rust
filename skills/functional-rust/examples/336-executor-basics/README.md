# 336: Executor Basics

**Difficulty:** 5  **Level:** Master

A minimal async executor — the engine that drives futures to completion by polling them.

## The Problem This Solves

`async fn` and `await` are syntax sugar. They compile into state machines that implement `Future`. But futures are inert — they do nothing on their own. Something must call `future.poll(cx)` to drive them forward. That something is an *executor*.

Tokio, async-std, and smol are all executors. Understanding how they work demystifies async Rust: why tasks are spawned, why `.await` doesn't block, and why `block_on` is needed at the top level. This example builds a real (if minimal) executor from scratch: it has a task queue, a waker that re-schedules tasks, and a run loop that processes tasks until all are done.

This is the most foundational example in the async section. Once you understand executors, everything else — structured concurrency, cancellation, async mutex — makes sense.

## The Intuition

Imagine a to-do list manager:
1. You add tasks to a queue.
2. The manager picks a task and runs it until it says "I'm blocked, come back later" (`Poll::Pending`).
3. When the task is unblocked, it re-adds itself to the queue.
4. The manager keeps working until the queue is empty.

That's an executor. The `Waker` is the mechanism by which a blocked task says "add me back to the queue."

Python asyncio works the same way: `asyncio.get_event_loop().run_until_complete(coro)` is exactly `block_on`.

## How It Works in Rust

```rust
struct Task {
    future: Mutex<Option<BoxFuture>>,      // the future to drive
    sender: mpsc::SyncSender<Arc<Task>>,   // self-reschedule channel
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        // Put self back in the executor queue
        let _ = self.sender.send(Arc::clone(self));
    }
}

// Waker wraps a Task Arc — waking = scheduling
fn make_waker(task: Arc<Task>) -> Waker {
    // unsafe: manual vtable for clone/wake/drop on the Arc<Task>
    unsafe { Waker::from_raw(RawWaker::new(...)) }
}

impl SimpleExecutor {
    fn run(self) {
        drop(self.tx);  // when no more tasks are spawnable, recv() will end
        while let Ok(task) = self.rx.recv() {
            let mut slot = task.future.lock().unwrap();
            if let Some(mut f) = slot.take() {
                let w = make_waker(Arc::clone(&task));
                let mut cx = Context::from_waker(&w);
                if f.as_mut().poll(&mut cx) == Poll::Pending {
                    *slot = Some(f);  // put it back — it'll reschedule via waker
                }
                // if Poll::Ready: task is done, slot stays empty
            }
        }
    }
}
```

The `mpsc::sync_channel` acts as the task queue. Dropping `self.tx` ensures `rx.recv()` returns `Err` once all tasks complete and no more can be enqueued — cleanly ending the run loop.

## What This Unlocks

- **Understanding tokio internals** — tokio's work-stealing thread pool is a sophisticated version of exactly this pattern.
- **Embedded async** — write a minimal executor for `no_std` environments where you can't use tokio.
- **Testing async code** — single-threaded executors like this make async tests deterministic and fast.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Executor | Lwt scheduler (implicit, always running) | Explicit: must call `block_on` or `run` |
| Task scheduling | Lwt internal queue | `mpsc::SyncSender<Arc<Task>>` |
| Wakeup mechanism | Lwt resolver / callbacks | `Waker::wake()` → re-enqueue task |
| Spawn | `Lwt.async (fun () -> ...)` | `executor.spawn(async { ... })` |
