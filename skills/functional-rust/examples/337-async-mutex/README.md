# 337: Async Mutex

**Difficulty:** 3  **Level:** Advanced

Lock shared state safely across async tasks — `std::sync::Mutex` panics when held across `.await`.

## The Problem This Solves

In synchronous code, `std::sync::Mutex` is the right tool for protecting shared mutable state across threads. But in async code, holding a `std::sync::Mutex` guard across an `.await` point causes a problem: the thread might switch to executing a different task while you hold the lock. If that other task also tries to lock the same mutex — deadlock. Worse, tokio's single-threaded executor panics immediately with "cannot lock a mutex on the current thread."

The rule is: **never hold a `std::sync::Mutex` guard across an `.await`**. If you need to hold a lock across an async operation, you need an async-aware mutex (like `tokio::sync::Mutex`) whose `lock().await` suspends the *task* rather than blocking the *thread*.

This example demonstrates the correct patterns: release the lock before awaiting, use `std::sync::Mutex` safely with short critical sections, and recover from mutex poisoning.

## The Intuition

Compare to JavaScript: there's no mutex in JS because it's single-threaded. But if Rust's async executor is also single-threaded (tokio's `current_thread` runtime), holding a sync lock across an await is a logic error — you've suspended the only thread while it holds the lock. Nothing can release it.

Python's `asyncio.Lock` is the async equivalent: `async with lock: await something()` — it yields the event loop while waiting to acquire, rather than blocking the thread.

## How It Works in Rust

```rust
// CORRECT: release the lock before awaiting
fn correct_pattern() {
    let shared = Arc::new(Mutex::new(vec![1i32, 2, 3]));

    // The braces create a scope — guard drops when scope exits
    let sum = { shared.lock().unwrap().iter().sum::<i32>() };
    //        ^-- guard drops here, BEFORE any .await                ^
    println!("Sum: {sum}");

    // WRONG (commented out):
    // let guard = shared.lock().unwrap();
    // some_async_fn().await;   // guard still held — DEADLOCK or PANIC
}

// Poison recovery: if a thread panics while holding the lock, it "poisons" it
match m.lock() {
    Ok(v)  => println!("Ok: {v}"),
    Err(p) => println!("Recovered: {}", p.into_inner()), // access data anyway
}
```

For cross-await locking in real async code, use `tokio::sync::Mutex`:
```rust
let mutex = Arc::new(tokio::sync::Mutex::new(0));
let mut guard = mutex.lock().await;  // suspends task, not thread
*guard += 1;
// guard can safely be held across other .await points here
```

## What This Unlocks

- **Safe counters and caches** — increment a shared counter or update a cache from multiple async tasks.
- **Connection pool management** — protect the list of available connections with a mutex across async acquire/release.
- **State machines** — guard state transitions in a long-running async service.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sync mutex | `Mutex.create()` / `Mutex.lock` (blocks thread) | `std::sync::Mutex` (blocks thread) |
| Async mutex | `Lwt_mutex.create()` / `Lwt_mutex.lock m >>= ...` | `tokio::sync::Mutex::lock().await` |
| Poison | Doesn't exist — no thread panics corrupt mutex | `PoisonError` if thread panics while locked |
| Guard scope | `Mutex.unlock m` (explicit) | RAII: guard drops at end of scope |
