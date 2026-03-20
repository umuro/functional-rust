📖 **[View on hightechmind.io →](https://hightechmind.io/rust/922-cancellation-token)**

---

# 922-cancellation-token — Cancellation Token
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Long-running operations — large file downloads, iterative computations, background scans — must be stoppable. Stopping a thread forcefully from outside is unsafe in any language (leaked resources, inconsistent state). The safe pattern is cooperative cancellation: the running task periodically checks a cancellation flag and exits cleanly when set. This is the `CancellationToken` pattern from .NET, `context.Context` from Go, and `AbortController` from JavaScript. Rust's `std::sync::atomic::AtomicBool` provides the thread-safe flag needed for this pattern without heap allocation overhead.

## Learning Outcomes

- Implement a `CancellationToken` using `Arc<AtomicBool>` for shared cancellation state
- Use `Ordering::Release` and `Ordering::Acquire` for correct cross-thread visibility
- Integrate cancellation checks into long-running loops
- Clone tokens for multi-task cancellation (one cancel, many stop)
- Compare with OCaml's `Lwt_switch` and `Fiber.with_cancellation`

## Rust Application

`CancellationToken` holds `Arc<AtomicBool>`. `.cancel()` stores `true` with `Ordering::Release`. `.is_cancelled()` loads with `Ordering::Acquire`. `long_task` checks `token.is_cancelled()` at each iteration step, returning `Err(format!("cancelled at step {i}"))` when set. `cancellable_sum` checks every 1000 elements. The `Arc` clone enables the same token to be shared between the task and the canceller — one `.cancel()` stops all tasks holding a clone.

## OCaml Approach

OCaml's `Lwt_switch` provides cooperative cancellation for `Lwt` promises. `Lwt_switch.create ()` creates a switch; `Lwt_switch.add_hook switch f` registers cleanup; `Lwt_switch.turn_off switch` cancels. OCaml 5 `Eio` uses `Fiber.with_cancellation` and `Cancel.cancel`. For plain threads: `let cancelled = ref false` with a `Mutex` for thread safety, equivalent to Rust's `AtomicBool`. The Go-style `context.Context` has no direct OCaml equivalent in the standard library.

## Key Differences

1. **Atomics vs Mutex**: Rust uses `AtomicBool` for lock-free cancellation checking; OCaml needs `ref + Mutex` for equivalent thread safety.
2. **Memory ordering**: Rust's `Release`/`Acquire` semantics are explicit and precise; OCaml's `Mutex` provides full mutual exclusion (stronger but more costly).
3. **Cloneability**: Rust tokens clone cheaply via `Arc::clone` (atomic reference count); OCaml's `Mutex`-wrapped bool requires similar `Arc` / `ref` wrapping.
4. **Structured cancellation**: OCaml's `Lwt_switch` integrates with the Lwt event loop; Rust's `AtomicBool` is a low-level primitive — `tokio::CancellationToken` is the high-level version.

## Exercises

1. Implement a `timeout_token` that automatically sets itself after a specified `Duration`, combining `CancellationToken` with `thread::sleep`.
2. Add a `cancel_after(n: usize)` method to `CancellationToken` that sets itself after n calls to `is_cancelled()`.
3. Write a `CancellableIter<I: Iterator>` wrapper that checks the token on each `.next()` call.
