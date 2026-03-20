**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

[retry-backoff on hightechmind.io](https://hightechmind.io/posts/functional-rust/retry-backoff)

---

## Problem Statement

Implement a retry combinator with exponential backoff: given a fallible operation `FnMut() -> Result<T, E>`, try up to `max_attempts` times, sleeping `base_delay * 2^attempt` milliseconds between failures. Also implement a jitter variant to avoid thundering-herd problems. The combinator is purely functional — it wraps any fallible operation.

## Learning Outcomes

- Implement `retry<T, E, F: FnMut() -> Result<T,E>>(max, base_ms, f) -> Result<T,E>`
- Use `1 << attempt` for exponential backoff: attempt 0 = base, attempt 1 = 2×base, attempt 2 = 4×base
- Skip the final sleep: `if attempt + 1 < max_attempts { sleep(...) }`
- Add jitter: add `base / 3 * (attempt % 3)` to spread retry times across time
- Recognize the combinator pattern: `retry` is higher-order, wrapping any `FnMut() -> Result`

## Rust Application

```rust
fn retry<T, E, F>(max_attempts: usize, base_delay_ms: u64, mut f: F) -> Result<T, E>
where F: FnMut() -> Result<T, E>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let delay = base_delay_ms * (1 << attempt);
                    thread::sleep(Duration::from_millis(delay));
                }
            }
        }
    }
    Err(last_err.unwrap())
}

fn retry_with_jitter<T, E, F>(max_attempts: usize, base_delay_ms: u64, mut f: F) -> Result<T, E>
where F: FnMut() -> Result<T, E>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                if attempt + 1 < max_attempts {
                    let base  = base_delay_ms * (1 << attempt);
                    let jitter = base / 3 * (attempt as u64 % 3);
                    thread::sleep(Duration::from_millis(base + jitter));
                }
            }
        }
    }
    Err(last_err.unwrap())
}
```

`FnMut` (not `Fn`) allows the closure to maintain state between calls — e.g., a retry counter, a connection handle, or a mutable buffer. `last_err` stores the most recent error to return if all attempts fail.

`1 << attempt` computes `2^attempt` efficiently: attempt 0 → 1×base, attempt 1 → 2×base, attempt 2 → 4×base. For `max_attempts = 8` and `base = 100ms`, the maximum backoff is `100 * 128 = 12.8s`.

## OCaml Approach

```ocaml
let retry max_attempts base_delay_ms f =
  let rec go attempt =
    match f () with
    | Ok v -> Ok v
    | Error e when attempt + 1 >= max_attempts -> Error e
    | Error _ ->
      let delay = base_delay_ms * (1 lsl attempt) in
      Thread.delay (float_of_int delay /. 1000.0);
      go (attempt + 1)
  in
  go 0

let retry_with_backoff ~max ~base_ms ?(max_delay_ms = 30_000) f =
  let rec loop attempt =
    match f () with
    | Ok v -> Ok v
    | Error e when attempt >= max -> Error e
    | Error _ ->
      let delay = min max_delay_ms (base_ms * (1 lsl attempt)) in
      Unix.sleepf (float_of_int delay /. 1000.0);
      loop (attempt + 1)
  in
  loop 0
```

OCaml's tail-recursive `go` avoids accumulating stack frames across retries. The `when` guard in the `Error` arm cleanly separates "final attempt" from "retry" without a separate branch.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Loop style | Imperative `for` loop | Recursive `go` function |
| Exponential delay | `1 << attempt` | `1 lsl attempt` |
| Skip final sleep | `if attempt + 1 < max` | `when attempt + 1 >= max -> Error e` |
| Max delay cap | Manual `min(max_delay, delay)` | Same |

Jitter prevents "thundering herd": when many clients retry simultaneously, jitter spreads them out in time. Adding `rand::thread_rng().gen_range(0..base/2)` provides true randomized jitter in production.

## Exercises

1. Add a maximum delay cap: `delay = delay.min(max_delay_ms)` before sleeping.
2. Implement `retry_until<F, P: Fn(&T) -> bool>(f, pred, max) -> Option<T>` — retry until the result satisfies `pred`.
3. Add an `on_retry: impl Fn(usize, &E)` callback that is called before each retry with attempt number and error.
4. Implement async retry: `async fn retry_async<T, E, F: AsyncFn>(...) -> Result<T,E>` using `tokio::time::sleep`.
5. Implement full binary exponential backoff with jitter: `delay = random(0, base * 2^attempt)` bounded by a cap.
