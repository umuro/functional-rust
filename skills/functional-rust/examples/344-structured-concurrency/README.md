# 344: Structured Concurrency

**Difficulty:** 4  **Level:** Expert

All spawned tasks are guaranteed to complete before the scope exits — no task leaks, no dangling work.

## The Problem This Solves

Traditional thread spawning with `thread::spawn` produces a "detached" thread — it runs independently, with no guarantee about when it finishes. If the spawning code exits or panics before joining the thread, you have a race condition. Forgetting to `join()` a thread means it may still be running after the function that created it returns. With many spawned tasks, tracking all the handles becomes error-prone.

Structured concurrency is a different model: spawned tasks are scoped to a lexical block. When the block exits, *all* tasks in that scope are guaranteed to have completed. This makes concurrent code as easy to reason about as sequential code — you know exactly what's running at any point, and you can borrow from the enclosing scope safely (the borrow checker verifies this).

`thread::scope` is Rust's built-in structured concurrency primitive since Rust 1.63. In async Rust, `tokio::task::JoinSet` and the `FuturesUnordered` stream provide similar guarantees.

## The Intuition

Like Python's Trio nursery:
```python
async with trio.open_nursery() as nursery:
    nursery.start_soon(task_a)
    nursery.start_soon(task_b)
# Both tasks are done here — guaranteed
```

Or Java's structured concurrency (JEP 428):
```java
try (var scope = new StructuredTaskScope.ShutdownOnFailure()) {
    Future<A> fa = scope.fork(taskA);
    Future<B> fb = scope.fork(taskB);
    scope.join();
}
```

`thread::scope` in Rust gives the same guarantee, enforced at compile time through lifetimes.

## How It Works in Rust

```rust
let results: Mutex<Vec<String>> = Mutex::new(Vec::new());

thread::scope(|s| {
    // Spawn threads that borrow from the outer scope
    s.spawn(|| {
        results.lock().unwrap().push("task-A".to_string());
    });
    s.spawn(|| {
        results.lock().unwrap().push("task-B".to_string());
    });
    // All threads are joined HERE — the scope blocks until all finish
});

// Safe to access results — all threads are definitely done
let mut r = results.lock().unwrap();
r.sort();
```

The key insight: threads spawned inside `thread::scope` can borrow from the enclosing stack frame because the scope guarantees they finish before the enclosing frame exits. The borrow checker verifies this — something impossible with `thread::spawn` which requires `'static` bounds.

Nested scopes work too: outer scope waits for inner scopes, creating a task hierarchy with clear parent-child relationships.

## What This Unlocks

- **Safe parallel algorithms** — divide work into threads that share borrowed data, impossible with `thread::spawn`.
- **Fan-out/fan-in** — spawn N worker threads, collect all results, then proceed — with guaranteed cleanup even on panic.
- **Zero-leak concurrent code** — no forgotten join handles, no background threads running after the function returns.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Structured task scope | `Lwt.join` / manual tracking | `thread::scope` (1.63+) / `tokio::task::JoinSet` |
| Lifetime guarantee | Convention only | Enforced by borrow checker |
| Borrow from parent | Must clone or `Arc`-wrap | Direct borrow — scope guarantees lifetime |
| Nursery equivalent | N/A in stdlib | `thread::scope` for threads, `JoinSet` for tasks |
