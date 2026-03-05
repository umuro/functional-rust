# Retry with Exponential Backoff — Comparison

## Core Insight
Retry is a **higher-order function** that wraps a fallible computation and adds resilience. It's pure functional: takes `f: unit -> Result<T,E>`, returns `Result<T,E>`. The combinator doesn't care what `f` does — separation of concerns.

## OCaml Approach
- `retry ~max_attempts ~base_delay_ms f` takes a `unit -> ('a, 'e) result` thunk
- Named arguments (`?max_attempts`) give ergonomic defaults
- `Unix.sleepf` for the delay (seconds as float)
- Pattern match on `Error e when attempt >= max_attempts` for clean exit
- Recursive loop with accumulator for attempt count

## Rust Approach
- `FnMut() -> Result<T, E>` — `FnMut` because we call it multiple times
- `delay = base_ms * (1 << attempt)` — bit shift for powers of 2
- `if !is_retryable(&e) { return Err(e) }` — short-circuit for permanent errors
- Builder pattern (`RetryConfig`) for ergonomic configuration
- `thread::sleep(Duration::from_millis(delay))` for backoff

## Comparison Table

| Concept              | OCaml                               | Rust                                 |
|----------------------|-------------------------------------|--------------------------------------|
| Function signature   | `(unit -> ('a,'e) result) -> result`| `FnMut() -> Result<T,E>`             |
| Default args         | `?max_attempts=3`                   | Struct/builder pattern               |
| Delay calculation    | `base * 2^(attempt-1)`             | `base_ms * (1 << attempt)`           |
| Sleep primitive      | `Unix.sleepf secs`                  | `thread::sleep(Duration::from_millis)` |
| Selective retry      | `when is_retryable e`               | `if !is_retryable(&e) { return Err }` |
| Jitter               | `Random.int (delay/2)`              | Deterministic or `rand` crate        |
| Configuration        | Named arguments                     | Builder struct or direct params      |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
