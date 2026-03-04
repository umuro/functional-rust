# 332: Retry Async

**Difficulty:** 3  **Level:** Advanced

Retry failed operations with exponential backoff — the foundation of resilient async services.

## The Problem This Solves

In distributed systems, transient failures are the norm: a database is briefly overloaded, a DNS lookup times out, a downstream API returns 503. Treating every failure as permanent and propagating an error immediately makes services fragile. What you need is a principled retry loop: try again, wait a bit longer each time, and give up only after a reasonable number of attempts.

Naive retry is easy to write but easy to get wrong: forgetting to distinguish transient from permanent errors (retrying a 404 is pointless), using a fixed delay (can cause thundering-herd storms), or allowing infinite retries (can cascade into runaway loops). This example formalizes all three concerns: `Transient` vs `Permanent` error variants, configurable exponential backoff with a multiplier, and a hard limit on attempts.

In async Rust with tokio, you'd replace `thread::sleep` with `tokio::time::sleep(delay).await`. The retry logic itself is identical.

## The Intuition

Like a JavaScript `fetchWithRetry`:
```js
async function fetchWithRetry(url, attempts = 3, delay = 100) {
  for (let i = 0; i < attempts; i++) {
    try { return await fetch(url); }
    catch (e) {
      if (i < attempts - 1) await sleep(delay * 2**i);
    }
  }
}
```

Rust's version is more explicit about *why* it's failing (`Transient` = worth retrying, `Permanent` = stop immediately). This distinction prevents retrying authentication errors, validation failures, or "not found" responses.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum RetryError<E> {
    Transient(E),  // Worth retrying — network blip, timeout, overload
    Permanent(E),  // Don't retry — bad input, auth failure, 404
}

struct RetryConfig { max_attempts: usize, base_delay: Duration, multiplier: f64 }

fn retry<T, E: Clone>(cfg: &RetryConfig, mut f: impl FnMut() -> Result<T, RetryError<E>>) -> Result<T, E> {
    let mut delay = cfg.base_delay;
    for attempt in 1..=cfg.max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(RetryError::Permanent(e)) => return Err(e),  // bail immediately
            Err(RetryError::Transient(e)) => {
                if attempt < cfg.max_attempts {
                    thread::sleep(delay);
                    delay = delay.mul_f64(cfg.multiplier);  // exponential backoff
                } else {
                    return Err(e);
                }
            }
        }
    }
    unreachable!()
}
```

`FnMut` (not `Fn`) lets the closure carry mutable state — like a counter tracking which attempt it's on. `Duration::mul_f64` doubles (or n×) the wait on each retry.

## What This Unlocks

- **Resilient HTTP clients** — retry on 429, 503, or connection reset; fail fast on 400, 401, 404.
- **Database reconnect loops** — back off when the DB is restarting instead of hammering the connection pool.
- **Message queue consumers** — retry poison messages with increasing delay before routing to a dead-letter queue.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error variants | `retry_err = Transient \| Permanent` (custom type) | `RetryError<E>` (generic enum) |
| Retry loop | Recursive `loop i d` with `Thread.delay d` | Iterative `for` with `thread::sleep` |
| Delay scaling | `d *. 2.0` (float multiply) | `delay.mul_f64(multiplier)` on `Duration` |
| Closure type | `unit -> ('a, retry_err) result` | `FnMut() -> Result<T, RetryError<E>>` |
