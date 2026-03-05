📖 **[View on hightechmind.io →](https://hightechmind.io/rust/325-select-futures)**

---

# 325: Racing Futures with select!

**Difficulty:** 3  **Level:** Advanced

`select!` races multiple futures and returns the first one to finish — the others are cancelled (dropped).

## The Problem This Solves

You need to fetch data from a slow external API, but you can't let the user wait forever. Without `select!`, you either block forever or implement complex cancellation logic with shared flags, mutexes, and condition variables.

`select!` also solves the "try multiple sources" problem: hit your primary cache and a fallback simultaneously, return whichever responds first.

## The Intuition

`select!` is like JavaScript's `Promise.race()` or Python's `asyncio.wait(return_when=FIRST_COMPLETED)` — whoever finishes first wins, the rest are abandoned.

```
join!:    task1 ──────────────────┐
          task2 ──────────┐       │  → waits for BOTH

select!:  task1 ──────────────────
          task2 ──────────┐  ← WINNER (first done)
                          → returns immediately, drops others
```

## How It Works in Rust

```rust
fn race<T: Send + 'static>(
    tasks: Vec<(Box<dyn FnOnce()->T+Send>, &'static str)>
) -> (&'static str, T) {
    let (tx, rx) = mpsc::channel();

    for (f, label) in tasks {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = tx.send((label, f()));  // first to finish sends
        });
    }

    rx.recv().unwrap()  // returns the first message
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Race futures | no stdlib equivalent | `select!` macro (tokio/futures) |
| First-wins semantics | `Lwt.pick [p1; p2]` | `select!` — first branch wins |
| Cancellation | exception propagation | losers are `drop`ped |
| Timeout | `Lwt_unix.with_timeout` | `time::timeout(dur, future)` |
