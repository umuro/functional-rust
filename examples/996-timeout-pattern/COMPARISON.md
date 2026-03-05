# Timeout Pattern — Comparison

## Core Insight
Timeouts express "I'd rather fail fast than wait forever." In OCaml's Lwt: `Lwt.pick [op; sleep]` races two promises and takes the first. In Rust std: `recv_timeout` is the primitive, or wrap in a thread for arbitrary operations.

## OCaml Approach
- `Lwt.pick [operation; Lwt_unix.sleep timeout]` — cancels the loser
- Thread-based: spawn worker, timed `Condition.wait` with deadline
- OCaml cannot kill threads — worker keeps running after "timeout"
- `Unix.gettimeofday` for wall-clock deadline tracking

## Rust Approach
- `rx.recv_timeout(Duration)` → `Result<T, RecvTimeoutError>`
- `RecvTimeoutError::Timeout` vs `RecvTimeoutError::Disconnected`
- `with_timeout(dur, f)` pattern: spawn thread, recv_timeout, discard handle
- The "lost" thread keeps running but its channel is dropped — no cleanup needed
- `race(tasks, timeout)` for "first-of-N" / Lwt.pick over multiple computations

## Comparison Table

| Concept              | OCaml (Lwt)                        | Rust                                |
|----------------------|------------------------------------|-------------------------------------|
| Timeout primitive    | `Lwt_unix.sleep t`                 | `rx.recv_timeout(Duration::from_millis(t))` |
| Race two futures     | `Lwt.pick [f; sleep t]`            | `race([task], timeout)`             |
| Timeout result       | `exception` or `None`              | `Err(RecvTimeoutError::Timeout)`    |
| Cancellation         | Lwt cancels the losing promise     | Thread keeps running (can't kill)   |
| Timed channel recv   | Manual Condition.wait with deadline| `rx.recv_timeout(dur)`              |
| Wrap arbitrary work  | `Lwt.wrap (fun () -> ...)`         | `with_timeout(dur, || f())`         |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
