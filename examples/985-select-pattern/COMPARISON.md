# Select Pattern — Comparison

## Core Insight
`select` picks from whichever channel is ready first. In async runtimes this is a syscall (`epoll`/`kqueue`); in pure std Rust we spin with `try_recv` + `yield_now`. In OCaml, `Lwt.pick` or `Event.select` provides similar semantics.

## OCaml Approach
- `Lwt.pick [p1; p2]` returns the first promise to resolve, cancels others
- `Event.select [ev1; ev2]` for `Thread`/`Event` module
- Non-blocking: no built-in `try_receive` — must use `Thread.create` + timeout tricks
- `try_recv` simulation: poll with `Unix.select` for I/O events

## Rust Approach
- `rx.try_recv()` is non-blocking: `Ok(v)` | `Err(Empty)` | `Err(Disconnected)`
- Loop over all receivers, `yield_now()` when all empty
- Priority select: check high-priority channel first
- For true async select: `crossbeam::select!` macro (external crate)
- For async/await: `tokio::select!` or `futures::select!`

## Comparison Table

| Concept               | OCaml                           | Rust (std)                           |
|-----------------------|---------------------------------|--------------------------------------|
| Select first ready    | `Lwt.pick [p1; p2]`             | `try_recv` spin loop                 |
| Non-blocking recv     | No built-in (use timeout)       | `rx.try_recv()`                      |
| Distinguish sources   | Pattern match on promise list   | Match on `(r1, r2)` tuples           |
| Cancel others         | `Lwt.pick` cancels losers       | Just ignore other channels           |
| Priority channels     | Not built-in                    | Check high first in loop             |
| Efficient (no spin)   | `Lwt.pick` event-driven         | `crossbeam::select!` (external)      |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
