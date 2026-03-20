📖 **[View on hightechmind.io →](https://hightechmind.io/rust/331-timeout-async)**

---

# 331: Timeouts with time::timeout

## Problem Statement

Network operations that hang indefinitely freeze applications. A DNS lookup, database query, or HTTP request that never responds must be bounded by a deadline. Timeouts are the fundamental resilience mechanism for distributed systems — every external call should have one. The `TimeoutError<E>` pattern distinguishes operation failures (the operation ran but failed) from timeout failures (the deadline expired), enabling different recovery strategies for each.

## Learning Outcomes

- Implement a `TimeoutError<E>` type with `Elapsed` and `TaskFailed(E)` variants
- Use `mpsc::channel` with a recv deadline to implement synchronous timeouts
- Distinguish timeout (deadline expired) from task failure (operation failed with error)
- Recognize the `tokio::time::timeout(dur, future)` pattern for async timeouts

## Rust Application

A `TimeoutError<E>` with structured discrimination:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeoutError<E> {
    Elapsed,         // Deadline expired
    TaskFailed(E),   // Operation ran but failed
}

pub fn run_with_timeout<T, E, F>(f: F, timeout: Duration) -> Result<T, TimeoutError<E>>
where
    F: FnOnce() -> Result<T, E> + Send + 'static,
    T: Send + 'static, E: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    match rx.recv_timeout(timeout) {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(e)) => Err(TimeoutError::TaskFailed(e)),
        Err(_) => Err(TimeoutError::Elapsed),
    }
}
```

## OCaml Approach

OCaml's `Lwt_unix.with_timeout` provides timeout functionality. In `Async`, `Clock.with_timeout` serves the same purpose:

```ocaml
let* result =
  Lwt_unix.with_timeout 5.0 (fun () -> perform_operation ())
(* Returns Error `Timeout on expiry, propagates other errors *)
```

## Key Differences

1. **Structured timeout error**: `TimeoutError<E>` preserves the operation's error type; raw timeout functions often just use string errors.
2. **Cancellation**: Rust's thread-based timeout doesn't cancel the spawned thread (it continues); `tokio::time::timeout` genuinely cancels the future.
3. **Cascading timeouts**: In distributed systems, outer timeouts should be smaller than the sum of inner ones — a common design error.
4. **Production**: Every `reqwest`, `sqlx`, and `tokio` operation should have a timeout — ungated external calls are a reliability risk.

## Exercises

1. Add a retry parameter: if the operation times out, retry up to N times before giving up.
2. Implement a `deadline_from_now(secs)` function that creates a timeout computed from the current moment.
3. Distinguish between "timed out waiting for response" and "operation failed with error" in a client function, using `TimeoutError<AppError>`.
