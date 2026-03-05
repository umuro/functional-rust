# Actor Pattern — Comparison

## Core Insight
Actors replace shared mutable state with **message passing to a single owner**. The actor owns its state exclusively — no locks needed because only one thread touches it. The channel IS the thread-safe boundary.

## OCaml Approach
- Thread + Queue + Mutex + Condition = manual mailbox
- Message type is a variant; actor loops over `recv` calls
- Request-response: embed `result ref` + `Condition` in message
- More boilerplate, same concept as Rust
- Akka/Erlang heritage — functional languages pioneered this pattern

## Rust Approach
- `mpsc::channel::<Message>()` is the mailbox
- `thread::spawn` runs the actor loop: `for msg in rx.iter() { match msg }`
- Request-response: embed `mpsc::Sender<Reply>` in the message variant
- Struct wraps the `Sender<Message>` — provides typed API methods
- No `Arc<Mutex<...>>` needed — the actor owns all state

## Comparison Table

| Concept              | OCaml                              | Rust                                |
|----------------------|------------------------------------|-------------------------------------|
| Mailbox              | `Queue` + `Mutex` + `Condition`    | `mpsc::channel::<Msg>()`            |
| Message type         | Variant type / ADT                 | `enum Message { ... }`              |
| Actor loop           | `while running { match recv }` | `for msg in rx.iter() { match msg }` |
| Request-response     | Embed `result ref` + Condition     | Embed `Sender<Reply>` in variant    |
| State ownership      | `ref` inside actor closure         | Local variable in thread closure    |
| Shutdown             | `Shutdown` variant                 | `Shutdown` variant (or drop tx)     |
| No lock needed       | Yes — one owner                    | Yes — one owner                     |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
