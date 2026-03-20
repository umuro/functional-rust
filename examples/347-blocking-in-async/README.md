📖 **[View on hightechmind.io →](https://hightechmind.io/rust/347-blocking-in-async)**

---

# 347: Blocking in Async
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Async runtimes like Tokio use a fixed pool of threads to drive many concurrent tasks. If any task calls a blocking operation (CPU-intensive computation, synchronous I/O, `thread::sleep`), it stalls the entire thread, preventing all other tasks on that thread from making progress. This is the "blocking in async" problem — it can silently starve the runtime of threads, causing latency spikes and timeouts. The solution is to offload blocking work to a dedicated thread pool (`tokio::task::spawn_blocking`) so the async thread pool remains responsive. Understanding this boundary is critical for mixing synchronous libraries (database drivers, compression codecs) with async code.

## Learning Outcomes

- Identify operations that are unsafe to call directly in async code
- Use `thread::spawn` (or `tokio::task::spawn_blocking`) to offload blocking work
- Run a batch of blocking items in parallel by spawning one thread per item
- Understand that `spawn_blocking` communicates results back via a oneshot channel
- Recognize that CPU-bound work is also "blocking" from the async runtime's perspective
- Know the rule: never hold a `Mutex` lock across an `.await` point

## Rust Application

```rust
use std::thread;
use std::time::Duration;

pub fn blocking_computation(n: u64) -> u64 {
    thread::sleep(Duration::from_millis(10)); // simulates blocking I/O
    (1..=n).product()
}

// In a real async context: tokio::task::spawn_blocking(|| blocking_computation(n))
pub fn spawn_blocking<F, R>(f: F) -> thread::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    thread::spawn(f)
}

pub fn run_blocking_batch<T, R, F>(items: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + Clone + 'static,
{
    let handles: Vec<_> = items.into_iter().map(|item| {
        let f = f.clone();
        thread::spawn(move || f(item))
    }).collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
```

With Tokio: `tokio::task::spawn_blocking(|| heavy_sync_work()).await` offloads to Tokio's dedicated blocking thread pool (separate from the async worker threads). The result is returned as a `JoinHandle<R>` that you `.await`.

## OCaml Approach

Lwt uses `Lwt_preemptive.detach` to run blocking code in a thread pool:

```ocaml
let blocking_work n =
  Unix.sleepf 0.01;
  List.fold_left ( * ) 1 (List.init n (fun i -> i + 1))

let async_wrapper n =
  Lwt_preemptive.detach (fun () -> blocking_work n) ()
```

`detach` runs the function in a preemptive thread, returning an Lwt promise. The Lwt event loop is not blocked — it continues handling other promises while the thread runs. This is the direct equivalent of `spawn_blocking`.

## Key Differences

| Aspect | Rust `spawn_blocking` | OCaml `Lwt_preemptive.detach` |
|--------|----------------------|-------------------------------|
| Thread pool | Separate from async workers | `Lwt_preemptive` thread pool |
| Return type | `JoinHandle<R>` / `async` result | Lwt promise |
| Backpressure | Tokio limits pool size | Configurable thread pool size |
| Composability | `.await` in async context | `let%lwt` in Lwt context |
| Detection of mistakes | None at compile time | None at compile time |

## Exercises

1. **Detect starvation**: In a Tokio runtime with 2 worker threads, spawn 3 tasks — 2 call `thread::sleep(1s)` directly (blocking!) and 1 prints a message every 100ms; observe that the print task starves; fix by using `spawn_blocking`.
2. **Parallel batch with results**: Extend `run_blocking_batch` to return `Vec<Result<R, String>>` where each thread's panic is caught with `thread::catch_unwind` and converted to `Err`.
3. **Bounded concurrency**: Limit `run_blocking_batch` to run at most `N` threads simultaneously using a semaphore (`Arc<(Mutex<usize>, Condvar)>`); test with 20 items and limit 4.
