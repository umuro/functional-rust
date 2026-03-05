📖 **[View on hightechmind.io →](https://hightechmind.io/rust/331-timeout-async)**

---

# 331: Timeouts with time::timeout

**Difficulty:** 3  **Level:** Advanced

Wrap any async operation with a deadline — if it doesn't complete in time, get a structured error instead of waiting forever.

## The Problem This Solves

External services fail in two ways: they return an error, or they go silent. An error you can handle. Silence hangs your service indefinitely. Without timeouts, a slow database or network partition can bring down your entire application.

The second problem is error types: you need to distinguish "we never got a response" from "the operation returned an error" for different handling strategies.

## The Intuition

```
Python asyncio:    asyncio.wait_for(coro, timeout=1.0)
Rust (tokio):      timeout(Duration::from_secs(1), async_op()).await
Rust (std/sync):   rx.recv_timeout(Duration::from_secs(1))
```

This example uses `mpsc::recv_timeout` as the synchronous analogy.

## How It Works in Rust

```rust
#[derive(Debug)]
enum TimeoutError<E> {
    Elapsed,              // operation took too long
    TaskFailed(E),        // operation ran but returned an error
}

fn with_timeout<T, E>(timeout: Duration, f: impl FnOnce() -> Result<T, E>) -> Result<T, TimeoutError<E>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });

    match rx.recv_timeout(timeout) {
        Ok(Ok(v))  => Ok(v),
        Ok(Err(e)) => Err(TimeoutError::TaskFailed(e)),
        Err(mpsc::RecvTimeoutError::Timeout) => Err(TimeoutError::Elapsed),
        Err(_) => Err(TimeoutError::TaskFailed(...)),
    }
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Timeout | `Lwt_unix.with_timeout` | `tokio::time::timeout` |
| Sync timeout | `Thread.delay` + flag | `rx.recv_timeout(dur)` |
| Result type | Exception | `Err(Elapsed)` enum |
