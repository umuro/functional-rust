# OCaml vs Rust: Retry Async

## Retry Loop

**OCaml:**
```ocaml
let rec loop attempt delay =
  match f () with
  | Ok v -> Ok v
  | Error (Permanent e) -> Error e
  | Error (Transient e) ->
    if attempt >= max_attempts then Error e
    else (Thread.delay delay; loop (attempt + 1) (delay *. multiplier))
```

**Rust:**
```rust
fn retry<T, E: Clone>(cfg: &RetryConfig, mut f: impl FnMut()->Result<T,RetryError<E>>) -> Result<T, E> {
    let mut delay = cfg.base_delay;
    for attempt in 1..=cfg.max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(RetryError::Permanent(e)) => return Err(e),
            Err(RetryError::Transient(e)) => {
                thread::sleep(delay);
                delay = delay.mul_f64(cfg.multiplier);
            }
        }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Loop style | Recursive | Iterative for loop |
| Delay type | Float seconds | `Duration` |
| Delay scaling | `*. 2.0` | `.mul_f64(2.0)` |
| State | Counter as ref | `FnMut` with internal state |
