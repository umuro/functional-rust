📖 **[View on hightechmind.io →](https://hightechmind.io/rust/450-crossbeam-channel)**

---

# 450: Crossbeam Channels

**Difficulty:** 3  **Level:** Intermediate

Multi-producer, multi-consumer channels with bounded backpressure — the communication primitive `std::sync::mpsc` should have been.

## The Problem This Solves

`std::sync::mpsc` gives you *multi-producer, single-consumer* channels. The moment you want multiple consumers reading from the same channel — a classic worker pool — you're wrapping the receiver in `Arc<Mutex<Receiver>>` and hoping the contention isn't too bad. There's also no bounded channel in stable std (only `sync_channel` which is bounded but still single-consumer).

Real-world pipelines need both: **bounded channels** that apply backpressure when consumers fall behind, and **multi-consumer** so multiple workers can drain the same queue efficiently.

Crossbeam's `channel` crate provides MPMC (multi-producer, multi-consumer), bounded and unbounded, with zero contention on the fast path. It also integrates with `select!` for waiting on multiple channels at once.

## The Intuition

A post office with multiple clerks and a single ticket queue. Any clerk (consumer) can serve the next customer (message), and customers (producers) arrive from any direction. If the waiting room fills up (bounded), new customers wait outside until there's space. That's MPMC with backpressure.

## How It Works in Rust

1. **Unbounded channel** — grows without limit; producers never block:
   ```rust
   let (tx, rx) = crossbeam_channel::unbounded::<String>();
   ```
2. **Bounded channel** — producers block when the buffer is full:
   ```rust
   let (tx, rx) = crossbeam_channel::bounded::<String>(100);
   ```
3. **Clone senders and receivers** — both sides are `Clone`, enabling MPMC naturally:
   ```rust
   let tx2 = tx.clone();  // second producer
   let rx2 = rx.clone();  // second consumer
   ```
4. **Send and receive** — same API as `mpsc`:
   ```rust
   tx.send("hello".to_string())?;
   let msg = rx.recv()?;
   ```
5. **Channel close** — drop all senders; `recv()` returns `Err(RecvError)` to signal workers to exit.

## What This Unlocks

- **Worker pools** — fan out a single channel to N threads; each worker calls `rx.recv()` in a loop.
- **Backpressure pipelines** — bounded channels let slow consumers throttle fast producers automatically.
- **`select!` integration** — combine multiple channels in a single wait (see example 451).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Channel type | `Event.channel` (single-consumer) | Crossbeam MPMC |
| Bounded buffer | `Mutex` + manual size check | `bounded(n)` built-in |
| Multi-consumer | Not built-in | `rx.clone()` |
| Backpressure | Manual | Automatic when bounded |
