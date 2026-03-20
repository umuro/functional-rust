📖 **[View on hightechmind.io →](https://hightechmind.io/rust/327-spawn-tasks)**

---

# 327: Spawning Concurrent Tasks
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Fire-and-forget background tasks, parallel work distribution, and pipeline stages all require spawning independent units of work. `thread::spawn` (synchronous) and `tokio::spawn` (async) start tasks that run independently and optionally return results via `JoinHandle`. Understanding how to spawn tasks, collect results, and handle panics is foundational for concurrent Rust programming.

## Learning Outcomes

- Use `thread::spawn()` to start independent background tasks
- Collect results from multiple threads using `JoinHandle::join()`
- Handle panics in spawned threads via `join()` returning `Result`
- Understand the difference between detached tasks (fire-and-forget) and joined tasks (collect results)

## Rust Application

Thread spawning and result collection:

```rust
pub fn spawn_worker(id: usize, delay_ms: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        format!("worker-{} done after {}ms", id, delay_ms)
    })
}

pub fn collect_results(count: usize) -> Vec<String> {
    let handles: Vec<_> = (0..count).map(|i| spawn_worker(i, 10)).collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
// All workers run concurrently; results collected after all complete
```

## OCaml Approach

OCaml 5's `Domain` module provides multi-core parallelism. For Lwt, `Lwt.async` fires background tasks, and `Lwt.join` waits for all:

```ocaml
let spawn_worker id =
  Domain.spawn (fun () ->
    Unix.sleepf 0.01;
    Printf.sprintf "worker-%d done" id)

let collect_results count =
  let domains = List.init count spawn_worker in
  List.map Domain.join domains
```

## Key Differences

1. **Panic isolation**: Rust's `join()` returns `Result<T, Box<dyn Any>>` — a panicking thread produces `Err`; OCaml domains propagate exceptions through `Domain.join`.
2. **Return value**: `JoinHandle<T>::join()` returns the thread's return value; async `tokio::task::JoinHandle<T>` does the same asynchronously.
3. **Detach**: Not calling `join()` on a `JoinHandle` causes the thread to run until completion but its result is discarded; the thread continues independently.
4. **Stack size**: `thread::Builder::new().stack_size(N)` customizes thread stack size for large recursive operations.

## Exercises

1. Spawn N threads each computing the prime numbers in a range, then collect and merge all results.
2. Handle panicking threads: spawn threads that may panic, use `join()` to detect panics, and recover gracefully.
3. Implement a parallel `map_reduce` that maps a function over chunks in parallel and reduces the chunk results to a final answer.
