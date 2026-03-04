# 343: Cancellation Token

**Difficulty:** 3  **Level:** Advanced

Signal running tasks to stop gracefully — a shared atomic flag that tasks check cooperatively.

## The Problem This Solves

Long-running async tasks need a way to be told "stop now." A web server shutting down needs to cancel in-flight request handlers. A background job needs to stop when the user logs out. A search operation needs to abort when the user types another character. Without a cancellation mechanism, tasks run to completion even when their results are no longer needed — wasting CPU, holding connections, and delaying shutdown.

The cancellation token pattern is simple: a shared `AtomicBool` flag. Tasks check `token.is_cancelled()` at safe points in their loop. The controller calls `token.cancel()`, which sets the flag. Tasks see it on their next check and return early. This is *cooperative* cancellation — tasks must actively check and respect the signal.

In async Rust, `tokio_util::CancellationToken` is the production-ready version, integrating with `select!` for zero-cost cancellation points.

## The Intuition

Like Python's `threading.Event`:
```python
stop_event = threading.Event()
def worker():
    while not stop_event.is_set():
        do_work()
stop_event.set()  # signal to stop
```

Or Go's `context.WithCancel`:
```go
ctx, cancel := context.WithCancel(context.Background())
go func() { work(ctx) }()
cancel()  // signal cancellation
```

Rust's token is the same concept, made explicit with `Arc<AtomicBool>` for cheap cloning across tasks.

## How It Works in Rust

```rust
#[derive(Clone)]
struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)  // Acquire pairs with Release
    }
}

fn long_task(token: CancellationToken, steps: usize) -> Result<String, String> {
    for i in 0..steps {
        if token.is_cancelled() {
            return Err(format!("cancelled at step {i}"));  // early exit
        }
        thread::sleep(Duration::from_millis(10));
        println!("Step {i} complete");
    }
    Ok(format!("completed all {steps} steps"))
}
```

`Ordering::Release` / `Ordering::Acquire` ensure the cancellation signal is visible across threads without a full memory barrier — more efficient than `SeqCst` for this use case.

Token is `Clone` because it wraps `Arc<AtomicBool>` — each clone shares the same flag. Cancelling one cancels all.

## What This Unlocks

- **Graceful shutdown** — cancel all background tasks on `SIGTERM`; each checks the token and cleans up.
- **Request scoping** — cancel all work spawned by a request when the client disconnects.
- **Timeout composition** — pair with `tokio::time::timeout` or check the token inside a `select!` for deadline-based cancellation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Cancellation signal | `ref bool` or `Lwt.cancel` | `Arc<AtomicBool>` (thread-safe) |
| Cooperative check | `if !cancelled then ...` | `if token.is_cancelled() { return Err(...) }` |
| Token sharing | Passed as argument | `Clone` — each clone shares the same `Arc` |
| Production crate | N/A | `tokio_util::CancellationToken` with `select!` support |
