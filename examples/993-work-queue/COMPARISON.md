# Thread Pool / Work Queue — Comparison

## Core Insight
A thread pool reuses a fixed number of threads for many tasks, avoiding thread-creation overhead. The shared queue distributes work; each worker races to grab the next task. Shutdown = close the channel.

## OCaml Approach
- `Queue` + `Mutex` + `Condition` for the work channel
- Each worker: loop calling `recv_work` (blocks on condition variable)
- `close_chan` sets `closed = true` + broadcasts to wake all workers
- Workers see `None` on closed+empty channel and exit
- Tasks are `unit -> unit` closures

## Rust Approach
- `mpsc::channel::<Task>()` where `Task = Box<dyn FnOnce() + Send>`
- `Arc<Mutex<Receiver<Task>>>` — workers compete to lock and receive
- Drop `Sender` to close channel — workers get `Err` from `recv()` and break
- `JoinHandle` collected; `shutdown()` joins all workers
- Rayon or tokio for production use; this is the minimal std pattern

## Comparison Table

| Concept             | OCaml                               | Rust                                  |
|---------------------|-------------------------------------|---------------------------------------|
| Task type           | `unit -> unit`                      | `Box<dyn FnOnce() + Send + 'static>`  |
| Shared queue        | `Queue` + `Mutex` + `Condition`     | `mpsc::channel` + `Arc<Mutex<Rx>>`    |
| Worker loop         | `while recv_work ... do task ()`    | `loop { lock.recv().ok_or_else(break) }` |
| Shutdown signal     | `close_chan` + condition broadcast  | Drop `Sender` — channel closes        |
| Worker count        | `List.init n Thread.create`         | `(0..n).map(spawn).collect()`         |
| Result collection   | `Mutex`-protected list              | Separate `mpsc::channel` or `Mutex<Vec>` |
| Production version  | Domain pool (OCaml 5)               | Rayon / tokio                         |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
