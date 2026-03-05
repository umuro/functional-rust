# 337: Async Mutex

**Difficulty:** 3  **Level:** Advanced

Lock shared state safely across async tasks — `std::sync::Mutex` works but must not be held across `.await`.

## The Problem This Solves

Holding a `std::sync::Mutex` guard across an `.await` point can cause deadlocks or panics. The rule: **never hold a sync Mutex guard across an `.await`**. For that, use `tokio::sync::Mutex`.

## The Intuition

In JavaScript (single-threaded), no mutexes needed. In Python's asyncio, `asyncio.Lock` yields while waiting. Rust exposes the distinction explicitly: sync mutex blocks the thread, async mutex suspends the task.

## How It Works in Rust

```rust
// CORRECT: release the lock before other work
fn correct_pattern() {
    let shared = Arc::new(Mutex::new(vec![1, 2, 3]));

    // Braces create a scope — guard drops when scope exits
    let sum = { shared.lock().unwrap().iter().sum::<i32>() };
    //        ^-- guard drops here, BEFORE any .await

    // WRONG: let guard = shared.lock().unwrap(); async_fn().await;
}

// Poison recovery if a thread panicked while holding the lock
match m.lock() {
    Ok(v) => println!("Ok: {v}"),
    Err(p) => println!("Recovered: {}", p.into_inner()),
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sync mutex | `Mutex.lock` | `std::sync::Mutex` |
| Async mutex | `Lwt_mutex.lock` | `tokio::sync::Mutex` |
| Poison | Doesn't exist | `PoisonError` |
| Guard scope | Explicit unlock | RAII drop |
