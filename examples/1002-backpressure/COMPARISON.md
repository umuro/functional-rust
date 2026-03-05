# Backpressure — Comparison

## Core Insight
Backpressure prevents **unbounded buffering**: instead of letting producers flood a buffer until it runs out of memory, the producer is forced to wait when the buffer is full. This propagates slowness upstream — the natural rate-limiting of processing pipelines.

## OCaml Approach
- Simulate bounded channel with `Queue` + `Mutex` + two `Condition` variables
- `send_bounded`: wait while `Queue.length >= capacity` (not_full condition)
- `recv_bounded`: signal `not_full` after each receive
- `try_send`: non-blocking check — returns bool indicating acceptance
- More boilerplate than Rust — no built-in bounded channel

## Rust Approach
- `mpsc::sync_channel(N)` creates a bounded channel with buffer of N
- `tx.send(v)` blocks when buffer is full — zero-cost backpressure
- `tx.try_send(v)` returns `Err(TrySendError::Full(_))` immediately
- `sync_channel(0)` is a CSP rendezvous — synchronous handoff
- Works transparently with `rx.iter()` — pipeline stages auto-throttle

## Comparison Table

| Concept              | OCaml (simulated)                   | Rust                                  |
|----------------------|-------------------------------------|---------------------------------------|
| Bounded channel      | Manual Queue + Mutex + 2 Condvar    | `mpsc::sync_channel(N)`               |
| Blocking send        | `Condition.wait not_full` in send   | `tx.send(v)` blocks automatically     |
| Non-blocking send    | `try_send` (custom)                 | `tx.try_send(v)` built-in             |
| Buffer full error    | `return false` from try_send        | `Err(TrySendError::Full(v))`          |
| Rendezvous (N=0)     | capacity=0 edge case                | `sync_channel(0)` first-class         |
| Pipeline backpressure| Manual per-stage                    | Each stage's `sync_channel` auto-throttles |
| Async backpressure   | N/A                                 | `tokio::sync::mpsc` with `send().await` |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
