📖 **[View on hightechmind.io →](https://hightechmind.io/rust/332-retry-async)**

---

# 332: Retry Async with Exponential Backoff
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Transient failures — network blips, temporary service overloads, rate limits — are common in distributed systems. Retrying immediately is counterproductive (may worsen overload); retrying with exponential backoff gives services time to recover while limiting wait time with a maximum delay cap. Distinguishing transient from permanent errors is essential: don't retry bad input or authentication failures. This is a fundamental resilience pattern for any service calling external APIs.

## Learning Outcomes

- Distinguish transient errors (worth retrying) from permanent errors (don't retry)
- Implement exponential backoff: delay doubles each attempt, capped at a maximum
- Add jitter to prevent thundering herd: randomize delays slightly
- Implement a configurable retry loop with max attempts, base delay, multiplier, and max delay

## Rust Application

`RetryError<E>` discriminates transient from permanent, and `RetryConfig` controls the backoff:

```rust
pub enum RetryError<E> {
    Transient(E),  // Retry this
    Permanent(E),  // Don't retry this
}

pub fn retry<T, E, F>(config: &RetryConfig, mut f: F) -> Result<T, E>
where F: FnMut() -> Result<T, RetryError<E>> {
    for attempt in 0..config.max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(RetryError::Permanent(e)) => return Err(e),
            Err(RetryError::Transient(e)) if attempt + 1 == config.max_attempts => return Err(e),
            Err(RetryError::Transient(_)) => {
                let delay = config.delay_for(attempt);
                thread::sleep(delay);
            }
        }
    }
    unreachable!()
}
```

## OCaml Approach

OCaml's Lwt provides `Lwt_retry` in some libraries, or a custom recursive retry:

```ocaml
let rec retry ?(attempts=3) ?(delay=0.1) f () =
  Lwt.catch (fun () -> f () >>= fun v -> Lwt.return (Ok v))
    (fun exn ->
       if attempts <= 1 then Lwt.return (Error exn)
       else Lwt_unix.sleep delay >>= retry ~attempts:(attempts-1) ~delay:(delay *. 2.0) f)
```

## Key Differences

1. **Transient vs permanent**: Rust's `RetryError<E>` embeds the retry decision in the error type; the caller signals "try again" or "give up".
2. **Jitter**: Adding randomness (`delay * (1 + 0.1 * random)`) prevents synchronized retries from all clients hitting the server at the same moment.
3. **Production libraries**: `backoff` and `tower::retry` crates provide production-ready retry middleware with customizable policies.
4. **Circuit breaker**: Combines with retry: after too many failures, open the circuit to stop retrying for a cooling period.

## Exercises

1. Add jitter to the delay calculation: multiply the computed delay by a random factor between 0.9 and 1.1.
2. Implement a retry with a deadline: stop retrying after a total wall-clock time budget regardless of attempt count.
3. Build a circuit breaker on top of retry: after 5 consecutive failures, open the circuit and fail fast for 30 seconds.
