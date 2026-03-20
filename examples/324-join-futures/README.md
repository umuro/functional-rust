📖 **[View on hightechmind.io →](https://hightechmind.io/rust/324-join-futures)**

---

# 324: Running Futures Concurrently with join!

## Problem Statement

Sequential async execution wastes time when multiple operations are independent: fetching user data and fetching their posts can happen simultaneously. `join!` (or `tokio::join!`) starts all futures concurrently and waits for all to complete. The total time equals the slowest task, not the sum of all tasks. This is the fundamental tool for parallelizing independent I/O operations in async Rust.

## Learning Outcomes

- Understand that `join!` starts all futures simultaneously and waits for all to complete
- Distinguish `join!` (wait for all) from `select!` (wait for first)
- Recognize that total time is `max(task_times)` not `sum(task_times)` with `join!`
- Apply `join!` to fetch independent data sources concurrently

## Rust Application

Thread-based simulation demonstrates the concept before a Tokio runtime is available:

```rust
// Phase 1: spawn all tasks simultaneously (all start now)
pub fn join_all<T, F>(tasks: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handles: Vec<_> = tasks.into_iter().map(|f| thread::spawn(f)).collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
// Total time: max(individual times), not sum
```

In Tokio: `let (a, b) = tokio::join!(fetch_a(), fetch_b())` — both futures polled concurrently.

## OCaml Approach

OCaml's `Lwt.both` and `Lwt.all` provide equivalent concurrent execution:

```ocaml
(* Wait for both: total time = max(a, b) *)
let* (a, b) = Lwt.both (fetch_a ()) (fetch_b ())

(* Wait for all: Lwt.all returns list of results *)
let* results = Lwt.all [fetch_a (); fetch_b (); fetch_c ()]
```

## Key Differences

1. **Macro vs function**: Rust's `tokio::join!` is a macro enabling heterogeneous future types; `futures::join_all()` is a function for homogeneous types.
2. **Error propagation**: `try_join!` fails fast if any future returns `Err`; `join!` returns a tuple including errors.
3. **Structured concurrency**: `join!` enforces a structured scope — all spawned work completes before proceeding; `spawn()` allows detached tasks.
4. **vs parallel**: `join!` is concurrent (single thread, cooperative), not necessarily parallel; `rayon::join!` is parallel (multi-thread).

## Exercises

1. Time the difference between sequential and `join_all` concurrent execution of 5 tasks with varying delays — measure wall-clock time.
2. Implement a `fetch_all(urls: Vec<Url>) -> Vec<Result<Response, Error>>` that fetches all URLs concurrently using `join_all`.
3. Show that `join!` with 3 tasks of 100ms, 200ms, and 300ms takes ~300ms total, not ~600ms as sequential would.
