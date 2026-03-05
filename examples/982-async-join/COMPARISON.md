# Join Parallel Async — Comparison

## Core Insight
`Lwt.both` and `thread::spawn + join` both express "run two things concurrently, wait for both". The key difference: Lwt uses cooperative concurrency on one thread; Rust `std::thread` uses OS threads with true parallelism.

## OCaml Approach
- `Lwt.both p1 p2` runs both promises on the event loop concurrently
- Returns `(v1, v2)` when both resolve
- `Lwt.all [p1; p2; p3]` for N promises
- Cooperative — yields at I/O points, single-threaded
- For true parallelism: OCaml 5 Domains or `Thread` + mutexes

## Rust Approach
- `thread::spawn(f)` starts a real OS thread, returns `JoinHandle<T>`
- `handle.join()` blocks until the thread completes, returns `Result<T>`
- True parallelism — all cores can be used simultaneously
- `Vec<JoinHandle>` pattern for N parallel tasks
- `Send + 'static` bounds ensure data is safe to transfer

## Comparison Table

| Concept           | OCaml (Lwt)                    | Rust                              |
|-------------------|--------------------------------|-----------------------------------|
| Run two in parallel| `Lwt.both p1 p2`              | `spawn(f1); spawn(f2); join both` |
| Run N in parallel | `Lwt.all [p1; p2; ...]`        | `tasks.map(spawn).map(join)`      |
| Wait for result   | `let* (a,b) = Lwt.both ...`   | `h.join().unwrap()`               |
| Concurrency model | Cooperative / event loop       | True parallelism (OS threads)     |
| Error propagation | `Lwt_result.both`              | `h.join()` returns `Result`       |
| Data sharing      | Shared heap (GC)               | `Send + 'static` + `Arc`         |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
