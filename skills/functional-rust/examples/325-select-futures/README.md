# 325: Racing Futures with select!

**Difficulty:** 3  **Level:** Advanced

`select!` races multiple futures and returns the first one to finish — the others are cancelled (dropped).

## The Problem This Solves

You need to fetch data from a slow external API, but you can't let the user wait forever. Without `select!`, you either block forever or implement complex cancellation logic with shared flags, mutexes, and condition variables. Getting that right is surprisingly hard.

`select!` also solves the "try multiple sources" problem: hit your primary cache and a fallback simultaneously, return whichever responds first. Or implement circuit breaking — if a database query takes longer than 100ms, bail out and serve a cached response rather than holding a connection open.

This is fundamentally different from `join!` — `join!` requires all tasks to succeed; `select!` races them and discards losers. The cancelled futures are simply dropped, which in Rust means their destructors run and resources are cleaned up safely.

## The Intuition

`select!` is like JavaScript's `Promise.race()` or Python's `asyncio.wait(return_when=FIRST_COMPLETED)` — whoever finishes first wins, the rest are abandoned.

```
join!:    task1 ──────────────────┐
          task2 ──────────┐       │  → waits for BOTH
          task3 ─────┘   ↑ (waits for slowest)

select!:  task1 ──────────────────
          task2 ──────────┐  ← WINNER (first done)
          task3 ─────────────      → returns immediately, drops others
```

The "non-determinism" warning you'll see in tokio's `select!` docs: if multiple futures complete in the same poll cycle, one is chosen arbitrarily. Don't rely on ordering — `select!` is for "I need any one result", not "I need the results in this order."

This example uses `mpsc::channel` + `recv_timeout` as the synchronous analogy: threads race to send on a channel, first message wins.

## How It Works in Rust

```rust
fn race<T: Send + 'static>(
    tasks: Vec<(Box<dyn FnOnce()->T+Send>, &'static str)>
) -> (&'static str, T) {
    let (tx, rx) = mpsc::channel();

    for (f, label) in tasks {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = tx.send((label, f()));  // first to finish sends its result
        });
    }

    rx.recv().unwrap()  // returns the first message — others may still be running
}

fn with_timeout<T: Send + 'static>(f: Box<dyn FnOnce()->T+Send>, ms: u64) -> Option<T> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    rx.recv_timeout(Duration::from_millis(ms)).ok()  // None if timeout fires first
}
```

Note `let _ = tx.send(...)` — we ignore the error because if the receiver was dropped (timeout fired), the losers will just fail to send, which is fine. No panic, no resource leak.

## What This Unlocks

- **Timeouts**: Wrap any async operation with a deadline — return an error if it doesn't complete in time.
- **Fallback sources**: Race primary and secondary data sources, use whichever responds first.
- **Cancellation**: `select!` with a cancellation channel lets you cleanly abort long-running tasks on demand.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Race futures | no stdlib equivalent | `select!` macro (tokio/futures) |
| First-wins semantics | `Lwt.pick [p1; p2]` | `select!` — first branch wins |
| Cancellation | exception propagation | losers are `drop`ped (destructors run) |
| Timeout | `Lwt_unix.with_timeout` | `time::timeout(dur, future)` |
