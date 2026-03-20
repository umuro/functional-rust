[circuit-breaker on hightechmind.io](https://hintechmind.io/posts/functional-rust/circuit-breaker)

---

## Problem Statement

Implement the Circuit Breaker pattern — a fault-tolerance mechanism with three states: Closed (normal operation), Open (refusing calls after too many failures), and Half-Open (testing recovery after a timeout). When failure count exceeds a threshold, the breaker opens and rejects calls. After a recovery timeout, it transitions to Half-Open and allows one test call.

## Learning Outcomes

- Model the circuit breaker as `enum BreakerState { Closed, Open { opened_at: Instant }, HalfOpen }`
- Protect state transitions with `Mutex<BreakerState>` and `Mutex<u32>` for failure count
- Implement `call<T, E, F>` that: checks state, executes `f()`, updates state based on success/failure
- Implement `maybe_transition_to_half_open` that checks if `recovery_timeout` has elapsed since opening
- Return `CallResult::CircuitOpen` when the breaker is open, without calling `f`

## Rust Application

```rust
#[derive(Debug, PartialEq, Clone)]
enum BreakerState { Closed, Open { opened_at: Instant }, HalfOpen }

pub struct CircuitBreaker {
    state: Mutex<BreakerState>,
    failures: Mutex<u32>,
    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    pub fn call<T, E, F>(&self, f: F) -> CallResult<T, E>
    where F: FnOnce() -> Result<T, E>,
    {
        self.maybe_transition_to_half_open();
        let state = self.state.lock().unwrap().clone();
        match state {
            BreakerState::Open { .. } => return CallResult::CircuitOpen,
            _ => {}
        }
        match f() {
            Ok(v) => {
                *self.state.lock().unwrap() = BreakerState::Closed;
                *self.failures.lock().unwrap() = 0;
                CallResult::Success(v)
            }
            Err(e) => {
                let mut failures = self.failures.lock().unwrap();
                *failures += 1;
                if *failures >= self.failure_threshold {
                    *self.state.lock().unwrap() = BreakerState::Open { opened_at: Instant::now() };
                }
                CallResult::Failure(e)
            }
        }
    }
}
```

`maybe_transition_to_half_open` checks whether the `Open` state has expired: if `opened_at.elapsed() >= recovery_timeout`, transition to `HalfOpen`. This is called on every `call()` before checking state.

In `HalfOpen`, the call is allowed. On success, transition to `Closed` and reset failures. On failure, transition back to `Open` with a fresh `opened_at`.

Two separate `Mutex` fields (`state` and `failures`) can cause TOCTOU issues if not careful — in production, combine them into a single `Mutex<BreakerInner>` struct.

## OCaml Approach

```ocaml
type state = Closed | Open of float | HalfOpen

type t = {
  mutable state: state;
  mutable failures: int;
  threshold: int;
  recovery_s: float;
  mutex: Mutex.t;
}

let call cb f =
  Mutex.protect cb.mutex (fun () ->
    (match cb.state with
     | Open t when Unix.gettimeofday () -. t >= cb.recovery_s ->
       cb.state <- HalfOpen
     | _ -> ());
    match cb.state with
    | Open _ -> Error `CircuitOpen
    | _ ->
      match f () with
      | Ok v ->
        cb.state <- Closed; cb.failures <- 0; Ok v
      | Error e ->
        cb.failures <- cb.failures + 1;
        if cb.failures >= cb.threshold then
          cb.state <- Open (Unix.gettimeofday ());
        Error e)
```

OCaml's `Mutex.protect` wraps the entire operation — simpler than Rust's two separate `Mutex` fields but holds the lock longer. The state machine logic is identical; only the synchronization primitive differs.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| State enum | `BreakerState` with `Instant` in `Open` | Custom type with `float` timestamp |
| Synchronization | Two `Mutex` fields (or one `Mutex<Inner>`) | Single `Mutex.protect` |
| Elapsed check | `opened_at.elapsed()` | `gettimeofday () -. t` |
| Return type | `CallResult<T, E>` — three variants | `Result` with `CircuitOpen` error |

Circuit breakers prevent cascading failures: if a downstream service is down, the breaker opens and fast-fails subsequent calls rather than piling up waiting threads. The Half-Open state enables automatic recovery without manual intervention.

## Exercises

1. Combine `state` and `failures` into a single `Mutex<BreakerInner>` to eliminate TOCTOU.
2. Add half-open timeout: if a Half-Open call takes longer than `timeout`, count as failure.
3. Implement metrics: track total calls, successes, failures, and rejections as `AtomicUsize` counters.
4. Add an `on_state_change: impl Fn(BreakerState, BreakerState)` callback fired on every state transition.
5. Implement a `CircuitBreakerRegistry` that manages multiple named breakers and provides aggregate health status.
