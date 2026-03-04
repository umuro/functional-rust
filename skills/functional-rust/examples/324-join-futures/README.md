# 324: Running Futures Concurrently with join!

**Difficulty:** 3  **Level:** Advanced

`join!` runs multiple futures at the same time and waits for *all* of them — total time is `max(individual)`, not their sum.

## The Problem This Solves

You have three database queries to run, each taking 50ms. Done sequentially: 150ms. Done with `join!`: 50ms. That's a 3× speedup for free — no architecture changes, no callback pyramids, just telling Rust "these can run at the same time."

The sequential approach isn't just slower — it's wrong for many use cases. If you're building an API response that aggregates data from multiple sources (user profile + recent orders + recommendations), every extra sequential call adds latency that compounds. With `join!`, they all start simultaneously and you get results when the *slowest* one finishes.

Without `join!`, the alternative is spawning threads (expensive) or writing manual synchronization with channels. `join!` gives you concurrency for independent work with zero boilerplate.

## The Intuition

`join!` is like Python's `asyncio.gather()` or JavaScript's `Promise.all()` — start everything, wait for everything.

```
Sequential:  [task1: 50ms] → [task2: 30ms] → [task3: 10ms] = 90ms total
join!:       [task1: 50ms]
             [task2: 30ms]  (all running simultaneously)
             [task3: 10ms]
             = 50ms total (the slowest one)
```

The key difference from `select!`: `join!` waits for ALL futures. `select!` returns when the FIRST one finishes. Use `join!` when you need all results; use `select!` when you need the fastest or want to cancel.

This example uses `thread::spawn` + `join()` as the synchronous analogy — spawning threads to run in parallel is the std equivalent of `join!`ing async futures on a runtime.

## How It Works in Rust

```rust
fn join_all<T: Send + 'static>(tasks: Vec<Box<dyn FnOnce()->T+Send>>) -> Vec<T> {
    // Phase 1: spawn everything (all start running now)
    let handles: Vec<_> = tasks.into_iter()
        .map(|f| thread::spawn(f))
        .collect();

    // Phase 2: collect results (wait for each to finish)
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}
```

The two-phase pattern (spawn all, then collect) is important. If you did `spawn(f).join()` in a single loop, you'd be sequential again — you'd wait for task 1 before starting task 2. Collecting all handles first ensures all tasks are running before any waiting begins.

`T: Send + 'static` — the result type must be sendable between threads (`Send`) and own its data (`'static`). This is the same constraint as async `spawn`.

## What This Unlocks

- **Parallel API calls**: Fetch from multiple services simultaneously — user service + order service + inventory service in one round trip.
- **Fan-out computation**: Split work into independent chunks, process in parallel, collect results.
- **Aggregate operations**: Build dashboard data combining multiple data sources, completing in the time of the slowest single source.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Run all concurrently | `Lwt.join_all [p1; p2; p3]` | `join!(f1, f2, f3)` or `futures::join_all` |
| Collect results | `list of 'a Lwt.t` → `'a list Lwt.t` | tuple from `join!` or `Vec<T>` from `join_all` |
| Error propagation | `Lwt.catch` | `?` on each result after join |
| Time complexity | max of all | max of all |
