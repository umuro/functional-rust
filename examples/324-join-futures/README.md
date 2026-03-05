# 324: Running Futures Concurrently with join!

**Difficulty:** 3  **Level:** Advanced

`join!` runs multiple futures at the same time and waits for *all* of them — total time is `max(individual)`, not their sum.

## The Problem This Solves

You have three database queries to run, each taking 50ms. Done sequentially: 150ms. Done with `join!`: 50ms. That's a 3× speedup for free — no architecture changes, no callback pyramids, just telling Rust "these can run at the same time."

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

The two-phase pattern (spawn all, then collect) is important. If you did `spawn(f).join()` in a single loop, you'd be sequential again.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Run all concurrently | `Lwt.join_all [p1; p2; p3]` | `join!(f1, f2, f3)` or `futures::join_all` |
| Collect results | `list of 'a Lwt.t` → `'a list Lwt.t` | tuple from `join!` or `Vec<T>` from `join_all` |
| Time complexity | max of all | max of all |
