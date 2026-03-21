📖 **[View on hightechmind.io →](https://hightechmind.io/rust/345-async-drop)**

---

# 345: Async Drop
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Resource cleanup (closing files, flushing buffers, notifying peers) often requires async operations — but Rust's `Drop` trait is synchronous. If an async task holding a database connection is cancelled, its `Drop` runs synchronously on the async runtime, potentially blocking the executor thread. This mismatch is a known pain point: Rust doesn't yet have `AsyncDrop` in stable (RFC 3541 is in progress). The workaround is RAII guards with synchronous `Drop` that signal cleanup flags, deferring actual async cleanup to explicit `close()` methods or `defer!`-like patterns that run before the future is abandoned.

## Learning Outcomes

- Implement `Drop` to run cleanup logic when a value goes out of scope
- Use `Arc<AtomicBool>` as a cleanup witness to verify `Drop` ran
- Implement RAII guards with `disarm()` to skip cleanup on success paths
- Understand why `Drop` cannot be `async` and the implications for async code
- Use the guard pattern to ensure cleanup even on panics or early returns
- Recognize where explicit `close()` methods are necessary for async cleanup

## Rust Application

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Resource {
    id: usize,
    cleaned_up: Arc<AtomicBool>,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // synchronous cleanup — no .await allowed here
        self.cleaned_up.store(true, Ordering::SeqCst);
        println!("Resource {} cleaned up", self.id);
    }
}

pub struct Guard<F: FnOnce()> {
    cleanup: Option<F>,
}

impl<F: FnOnce()> Guard<F> {
    pub fn new(cleanup: F) -> Self {
        Self { cleanup: Some(cleanup) }
    }
    pub fn disarm(mut self) {
        self.cleanup = None; // don't run cleanup on success
    }
}

impl<F: FnOnce()> Drop for Guard<F> {
    fn drop(&mut self) {
        if let Some(f) = self.cleanup.take() {
            f(); // runs on panic or early return
        }
    }
}
```

The `Option<F>` trick lets `disarm()` prevent cleanup on the success path while `Drop` still guarantees cleanup runs if the guard is dropped without `disarm()`. This is identical to how `scopeguard::defer!` works internally.

## OCaml Approach

OCaml lacks RAII (no destructors). Cleanup is managed explicitly through `Fun.protect`:

```ocaml
let with_resource id f =
  let cleaned_up = ref false in
  Fun.protect
    ~finally:(fun () -> cleaned_up := true)
    (fun () -> f id)
```

`Fun.protect ~finally` guarantees `finally` runs even if `f` raises an exception — the functional equivalent of RAII. For Lwt async cleanup: `Lwt.finalize` runs a cleanup promise whether the main promise succeeds or fails.

## Key Differences

| Aspect | Rust `Drop` | OCaml `Fun.protect` |
|--------|-------------|---------------------|
| Trigger | Automatic when value leaves scope | Must explicitly wrap with `protect` |
| Async cleanup | Not supported in `Drop` | `Lwt.finalize` handles async |
| Panic safety | `Drop` runs even on panic (usually) | `finally` runs even on exception |
| Zero-cost | Yes — no runtime overhead | Minor overhead for exception handling |
| Guard pattern | `Option<F>` + `disarm()` | Return value or flag from `finally` |

## Exercises

1. **File flush guard**: Implement a `FlushGuard` that wraps a `BufWriter<File>` and calls `flush()` in `Drop`; verify that partial writes are flushed even if the function panics midway.
2. **Disarm test**: Write a test that creates a `Guard` with a counter, calls `disarm()`, lets it drop, and verifies the counter wasn't incremented; then write a complementary test without `disarm()`.
3. **Async cleanup workaround**: In a Tokio context, implement a `Resource` that has a `close(self) -> impl Future` method for async cleanup; wrap it in a sync `Drop` that logs a warning if `close()` was never called.
