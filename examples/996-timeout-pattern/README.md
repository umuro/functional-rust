**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[timeout-pattern on hightechmind.io](https://hightechmind.io/posts/functional-rust/timeout-pattern)

---

## Problem Statement

Implement timeout patterns in Rust using `mpsc::recv_timeout`. Run an operation in a background thread and wait for its result with a deadline; return an error if the deadline expires. Also implement a "race" pattern where N tasks run concurrently and the first result wins — the async equivalent of `Lwt.pick`.

## Learning Outcomes

- Use `rx.recv_timeout(Duration)` to wait for a channel message with a deadline
- Implement `with_timeout<T, F>(timeout, f) -> Option<T>` — run `f` in a thread, return `None` on timeout
- Implement `race<T>(tasks, timeout) -> Option<T>` — spawn N tasks sharing one sender, return first result
- Handle `RecvTimeoutError::Timeout` vs `RecvTimeoutError::Disconnected` in match arms
- Understand why leftover threads continue running after timeout — graceful cancellation requires `AtomicBool`

## Rust Application

```rust
fn channel_with_timeout(delay_ms: u64, timeout_ms: u64) -> Result<i32, &'static str> {
    let (tx, rx) = mpsc::channel::<i32>();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        tx.send(42).ok();
    });
    match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(v)                                  => Ok(v),
        Err(RecvTimeoutError::Timeout)         => Err("timeout"),
        Err(RecvTimeoutError::Disconnected)    => Err("disconnected"),
    }
}

fn with_timeout<T, F>(timeout: Duration, f: F) -> Option<T>
where T: Send + 'static, F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<T>();
    thread::spawn(move || { tx.send(f()).ok(); });
    rx.recv_timeout(timeout).ok()
}

fn race<T: Send + 'static>(tasks: Vec<Box<dyn FnOnce() -> T + Send + 'static>>, timeout: Duration) -> Option<T> {
    let (tx, rx) = mpsc::channel::<T>();
    for task in tasks {
        let tx = tx.clone();
        thread::spawn(move || { tx.send(task()).ok(); });
    }
    drop(tx);  // only clones are in threads; drop original
    rx.recv_timeout(timeout).ok()
}
```

`recv_timeout` blocks until a message arrives or the duration expires. The spawned thread continues running even after `recv_timeout` returns a timeout — the `tx.send(result).ok()` silently discards the result when `rx` has been dropped.

`race` shares one `mpsc::Sender` among all tasks via `tx.clone()`. The first thread to complete sends its result; subsequent `send` calls return `Err(SendError)` because the receiver (timed-out caller) has dropped `rx`.

## OCaml Approach

```ocaml
open Lwt

(* Lwt.pick cancels all losers; Lwt.choose does not *)
let with_timeout duration f =
  Lwt.pick [
    (let* () = Lwt_unix.sleep duration in Lwt.return None);
    (let* v = f () in Lwt.return (Some v));
  ]

let race tasks timeout =
  Lwt.pick (
    (let* () = Lwt_unix.sleep timeout in Lwt.return None) ::
    List.map (fun f ->
      let* v = f () in Lwt.return (Some v)
    ) tasks
  )
```

`Lwt.pick` runs all promises concurrently and returns the first result, cancelling all others. `Lwt_unix.sleep` integrates with the Lwt scheduler for non-blocking sleep — unlike `Thread.sleep` which blocks the OS thread.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Timeout primitive | `recv_timeout(Duration)` | `Lwt.pick` + `Lwt_unix.sleep` |
| Cancellation | Thread leaks (manual `AtomicBool` needed) | `Lwt.pick` cancels losers automatically |
| Non-blocking sleep | `thread::sleep` blocks the OS thread | `Lwt_unix.sleep` is a non-blocking yield |
| Race result | First to send wins; rest silently fail | First resolved; rest are cancelled |

`thread::sleep` blocks the OS thread — spawning many concurrent timeouts wastes one thread each. Use `tokio::time::timeout` for non-blocking async timeouts in production.

## Exercises

1. Implement `with_cancel<T, F>(cancel: Arc<AtomicBool>, f: F) -> Option<T>` — `f` checks `cancel` periodically and stops early.
2. Implement `retry_with_timeout(n, timeout, f)` — retry up to `n` times, each attempt capped at `timeout`.
3. Implement `first_k_of_n<T>(tasks, k, timeout) -> Vec<T>` — collect the first `k` results.
4. Rewrite `with_timeout` using `tokio::time::timeout` and compare implementation complexity.
5. Benchmark: 100 concurrent `with_timeout` calls using threads vs using `tokio::spawn` + `tokio::time::timeout`.
