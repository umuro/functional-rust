# 327: Spawning Concurrent Tasks

**Difficulty:** 3  **Level:** Advanced

Spawn a task to run it in the background independently — fire and forget, or collect the result later via a handle.

## The Problem This Solves

Some work should happen concurrently without blocking the current flow. Sending a welcome email when a user registers shouldn't make the registration response wait. Processing an uploaded file shouldn't hold open the HTTP connection. Spawning tasks separates the "start this work" moment from the "wait for the result" moment.

Without task spawning, everything runs sequentially in the order you wrote it. You'd have to manually manage threads (expensive), channels (verbose), or callbacks (inverted control flow). Spawning gives you a handle you can optionally await — use it when you need the result, ignore it if you don't.

The critical constraint: spawned tasks must be `'static`. They cannot borrow from the scope that spawned them — they might outlive it. This forces you to be explicit about ownership: pass cloned data, use `Arc`, or move data in.

## The Intuition

`thread::spawn` in std Rust is like `asyncio.create_task()` in Python or firing off a Promise in JavaScript without awaiting it — the work starts immediately in the background.

```python
# Python: fire and forget
task = asyncio.create_task(do_something())
# ... continue with other work ...
result = await task  # collect later if needed
```

```rust
// Rust: same pattern with threads
let handle = thread::spawn(move || do_something());
// ... continue with other work ...
let result = handle.join().unwrap();  // collect later if needed
```

With `tokio::spawn` in async code, the analogy is even closer — lightweight tasks (not OS threads) scheduled cooperatively. This example uses `thread::spawn` as the synchronous demonstration.

## How It Works in Rust

```rust
fn spawn_worker(id: usize, delay_ms: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {  // move id and delay_ms into the thread
        thread::sleep(Duration::from_millis(delay_ms));
        format!("worker-{id} done after {delay_ms}ms")
    })
    // returns immediately — thread is running in background
}

// Spawn all, then collect results
let handles: Vec<_> = (0..5)
    .map(|i| spawn_worker(i, (5-i) as u64 * 10))
    .collect();                              // all 5 threads now running

for h in handles {
    println!("{}", h.join().unwrap());      // wait for each in order
}
```

The `drop(tx)` pattern in `spawn_with_channel` is important: you must drop the original sender after cloning it for each thread. Otherwise the receiver never closes — it waits forever for a sender that will never send.

## What This Unlocks

- **Background processing**: Offload work (logging, analytics, emails) that shouldn't block the critical path.
- **Parallel pipelines**: Spawn N workers processing items concurrently, collect results as they complete.
- **Async task management**: With `tokio::spawn`, get lightweight green threads — thousands of concurrent tasks on a handful of OS threads.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Spawn | `Thread.create f ()` | `thread::spawn(\|\| ...)` |
| Await result | `Thread.join t` | `handle.join().unwrap()` |
| Async spawn | `Lwt.async (fun () -> ...)` | `tokio::spawn(async move { ... })` |
| Static requirement | N/A | spawned closures must be `'static` + `Send` |
