# 327: Spawning Concurrent Tasks

**Difficulty:** 3  **Level:** Advanced

Spawn a task to run it in the background independently — fire and forget, or collect the result later via a handle.

## The Problem This Solves

Some work should happen concurrently without blocking the current flow. Sending a welcome email when a user registers shouldn't make the registration response wait. Spawning tasks separates the "start this work" moment from the "wait for the result" moment.

The critical constraint: spawned tasks must be `'static`. They cannot borrow from the scope that spawned them — they might outlive it.

## The Intuition

`thread::spawn` is like `asyncio.create_task()` in Python or firing off a Promise without awaiting it — the work starts immediately in the background.

```rust
// Rust: spawn and optionally collect later
let handle = thread::spawn(move || do_something());
// ... continue with other work ...
let result = handle.join().unwrap();  // collect later if needed
```

## How It Works in Rust

```rust
fn spawn_worker(id: usize, delay_ms: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        format!("worker-{id} done after {delay_ms}ms")
    })
}

// Spawn all, then collect results
let handles: Vec<_> = (0..5)
    .map(|i| spawn_worker(i, (5-i) as u64 * 10))
    .collect();

for h in handles {
    println!("{}", h.join().unwrap());
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Spawn | `Thread.create f ()` | `thread::spawn(\|\| ...)` |
| Await result | `Thread.join t` | `handle.join().unwrap()` |
| Static requirement | N/A | `'static` + `Send` required |
