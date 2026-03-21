[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1002 — Backpressure
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Demonstrate backpressure in a concurrent pipeline using `mpsc::sync_channel` — a bounded channel where the sender blocks when the buffer is full. Show `try_send` for non-blocking drop-on-full semantics, and a multi-stage bounded pipeline. Compare with OCaml's `Mutex`/`Condition`-based bounded channel implementation.

## Learning Outcomes

- Use `mpsc::sync_channel(N)` to create a channel that blocks senders when `N` items are buffered
- Understand backpressure: the producer is implicitly rate-limited by the consumer's speed
- Use `try_send` for non-blocking send that returns `TrySendError::Full` when the buffer is full
- Build a two-stage pipeline with bounded channels between stages
- Map Rust's `sync_channel` to OCaml's manual `Mutex` + `Condition` bounded queue
- Recognise backpressure as a fundamental flow-control pattern in streaming systems

## Rust Application

`mpsc::sync_channel(3)` creates a channel with buffer size 3. The producer spawns a thread sending 9 items; `tx.send(i)` blocks when 3 items are already buffered — the sender waits until the consumer frees space. `try_send` returns `Err(TrySendError::Full)` immediately without blocking, enabling drop-on-full semantics. The bounded pipeline chains two `sync_channel(2)` channels with a processing thread in between — each stage is rate-limited by the next.

## OCaml Approach

OCaml's bounded channel uses `Queue.t` with `Mutex` and two `Condition` variables: `not_full` (producer waits) and `not_empty` (consumer waits). `send_bounded` locks the mutex, waits on `not_full` while the queue is at capacity, pushes the value, and signals `not_empty`. This is the standard `Condition`-based producer-consumer pattern, equivalent to Rust's `sync_channel` semantics but implemented manually.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Bounded channel | `mpsc::sync_channel(N)` | Manual `Mutex` + `Condition` |
| Blocking send | `tx.send(v)` blocks when full | `Condition.wait not_full m` |
| Non-blocking | `tx.try_send(v)` → `TrySendError` | Conditional `if Queue.length < capacity` |
| Consumer | `rx.iter()` | `recv_bounded` with `Condition.wait not_empty` |
| Pipeline | Chain `sync_channel`s | Chain `make_bounded_chan`s |
| Code length | Short (stdlib) | Verbose (manual synchronisation) |

Backpressure prevents fast producers from overwhelming slow consumers. Without it, unbounded buffers grow until memory is exhausted. `sync_channel` is Rust's built-in solution; the manual OCaml implementation shows the underlying mechanism.

## Exercises

1. Add a timeout to the blocking send: use `tx.send_timeout(v, Duration::from_millis(100))` (if available) or implement via `try_send` + `sleep` loop.
2. Add a drop counter: modify `try_send_demo` to return `(accepted, dropped, drained)` and verify `accepted == drained`.
3. Implement a three-stage pipeline: producer → transform → consumer with bounded channels between each stage.
4. Benchmark `sync_channel(1)` vs `sync_channel(100)` for throughput on a CPU-bound transform stage.
5. In OCaml, add a `try_send` variant to the bounded channel that returns `false` instead of blocking when full.
