# MPSC Channel Basics — Comparison

## Core Insight
Channels are the functional alternative to shared mutable state: send immutable values between threads instead of sharing pointers. Both OCaml and Rust use typed channels, but Rust's MPSC is part of std while OCaml needs the `Thread` + `Event` or `Thread` + `Queue` + `Mutex` pattern.

## OCaml Approach
- `Event.channel ()` creates a synchronous channel (rendezvous semantics)
- `Event.sync (Event.send c v)` blocks until receiver is ready
- `Thread` + `Queue` + `Mutex` for asynchronous buffered channels
- No built-in MPSC — must implement with `Mutex` + `Queue` + `Condition`
- Type-safe: channels are parameterized by message type

## Rust Approach
- `mpsc::channel()` creates an unbounded asynchronous channel
- `mpsc::sync_channel(n)` creates a bounded channel (blocks on full)
- Multiple producers via `tx.clone()` — all senders share one receiver
- Channel closes automatically when all `Sender`s are dropped
- `rx.iter()` is idiomatic for "drain until closed"

## Comparison Table

| Concept             | OCaml                                  | Rust                              |
|---------------------|----------------------------------------|-----------------------------------|
| Create channel      | `Event.channel ()` / `Queue+Mutex`     | `mpsc::channel()`                 |
| Send message        | `Event.sync (Event.send c v)`          | `tx.send(v).unwrap()`             |
| Receive message     | `Event.sync (Event.receive c)`         | `rx.recv().unwrap()`              |
| Multiple producers  | Multiple threads with shared mutex     | `tx.clone()` per producer         |
| Close channel       | GC when last ref dropped               | Drop all `Sender`s                |
| Bounded buffer      | Manual ring buffer                     | `mpsc::sync_channel(n)`           |
| Drain all messages  | Loop until done signal                 | `rx.iter().collect()`             |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
