# 332: Retry Async

**Difficulty:** 3  **Level:** Advanced

Retry failed operations with exponential backoff — the foundation of resilient async services.

## The Problem This Solves

In distributed systems, transient failures are the norm. Treating every failure as permanent makes services fragile. You need a principled retry loop: try again, wait longer each time, give up after reasonable attempts.

This example formalizes: `Transient` vs `Permanent` error variants, configurable exponential backoff, and a hard limit on attempts.

## The Intuition

Like a JavaScript `fetchWithRetry`:
```js
async function fetchWithRetry(url, attempts = 3, delay = 100) {
  for (let i = 0; i < attempts; i++) {
    try { return await fetch(url); }
    catch (e) { if (i < attempts - 1) await sleep(delay * 2**i); }
  }
}
```

Rust's version is more explicit about *why* it's failing.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum RetryError<E> {
    Transient(E),  // Worth retrying
    Permanent(E),  // Don't retry
}

fn retry<T, E: Clone>(cfg: &RetryConfig, mut f: impl FnMut() -> Result<T, RetryError<E>>) -> Result<T, E> {
    let mut delay = cfg.base_delay;
    for attempt in 1..=cfg.max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(RetryError::Permanent(e)) => return Err(e),
            Err(RetryError::Transient(e)) => {
                if attempt < cfg.max_attempts {
                    thread::sleep(delay);
                    delay = delay.mul_f64(cfg.multiplier);
                } else {
                    return Err(e);
                }
            }
        }
    }
    unreachable!()
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error variants | Custom ADT | `RetryError<E>` enum |
| Delay scaling | `d *. 2.0` | `delay.mul_f64(multiplier)` |
| Closure type | `unit -> result` | `FnMut() -> Result<T, RetryError<E>>` |
