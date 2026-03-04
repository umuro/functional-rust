# 321: async fn and .await Fundamentals

**Difficulty:** 3  **Level:** Advanced

Async functions return a Future — a description of work, not work itself. Nothing runs until you drive it.

## The Problem This Solves

You're building a web service that needs to fetch a user record and their posts from a database. Done sequentially, each operation blocks until the previous one finishes — if each takes 50ms, you wait 100ms total. With dozens of concurrent requests, you're burning threads that mostly just sleep.

The real problem is that threads are expensive. Each OS thread uses ~1–8 MB of stack memory and has significant scheduling overhead. When your work is I/O-bound (waiting on network, disk, database), most of that thread is idle. You need a way to say "start this work, come back when it's done" without dedicating an entire thread to waiting.

Without async, you either block (wasting threads) or use callbacks (callback hell, inverted control flow). Async gives you linear-looking code that doesn't block.

## The Intuition

Think of `async fn` like a JavaScript `async function` or Python `async def` — it looks like normal code but runs cooperatively. The key insight: **an async function doesn't run when called. It returns a Future, which is just a description of what to do.** You need to `.await` it (or give it to a runtime) for anything to happen.

```
Normal fn:   call → runs immediately → returns value
Async fn:    call → returns Future (nothing runs yet) → .await → runs → returns value
```

In JavaScript: `fetch(url)` returns a Promise. Until you `await` it, nothing happens. Same idea in Rust, but stricter — the compiler enforces it.

This example uses `std::thread` as a synchronous analogy (no tokio needed), showing the same sequential vs. concurrent patterns you'd use with `.await` and `join!`.

## How It Works in Rust

```rust
// Sequential — like: let user = fetch_user(id).await; let posts = fetch_posts(id).await;
fn sequential_fetch(id: u32) -> (String, Vec<String>) {
    (fetch_user(id), fetch_posts(id))   // each call blocks until done
}

// Concurrent — like: join!(fetch_user(id), fetch_posts(id))
fn concurrent_fetch(id: u32) -> (String, Vec<String>) {
    let h1 = thread::spawn(move || fetch_user(id));    // start both
    let h2 = thread::spawn(move || fetch_posts(id));   // before waiting for either
    (h1.join().unwrap(), h2.join().unwrap())           // now wait for both
}
```

The `move` keyword transfers ownership of `id` into the closure — necessary because the thread might outlive the current scope. In real async code, `async move { ... }` does the same thing.

## What This Unlocks

- **I/O-bound services**: Serve thousands of concurrent requests with far fewer threads than traditional blocking I/O.
- **Parallel data fetching**: Fetch from multiple sources simultaneously instead of sequentially.
- **Responsive UIs**: Keep interfaces interactive while background work runs without blocking the main thread.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Async function | no native async (Lwt library) | `async fn foo() -> T` |
| Await result | `Lwt.bind p (fun x -> ...)` / `>>=` | `.await` |
| Run both concurrently | `Lwt.join [p1; p2]` | `join!(f1, f2)` (needs runtime) |
| Lazy evaluation | explicit thunks `fun () -> ...` | implicit — Future not polled until driven |
