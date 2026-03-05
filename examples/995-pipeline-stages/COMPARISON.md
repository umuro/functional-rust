# N-Stage Streaming Pipeline — Comparison

## Core Insight
Channel pipelines are **lazy iterators that run in parallel**: while stage N processes item K, stage N+1 can process item K-1. This is the concurrent equivalent of `Seq.map f |> Seq.filter p |> Seq.map g`.

## OCaml Approach
- `make_stage in_c f` creates an output channel and spawns a thread
- `make_filter in_c pred` filters items — only passes matching ones
- Chained: `let c2 = make_stage (make_stage c0 f) g`
- Backpressure: natural if using bounded channels
- Each stage closes its output when input is exhausted

## Rust Approach
- `map_stage(rx, f)` and `filter_stage(rx, pred)` return `Receiver<T>`
- Composable by chaining: `map_stage(filter_stage(rx, pred), f)`
- `flat_map_stage` for one-to-many expansion
- Thread runs `for item in rx.iter()` — stops when channel closes
- Stages run truly in parallel — all cores can be utilized

## Comparison Table

| Stage type         | OCaml                             | Rust                              |
|--------------------|-----------------------------------|-----------------------------------|
| Map                | `make_stage in_c f`               | `map_stage(rx, f)`                |
| Filter             | `make_filter in_c pred`           | `filter_stage(rx, pred)`          |
| Flat-map           | Custom stage with multiple sends  | `flat_map_stage(rx, f)`           |
| Compose 2 stages   | `make_stage (make_stage c0 f) g`  | `map_stage(map_stage(rx, f), g)`  |
| Backpressure       | Bounded queue (manual)            | `sync_channel(n)` for stage tx    |
| Stage count        | Any N                             | Any N                             |
| Channel close      | Explicit `close_chan out_c`       | Drop `tx` (RAII)                  |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
