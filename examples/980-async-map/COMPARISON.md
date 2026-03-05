# Map over Async — Comparison

## Core Insight
`map` lifts a pure function `f: A -> B` into async context without needing to chain two `bind`s. In both OCaml and Rust it's a derived operation: `map f m = bind m (fun x -> return (f x))`.

## OCaml Approach
- `Lwt.map f promise` transforms the resolved value without blocking
- Satisfies functor laws: `map Fun.id = Fun.id`, `map (f ∘ g) = map f ∘ map g`
- Can be composed in a pipeline: `promise |> Lwt.map f |> Lwt.map g`
- Lwt also provides `Lwt.( >|= )` as infix map operator

## Rust Approach
- `async { f(fut.await) }` is the idiomatic inline map
- Can be a helper `async fn map(fut, f) -> U { f(fut.await) }`
- Functor laws hold because `async`/`await` is pure transformation
- No allocation beyond the state machine

## Comparison Table

| Concept           | OCaml (Lwt)                    | Rust                              |
|-------------------|--------------------------------|-----------------------------------|
| Map a future      | `Lwt.map f promise`            | `async { f(fut.await) }`          |
| Infix map         | `promise >|= f`                | (no built-in infix, use closure)  |
| Identity law      | `Lwt.map Fun.id p = p`         | `async { id(p.await) } = p`       |
| Composition law   | `map (f∘g) = map f ∘ map g`   | Same via async nesting             |
| Map via bind      | `bind p (fun x -> return f x)` | `async { f(p.await) }`            |
| Allocation        | Lwt promise allocation         | Zero-cost state machine            |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
