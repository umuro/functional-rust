# 331: Timeouts with time::timeout

**Difficulty:** 3  **Level:** Advanced

Wrap any async operation with a deadline — if it doesn't complete in time, get a structured error instead of waiting forever.

## The Problem This Solves

External services fail in two ways: they return an error, or they go silent. An error you can handle. Silence hangs your service indefinitely — connections pile up, memory grows, users wait. Without timeouts, a slow database, a network partition, or a stuck worker can bring down your entire application.

Timeouts are not optional in production code. But adding them naively — with threads, flags, and polling — is error-prone and verbose. You end up with shared `AtomicBool` cancelled flags, checking them periodically, and still having race conditions. You need a clean abstraction that says "try this, give up after N milliseconds."

The second problem is error types: when a timeout fires, you need to distinguish it from the operation actually failing. "We never got a response" and "the operation returned an error" are different situations requiring different handling (retry vs. fail fast).

## The Intuition

In async Rust with tokio: `tokio::time::timeout(Duration::from_millis(100), some_future).await` — that's it. Returns `Ok(value)` if the future completes in time, or `Err(Elapsed)` if not.

This example uses `mpsc::recv_timeout` as the synchronous analogy — the same "try to get a result, give up after this long" pattern, just with channels instead of futures.

```
Python asyncio:    asyncio.wait_for(coro, timeout=1.0)
JavaScript:        Promise.race([fetch(...), delay(1000).then(() => { throw new Error('timeout') })])
Rust (tokio):      timeout(Duration::from_secs(1), async_operation()).await
Rust (std/sync):   rx.recv_timeout(Duration::from_secs(1))
```

The Rust version has a structural advantage: the compiler forces you to handle both cases. You can't accidentally ignore the timeout.

## How It Works in Rust

```rust
#[derive(Debug)]
enum TimeoutError {
    Elapsed,                  // operation took too long
    TaskFailed(String),       // operation ran but returned an error
}

fn with_timeout<T: Send + 'static>(
    timeout: Duration,
    f: impl FnOnce() -> Result<T, String> + Send + 'static
) -> Result<T, TimeoutError> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });  // run in background

    match rx.recv_timeout(timeout) {
        Ok(Ok(v))  => Ok(v),                                           // success
        Ok(Err(e)) => Err(TimeoutError::TaskFailed(e)),                // task failed
        Err(mpsc::RecvTimeoutError::Timeout) => Err(TimeoutError::Elapsed),  // too slow
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(TimeoutError::TaskFailed("disconnected".into())),
    }
}
```

The `let _ = tx.send(f())` in the background thread: if the timeout fires and the receiver is dropped, `send` will return `Err`. We ignore it — the work can finish or not, we've already moved on.

For async code, `tokio::time::timeout` works the same way but cancels the future properly instead of letting the thread continue.

## What This Unlocks

- **Resilient services**: Every external call (database, HTTP, file I/O) wrapped with a timeout — no silent hangs in production.
- **SLA enforcement**: Guarantee response times by bailing out of slow paths and serving a cached or degraded response.
- **Retry with backoff**: Combine timeout + retry loop: try, if `Elapsed` wait exponentially and try again up to N times.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Operation timeout | `Lwt_unix.with_timeout secs f` | `tokio::time::timeout(dur, fut).await` |
| Sync timeout | `Thread.delay` + shared flag | `rx.recv_timeout(dur)` |
| Timeout result type | exception `Lwt_unix.Timeout` | `Err(Elapsed)` — matches cleanly |
| Distinguish timeout vs error | exception handlers | separate enum variants in `Result` |
