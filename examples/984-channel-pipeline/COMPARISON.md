# Channel Pipeline — Comparison

## Core Insight
A channel pipeline is **function composition in concurrent space**: instead of `f ∘ g ∘ h`, you have `stage(f) | stage(g) | stage(h)` where `|` is a channel. This is Unix pipes, CSP, and the actor model all rolled into one pattern.

## OCaml Approach
- No built-in pipeline abstraction — build with `Thread` + `Queue` + `Mutex` + `Condition`
- Each stage is a thread looping over `recv` calls
- Close downstream by signalling `closed = true` + broadcasting
- More boilerplate, but same idea: transform + forward

## Rust Approach
- `pipeline_stage(rx, f)` creates a thread internally, returns new `Receiver`
- Composable: `let rx2 = pipeline_stage(pipeline_stage(rx0, f), g)`
- Channel closes automatically when `Sender` drops — propagates through pipeline
- `rx.iter()` is the idiomatic "read until closed" loop

## Comparison Table

| Concept             | OCaml                                  | Rust                                   |
|---------------------|----------------------------------------|----------------------------------------|
| Stage abstraction   | Manual thread + queue + mutex          | `fn pipeline_stage(rx, f) -> Receiver` |
| Close propagation   | Explicit `closed` flag + broadcast     | Drop `Sender` → next stage's `rx.iter()` stops |
| Back-pressure       | Queue fills up (manual limit needed)   | `sync_channel(n)` blocks producer      |
| Compose N stages    | Create N channel/thread pairs          | Chain `pipeline_stage` calls           |
| Collect output      | Loop over recv until None              | `rx.iter().collect()`                  |
| Parallelism         | One thread per stage                   | One thread per stage (same)            |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
