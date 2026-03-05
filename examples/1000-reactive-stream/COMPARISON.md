# Reactive Stream — Comparison

## Core Insight
Reactive streams are **push-based lazy sequences**: a producer pushes values to a subscriber. Unlike iterators (pull-based), the producer controls timing. Operators like `map`/`filter` wrap one observable in another — forming a lazy chain that only runs when subscribed.

## OCaml Approach
- `observable = { subscribe: observer -> unsubscribe }` as a record
- `observer = { on_next; on_error; on_complete }` callbacks
- `map`/`filter`/`take` create new observables wrapping the old
- `Subject` (hot observable): broadcasts to multiple subscribers
- Closely mirrors RxJS architecture

## Rust Approach
- `Observable<T>` wraps `Box<dyn Fn(&mut dyn Observer<T>)>`
- `Observer<T>` trait with `on_next`, `on_error`, `on_complete`
- `FnObserver` adapter for closure-based observers
- `Rc<RefCell<_>>` for shared state within single-threaded observable
- `obs_map`, `obs_filter`, `obs_take` as free functions returning new Observable
- For production: `futures::Stream` or the `rxrust` crate

## Comparison Table

| Concept              | OCaml                                 | Rust                                   |
|----------------------|---------------------------------------|----------------------------------------|
| Observable type      | `{ subscribe: observer -> unsub }`    | `struct Observable<T> { subscribe_fn }` |
| Observer type        | `{ on_next; on_error; on_complete }`  | `trait Observer<T>`                    |
| Map operator         | `map f obs` → new observable          | `obs_map(source, f)` → `Observable<U>` |
| Filter operator      | `filter pred obs`                     | `obs_filter(source, pred)`             |
| Take operator        | `take n obs`                          | `obs_take(source, n)`                  |
| Hot observable       | `Subject` type                        | Manual with `Arc<Mutex<Vec<...>>>`     |
| Production           | `Lwt_react`, `rxocaml`               | `futures::Stream`, `rxrust`            |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
