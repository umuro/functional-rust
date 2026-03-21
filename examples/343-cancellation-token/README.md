📖 **[View on hightechmind.io →](https://hightechmind.io/rust/343-cancellation-token)**

---

# 343: Cancellation Token
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Long-running background workers need a way to stop cleanly when the application shuts down or a user cancels an operation. Forcibly killing threads (`pthread_cancel` in C) is unsafe — it can leave mutexes locked or resources open. Cooperative cancellation instead asks workers to check a shared flag and stop themselves at defined safe points. The cancellation token pattern formalizes this: a shared atomic boolean that any thread can check and any caller can flip. This is the same pattern used by Linux kernel's `kthread_should_stop()`, Java's `Thread.interrupted()`, and the `CancellationToken` in .NET and Tokio.

## Learning Outcomes

- Use `Arc<AtomicBool>` to share a cancellation signal across threads safely
- Use `Ordering::Relaxed` for the cancellation check — correctness doesn't require sequential consistency here
- Implement worker loops that check `is_cancelled()` at each iteration
- Drop senders / signal tokens to trigger graceful shutdown
- Understand why cooperative cancellation is safer than forced termination
- Recognize where to place cancellation checkpoints in compute-bound loops

## Rust Application

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn make_token() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

fn cancel(token: &AtomicBool) {
    token.store(true, Ordering::Relaxed);
}

fn is_cancelled(token: &AtomicBool) -> bool {
    token.load(Ordering::Relaxed)
}

fn worker(token: Arc<AtomicBool>, name: String) -> String {
    let mut count = 0u64;
    while !is_cancelled(&token) && count < 1_000_000 {
        count += 1;
        // real work here
    }
    format!("{} did {} iterations", name, count)
}

fn run_workers(n: usize) -> Vec<String> {
    let token = make_token();
    let handles: Vec<_> = (0..n).map(|i| {
        let t = token.clone();
        thread::spawn(move || worker(t, format!("worker-{}", i)))
    }).collect();
    // cancel after some time
    cancel(&token);
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
```

`Ordering::Relaxed` is sufficient for a cancellation flag because we only need the value to eventually become visible — we don't need to synchronize other memory operations around it. If you need to ensure work done before the flag is set is visible after the flag is read, use `Release`/`Acquire` ordering.

## OCaml Approach

OCaml 5 domains use `Atomic` references for shared flags:

```ocaml
let cancelled = Atomic.make false in
let worker () =
  while not (Atomic.get cancelled) do
    (* work *)
  done
in
let d = Domain.spawn worker in
Unix.sleepf 0.1;
Atomic.set cancelled true;
Domain.join d
```

In OCaml 4 with threads, a `ref` plus `Mutex` achieves the same — the GIL often makes plain `ref` safe for simple boolean flags, but `Atomic` is the correct approach for multi-domain OCaml 5 programs.

## Key Differences

| Aspect | Rust `Arc<AtomicBool>` | OCaml `Atomic.make bool` |
|--------|------------------------|--------------------------|
| Sharing mechanism | Explicit `Arc::clone` | GC handles sharing |
| Memory ordering | Explicit (`Relaxed`/`Acquire`/`Release`) | Sequential consistency by default |
| Compile-time safety | `AtomicBool: Sync + Send` verified | No equivalent check |
| Tokio integration | `tokio_util::CancellationToken` (structured) | `Lwt.cancel` for async tasks |

## Exercises

1. **Timeout-based cancellation**: Start a worker and cancel it after 100ms using `thread::sleep` in the main thread; report how many iterations the worker completed.
2. **Multi-signal cancellation**: Create a token that requires two callers to both signal before workers stop — implement this as a counter (`AtomicUsize`) that workers check against a threshold.
3. **Graceful shutdown with cleanup**: Add a second channel alongside the cancellation token so workers can send a "cleanup complete" message before exiting; have the main thread wait for all cleanup confirmations.
