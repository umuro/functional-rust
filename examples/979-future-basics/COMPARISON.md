# Future/Promise Basics — Comparison

## Core Insight
Both OCaml's Lwt and Rust's async/await express the **Future monad**: a computation that produces a value later. The monad laws hold: `return` wraps a value, `bind` chains computations, `map` transforms results.

## OCaml Approach
- `Lwt.return x` wraps a value in an already-resolved promise
- `Lwt.bind p f` (or `let*`) chains: when `p` resolves, pass result to `f`
- `Lwt.map f p` transforms the resolved value with `f`
- Simulated here as `unit -> 'a` thunks (lazy evaluation)
- `Lwt_main.run` drives the event loop to completion

## Rust Approach
- `async fn` creates a state machine implementing `Future`
- `.await` is desugared `bind`: suspend until the sub-future resolves
- `async { expr }` is an async block (anonymous future)
- Futures are **lazy** — nothing runs until polled by an executor
- A minimal `block_on` executor can drive immediate futures without tokio

## Comparison Table

| Concept         | OCaml (Lwt)                    | Rust                          |
|-----------------|--------------------------------|-------------------------------|
| Return / wrap   | `Lwt.return x`                 | `async { x }` or ready future |
| Bind / chain    | `Lwt.bind p f` / `let*`        | `p.await` inside `async fn`  |
| Map / transform | `Lwt.map f p`                  | `async { f(p.await) }`        |
| Run / execute   | `Lwt_main.run p`               | `executor::block_on(f)`       |
| Laziness        | Explicit thunk                 | Implicit — poll-driven        |
| Error handling  | `Lwt_result.t`                 | `async fn -> Result<T,E>`     |
| Custom future   | `Lwt.task` + resolver          | `impl Future for T`           |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
