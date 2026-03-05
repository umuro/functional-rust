# Thread-Local Storage — Comparison

## Core Insight
Thread-local storage is the answer to "I want mutable state but don't want synchronization overhead." Each thread has its own private copy — no races possible, no locks needed.

## OCaml Approach
- OCaml 5: `Domain.DLS.new_key` / `Domain.DLS.get` / `Domain.DLS.set` (domain-local)
- OCaml < 5: Simulate with `Thread.id` → `Hashtbl` (requires mutex for the table itself)
- Domains ≠ threads in OCaml 5 — one domain can run many lightweight threads
- Typical use: per-domain RNG seeds, error buffers, caches

## Rust Approach
- `thread_local! { static NAME: Type = init; }` declares the variable
- `.with(|v| ...)` is the only access method — ensures scoped lifetime
- Usually paired with `Cell<T>` (copy types) or `RefCell<T>` (arbitrary types)
- Initialized lazily on first access per thread
- Dropped when thread exits

## Comparison Table

| Concept              | OCaml                                | Rust                              |
|----------------------|--------------------------------------|-----------------------------------|
| Declare              | `Domain.DLS.new_key (fun () -> init)`| `thread_local! { static X: T }`  |
| Read                 | `Domain.DLS.get key`                 | `X.with(\|v\| *v.borrow())`       |
| Write                | `Domain.DLS.set key val`             | `X.with(\|v\| *v.borrow_mut() = x)`|
| Interior mutability  | Mutable by nature                    | `Cell<T>` or `RefCell<T>`         |
| Initialization       | Closure passed at creation           | Expression in macro               |
| Isolation            | Per-domain (not per-thread in OCaml 5)| Per-OS-thread                    |
| No sync needed       | Yes                                  | Yes — the whole point             |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
