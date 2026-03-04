# 345: Async Cleanup and Drop

**Difficulty:** 4  **Level:** Expert

Run cleanup code before a resource is released — but Rust's `Drop` is synchronous, so async shutdown needs an explicit contract.

## The Problem This Solves

Many resources — database connections, file handles, network sockets — need graceful shutdown: flush pending writes, send a `FIN`, release a lock. In an async system you'd `await` these teardown steps. But Rust's `Drop` trait is called synchronously by the destructor; you cannot `.await` inside it.

This gap creates a real hazard. If you rely on `Drop` for async cleanup, the cleanup silently becomes fire-and-forget at best, or panics at worst. Production codebases that ignore this lose connections, leave locks held, or corrupt state during shutdown.

The standard Rust solution is an explicit contract: callers must call `async fn shutdown()` (or `close()`) before dropping the value. The `Drop` implementation serves as a safety net — it detects the "dropped without shutdown" case and at minimum logs a warning.

## The Intuition

Think of a database session. Properly closing it sends a goodbye packet so the server reclaims the slot immediately. If the client just disconnects, the server waits for a TCP timeout. `Drop` is the TCP-level disconnect — it works, but it's ugly. `shutdown()` is the clean goodbye.

An RAII guard wraps the resource and calls `shutdown()` automatically in its own `Drop`, so callers who use the guard never have to remember.

## How It Works in Rust

1. **`AsyncConnection`** — wraps a resource plus an `Arc<AtomicBool>` marking whether it is open.
2. **`fn shutdown(&mut self)`** — sets the atomic to `false`, runs cleanup. In real async code this would be `async fn shutdown()`.
3. **`impl Drop`** — checks if still open; if so, logs a warning and force-closes. This is the fallback, not the happy path.
4. **`ConnectionGuard`** — a newtype wrapper whose `Drop` calls `shutdown()`. Callers get clean teardown for free.

```rust
impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.0.shutdown(); // always closes, even on panic
    }
}
```

The atomic flag ensures `shutdown()` is idempotent — calling it twice is safe.

## What This Unlocks

- **Async-safe resource teardown** — explicit `shutdown()` composes with `.await`; `Drop` cannot.
- **Leak detection** — the `Drop` fallback makes "dropped without cleanup" visible as a warning instead of a silent bug.
- **RAII guards** — scope-based cleanup works for sync shutdown; pair with `drop(guard)` when you need early exit.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Destructor hook | GC finalizer (non-deterministic) | `impl Drop` (deterministic, sync) |
| Async cleanup | GC defers, Lwt.finalize | Explicit `async fn shutdown()` before drop |
| RAII guard | No RAII idiom | `struct Guard; impl Drop { shutdown() }` |
| Forced cleanup | GC eventually calls finalizer | `Drop` runs when value goes out of scope |
