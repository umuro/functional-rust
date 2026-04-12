📖 **[View on hightechmind.io →](https://hightechmind.io/rust/325-select-futures)**

---

# 325: Racing Futures with select!
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Sometimes you want the first result from multiple concurrent operations — a timeout competing with an operation, querying multiple replicas and using the fastest response, or cancelling work when a stop signal arrives. The `select!` macro (in `tokio` or `futures`) polls multiple futures and returns when the first one completes, cancelling the others. This is the fundamental tool for implementing timeouts, fallbacks, and cancellation in async code.

## Learning Outcomes

- Understand `select!` as polling multiple futures and returning on first completion
- Distinguish `select!` (first wins) from `join!` (all must complete)
- Implement racing with timeouts as a common `select!` pattern
- Recognize that unfinished futures in `select!` are dropped (cancelled)

## Rust Application

Thread-based racing simulation shows the concept:

```rust
pub fn race<T>(
    tasks: Vec<(&'static str, Box<dyn FnOnce() -> T + Send>)>
) -> (&'static str, T)
where T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    for (label, f) in tasks {
        let tx = tx.clone();
        thread::spawn(move || { let _ = tx.send((label, f())); });
    }
    rx.recv().expect("all senders dropped")  // First result wins
}
```

In Tokio: `tokio::select! { v = fut1 => handle_v, _ = timeout => handle_timeout }`.

## OCaml Approach

OCaml's `Lwt.pick` takes a list of promises and returns the first to resolve, cancelling the others:

```ocaml
(* Lwt.pick: first to resolve wins, others are cancelled *)
let* result = Lwt.pick [
  Lwt.map (fun v -> `Result v) (fetch ());
  Lwt.map (fun () -> `Timeout) (Lwt_unix.sleep timeout_secs);
]
```

## Key Differences

1. **Cancellation**: Rust's `select!` drops (cancels) unfinished futures when one completes; Lwt's `pick` actively cancels losers.
2. **Macro syntax**: `tokio::select!` uses Rust macro syntax with `pattern = future => body` arms; Lwt's `pick` is a regular function.
3. **Non-determinism**: When multiple futures complete simultaneously, `select!` chooses one biased toward the first arm by default; `tokio::select! { biased; }` makes this explicit.
4. **Timeout pattern**: `tokio::time::timeout(dur, future)` is a specialized `select!` for adding a deadline to any future.

## Exercises

1. Implement a `with_timeout<T>(f: impl FnOnce() -> T, timeout: Duration) -> Option<T>` that returns `None` if the operation takes too long.
2. Race two "replicas" returning the same type and use the first result, ensuring both tasks are started before waiting.
3. Implement a cancellation-aware worker: the worker computes a value, but can be interrupted by a cancellation signal arriving first.
