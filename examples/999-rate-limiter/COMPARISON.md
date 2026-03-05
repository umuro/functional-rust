# Rate Limiter — Token Bucket — Comparison

## Core Insight
The token bucket algorithm is continuous-time rate limiting: tokens accumulate at a fixed rate up to capacity, and each request costs tokens. This allows bursts (up to capacity) while enforcing average rate.

## OCaml Approach
- `Unix.gettimeofday ()` returns a `float` (seconds since epoch)
- `elapsed = now - last_refill` gives time delta
- `new_tokens = elapsed * refill_rate` — continuous refill
- `Float.min capacity (tokens + new_tokens)` — cap at capacity
- `Mutex` for thread safety; `try_acquire` is non-blocking

## Rust Approach
- `Instant::now()` — monotonic clock, immune to system time changes
- `.elapsed().as_secs_f64()` for fractional seconds
- `Mutex<BucketState>` wraps mutable state (tokens + last_refill)
- `acquire` spins with `thread::sleep(1ms)` when empty
- `try_acquire(cost)` for variable-cost requests (e.g., large queries cost more)

## Comparison Table

| Concept              | OCaml                               | Rust                                |
|----------------------|-------------------------------------|-------------------------------------|
| Time primitive       | `Unix.gettimeofday ()` (wall clock) | `Instant::now()` (monotonic)        |
| Elapsed time         | `now -. last_refill` (float secs)   | `.elapsed().as_secs_f64()`          |
| Refill formula       | `min capacity (tokens + dt * rate)` | `(tokens + dt * rate).min(capacity)` |
| Non-blocking check   | `try_acquire`                       | `try_acquire(cost) -> bool`         |
| Blocking acquire     | Spin with `sleepf 0.001`            | Spin with `thread::sleep(1ms)`      |
| Variable cost        | `~cost` parameter                   | `cost: f64` parameter               |
| Thread safety        | `Mutex.t` + explicit lock/unlock    | `Mutex<BucketState>` RAII           |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
