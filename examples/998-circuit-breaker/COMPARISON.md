# Circuit Breaker — Comparison

## Core Insight
The circuit breaker is an **automatic state machine** that protects callers from cascading failures. Like an electrical circuit breaker: too many failures "trip" it to Open, then it tests recovery in HalfOpen, then resets to Closed on success.

## OCaml Approach
- Mutable record fields for `state`, `failures`, `failure_threshold`
- `Mutex` to protect state transitions (thread-safe)
- `Unix.gettimeofday()` float for wall-clock timing
- `Open of float` carries the timestamp when it opened
- State check and transition in `call` function

## Rust Approach
- `Mutex<BreakerState>` + `Mutex<u32>` for state and failures
- `BreakerState::Open { opened_at: Instant }` — `Instant` for elapsed time
- `Instant::now().elapsed() >= recovery_timeout` for timeout check
- `call<T, E, F>(&self, f: F) -> CallResult<T, E>` — generic over result type
- `maybe_transition_to_half_open()` for clean separation

## Comparison Table

| Concept              | OCaml                                | Rust                                   |
|----------------------|--------------------------------------|----------------------------------------|
| State enum           | `type state = Closed \| Open \| HalfOpen` | `enum BreakerState { Closed, Open { at: Instant }, HalfOpen }` |
| Thread safety        | `Mutex.t` + explicit lock/unlock     | `Mutex<BreakerState>` RAII             |
| Timing               | `Unix.gettimeofday ()` (f64 secs)    | `Instant::now()` / `.elapsed()`        |
| Call result          | `BrResult v \| CircuitOpen \| CallError e` | `CallResult<T,E>` enum             |
| Transition logic     | Pattern match in `call`              | Separate `maybe_transition` method     |
| Generic over types   | `'a circuit_breaker`                 | Generic `<T, E, F>` on `call`          |
| Production           | Manual or library (retrying-oc)      | `failsafe-rs`, `tower::limit`          |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
