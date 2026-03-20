📖 **[View on hightechmind.io →](https://hightechmind.io/rust/451-crossbeam-select)**

---

# 451: Crossbeam Select — Multiplexing Channels
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A thread watching multiple channels needs to respond to whichever has data first, without blocking on one while the other has messages. Go's `select` statement solves this natively. Rust's `std::sync::mpsc` has no select mechanism — you'd need to poll with `try_recv` in a loop, wasting CPU. `crossbeam::select!` provides efficient blocking select across multiple channels: the calling thread blocks until any channel has a message, then executes the matching arm.

Channel select appears in event-driven systems, timeouts combined with work channels, message routing, and any pattern where a thread must respond to multiple event sources.

## Learning Outcomes

- Understand why channel select is needed (avoid blocked on wrong channel)
- Learn how `crossbeam::select!` blocks efficiently until any channel is ready
- See the polling approach as the naive alternative with busy-wait overhead
- Understand how to combine select with timeouts using a timeout channel
- Learn the use cases: event loops, control channels, multi-source aggregation

## Rust Application

In `src/lib.rs`, `poll_select` implements select via polling `try_recv` in a loop with a 1ms sleep — correct but wasteful. The test demonstrates receiving from whichever of two channels has a message first. The real `crossbeam::select!` macro would replace this with a blocking efficient wait: `select! { recv(rx1) -> v => ..., recv(rx2) -> v => ... }`. The `SelectResult` enum captures which channel produced the message.

## OCaml Approach

OCaml's `Event` module has `Event.select [Event.receive ch1; Event.receive ch2]` for channel-level select — blocking until any event is ready. `Async.choose` and `Lwt.pick` provide async-style select. `Domainslib.Chan.recv_poll` enables non-blocking attempts in OCaml 5.x. Event-driven OCaml programming uses these primitives to multiplex across I/O, timers, and inter-domain communication.

## Key Differences

1. **Blocking vs. polling**: `crossbeam::select!` blocks without CPU waste; the std polling approach wastes CPU on the spin loop.
2. **Fairness**: `crossbeam::select!` handles fairness for multiple ready channels; the polling approach picks the first channel it checks.
3. **Timeout integration**: `crossbeam::select!` with `crossbeam_channel::after(duration)` adds timeout cleanly; polling needs explicit `Instant::now()` tracking.
4. **Go comparison**: Go's `select` is built into the language; Rust requires the `crossbeam` crate or `tokio::select!` for async code.

## Exercises

1. **Control channel**: Add a "shutdown" channel to a long-running worker. Use `crossbeam::select!` to receive either work items or shutdown signals, stopping when shutdown arrives.
2. **Timeout with select**: Implement `receive_with_timeout<T>(rx: &Receiver<T>, timeout: Duration) -> Option<T>` using `crossbeam::select!` with a `crossbeam_channel::after(timeout)` timeout channel.
3. **Priority channels**: Use two channels (high_priority, low_priority) and `select!`. First always check high_priority; only check low_priority when high_priority is empty. Implement this fairly (low priority eventually served even with continuous high-priority traffic).
