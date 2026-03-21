**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[rate-limiter on hightechmind.io](https://hightechmind.io/posts/functional-rust/rate-limiter)

---

## Problem Statement

Implement a token bucket rate limiter. The bucket holds up to `capacity` tokens and refills at `refill_rate` tokens per second. Each request consumes `cost` tokens. `try_acquire` returns immediately (non-blocking); `acquire` sleeps until tokens are available. The bucket is thread-safe via `Mutex<BucketState>`.

## Learning Outcomes

- Model a token bucket with `{ tokens: f64, last_refill: Instant }` protected by `Mutex`
- Implement lazy refill: on every access, compute elapsed time and add `elapsed * refill_rate` tokens (capped at `capacity`)
- Implement `try_acquire(cost) -> bool` — non-blocking check and decrement
- Implement `acquire(cost)` — blocking version that sleeps until tokens are available
- Understand the token bucket vs leaky bucket distinction: token bucket allows short bursts up to `capacity`

## Rust Application

```rust
struct TokenBucket {
    state: Mutex<BucketState>,
    capacity: f64,
    refill_rate: f64,  // tokens per second
}

struct BucketState {
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        TokenBucket {
            state: Mutex::new(BucketState { tokens: capacity, last_refill: Instant::now() }),
            capacity, refill_rate,
        }
    }

    fn refill(state: &mut BucketState, capacity: f64, refill_rate: f64) {
        let elapsed = state.last_refill.elapsed().as_secs_f64();
        let new_tokens = elapsed * refill_rate;
        state.tokens = (state.tokens + new_tokens).min(capacity);
        state.last_refill = Instant::now();
    }

    fn try_acquire(&self, cost: f64) -> bool {
        let mut state = self.state.lock().unwrap();
        Self::refill(&mut state, self.capacity, self.refill_rate);
        if state.tokens >= cost {
            state.tokens -= cost;
            true
        } else {
            false
        }
    }

    fn acquire(&self, cost: f64) {
        loop {
            if self.try_acquire(cost) { return; }
            let wait_ms = (cost / self.refill_rate * 1000.0) as u64;
            thread::sleep(Duration::from_millis(wait_ms.max(1)));
        }
    }
}
```

Lazy refill: tokens are added only when the bucket is accessed, not on a background timer. `elapsed * refill_rate` computes exactly how many tokens have accumulated since the last access. `.min(capacity)` caps at the bucket size.

`Instant::now()` is called inside `refill` to update `last_refill` — this ensures the next refill computation is relative to the last refill time, not the original creation time.

`acquire` spins with a calculated sleep: if `cost = 1.0` and `refill_rate = 10.0`, sleep 100ms between retries — exactly the time to accumulate one token.

## OCaml Approach

```ocaml
type state = {
  mutable tokens: float;
  mutable last_refill: float;
}

type t = {
  state: state;
  capacity: float;
  refill_rate: float;
  mutex: Mutex.t;
}

let create capacity refill_rate =
  { state = { tokens = capacity; last_refill = Unix.gettimeofday () };
    capacity; refill_rate; mutex = Mutex.create () }

let try_acquire tb cost =
  Mutex.protect tb.mutex (fun () ->
    let now = Unix.gettimeofday () in
    let elapsed = now -. tb.state.last_refill in
    tb.state.tokens <- Float.min tb.capacity (tb.state.tokens +. elapsed *. tb.refill_rate);
    tb.state.last_refill <- now;
    if tb.state.tokens >= cost then begin
      tb.state.tokens <- tb.state.tokens -. cost;
      true
    end else false
  )
```

OCaml's `Unix.gettimeofday()` returns `float` seconds since epoch — less ergonomic than Rust's `Instant` (monotonic, no epoch arithmetic). `Mutex.protect` wraps the critical section cleanly.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Monotonic time | `Instant::now()` — guaranteed monotonic | `Unix.gettimeofday()` — wall clock, may go backward |
| Elapsed time | `.elapsed().as_secs_f64()` | `now -. last_refill` |
| Token cap | `.min(capacity)` | `Float.min capacity ...` |
| Lock scope | `Mutex::lock` guard (RAII) | `Mutex.protect` closure |

The token bucket allows bursts: if no requests arrive for 10 seconds and `capacity = 100`, the next burst of 100 requests all succeed immediately. Use a leaky bucket (fixed rate, no burst) when strict rate smoothing is required.

## Exercises

1. Implement a sliding window rate limiter: track timestamps of the last `N` requests, reject if `N` requests have occurred in the last `window_duration`.
2. Add metrics: track `requests_allowed`, `requests_denied`, and `total_wait_ms`.
3. Implement `acquire_many(cost: f64, timeout: Duration) -> bool` — acquire tokens but give up after `timeout`.
4. Build a rate-limited HTTP client: wrap `reqwest::Client` with `TokenBucket` to limit to `N` requests/second.
5. Implement a hierarchical rate limiter: global bucket + per-user bucket; both must have tokens for a request to proceed.
