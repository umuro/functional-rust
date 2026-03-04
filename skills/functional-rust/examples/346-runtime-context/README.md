# 346: Runtime Context

**Difficulty:** 3  **Level:** Advanced

The runtime is the engine behind async Rust — a `Handle` lets you interact with it from outside async code.

## The Problem This Solves

When you write `#[tokio::main]`, tokio creates a runtime, starts a thread pool, and runs your async code inside it. But sometimes you need to interact with the runtime from synchronous code — spawn a task from a callback, block on a future from a library that doesn't expose async, or shut down the runtime cleanly after all work is done.

The `Handle` pattern solves this: a `Handle` is a cheaply cloneable reference to a running runtime. You can spawn tasks on it, block on futures from sync code (`Handle::block_on`), or pass it to other threads. This is what `tokio::runtime::Handle::current()` does — it returns a handle to the runtime that's currently driving the current async context.

This example builds a minimal runtime (a worker thread + a channel for tasks) to illustrate the concept without requiring tokio.

## The Intuition

Think of the runtime as a web server's event loop (like Node.js's libuv). A `Handle` is like an `EventEmitter` reference you can pass around — it lets external code post work to the loop.

In Python: `loop = asyncio.get_event_loop()` gives you a handle to submit coroutines from sync code with `loop.run_until_complete(coro)`.

In tokio: `tokio::runtime::Handle::current().spawn(async { ... })` submits a task from sync code to the running async runtime.

## How It Works in Rust

```rust
struct Handle {
    sender: mpsc::SyncSender<Box<dyn FnOnce() + Send>>,
}

impl Handle {
    // Submit sync work to the runtime thread
    fn spawn_sync(&self, f: impl FnOnce() + Send + 'static) {
        let _ = self.sender.send(Box::new(f));
    }

    // Block the calling thread until the runtime thread completes the work
    fn block_on_simple<T: Send + 'static>(&self, f: impl FnOnce() -> T + Send + 'static) -> T {
        let (tx, rx) = mpsc::channel();
        self.spawn_sync(move || { let _ = tx.send(f()); });
        rx.recv().unwrap()  // wait for result
    }
}

// Graceful shutdown: signal the worker to stop after current task
fn shutdown(mut self) {
    *self.shutdown.lock().unwrap() = true;
    self.handle.spawn_sync(|| {});  // wake the worker thread
    if let Some(w) = self.worker.take() { w.join().unwrap(); }
}
```

The runtime worker thread loops on `rx.recv()`, executing tasks as they arrive. Dropping the sender (`drop(tx)`) causes `recv()` to return `Err`, cleanly ending the loop. The shutdown flag ensures the worker stops after the current task.

## What This Unlocks

- **FFI callbacks into async** — pass a `Handle` to a C callback; the callback uses it to submit work to the async runtime.
- **Sync → async bridge** — call async functions from synchronous library code using `Handle::block_on`.
- **Multi-runtime setups** — isolate different subsystems (HTTP, background jobs) on separate runtimes, each accessible via their own handle.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Runtime | Lwt scheduler (implicit global) | Explicit runtime (`tokio::Runtime`) |
| Handle | N/A — Lwt is global | `tokio::runtime::Handle` (clonable reference) |
| Submit from sync | `Lwt_main.run` (blocks) | `handle.spawn(...)` or `handle.block_on(...)` |
| Worker thread model | Single-threaded event loop | Multi-threaded work-stealing (tokio default) |
