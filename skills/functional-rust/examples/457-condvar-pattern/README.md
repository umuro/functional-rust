# 457: Condvar — Sleep Until a Condition Is True

**Difficulty:** 3  **Level:** Intermediate

Use `Condvar` to park a thread until another thread signals a condition change — the efficient alternative to spinning on a flag.

## The Problem This Solves

A thread that needs to wait for something to happen has two options: spin (repeatedly check a flag in a loop) or sleep (park the thread and wake it when the condition changes). Spinning wastes CPU cycles — the thread burns 100% of a core doing nothing useful. With atomics, the spin is at least not blocking other threads, but you're still burning power and core time that another process could use.

`Condvar` solves this: the waiting thread atomically releases its mutex and parks. The OS removes it from the scheduler — it uses no CPU. When the condition changes, another thread calls `notify_one()` or `notify_all()`, and the OS wakes the parked thread. This is the foundation of producer-consumer queues, event loops, and any system where work arrives asynchronously.

A critical subtlety: **spurious wakeups**. An OS is allowed to wake a thread waiting on a condvar even without a `notify`. Any waiting code must re-check the condition in a loop after waking. `Condvar::wait_while(guard, predicate)` handles this automatically — it re-acquires the lock and re-checks the predicate before returning.

## The Intuition

A `Condvar` always pairs with a `Mutex`. The mutex protects the condition state (the `bool`, the `Vec`, whatever you're waiting for). The condvar provides the sleep/wake mechanism. Together they're `Arc<(Mutex<State>, Condvar)>`.

In Java: `Object.wait()` / `Object.notifyAll()` — but Java's version has footguns (you can call `wait()` without holding the monitor). In Python: `threading.Condition`. In Go: `sync.Cond`. Rust's version is safer: `wait()` takes a `MutexGuard` and returns one, making it impossible to call without holding the lock.

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

// Condvar always pairs with a Mutex — bundle them together
let pair = Arc::new((Mutex::new(Vec::<u32>::new()), Condvar::new()));

// Producer: push values, notify after each
let p = Arc::clone(&pair);
let producer = thread::spawn(move || {
    let (lock, cvar) = &*p;
    for i in 1..=5 {
        lock.lock().unwrap().push(i);  // modify state
        cvar.notify_one();              // wake one waiting consumer
    }
});

// Consumer: wait while queue is empty, consume one at a time
let c = Arc::clone(&pair);
let consumer = thread::spawn(move || {
    let (lock, cvar) = &*c;
    for _ in 0..5 {
        // wait_while: atomically release lock and sleep
        // wakes when notified AND predicate returns false
        let mut guard = cvar.wait_while(
            lock.lock().unwrap(),
            |v| v.is_empty()  // sleep while empty
        ).unwrap();
        let v = guard.remove(0);
        println!("consumed {}", v);
    }
});

producer.join().unwrap();
consumer.join().unwrap();

// notify_all: wake all waiting threads simultaneously (e.g., start signal)
let barrier = Arc::new((Mutex::new(false), Condvar::new()));
let hs: Vec<_> = (0..3).map(|id| {
    let b = Arc::clone(&barrier);
    thread::spawn(move || {
        let _g = b.1.wait_while(b.0.lock().unwrap(), |&mut ready| !ready).unwrap();
        println!("worker {} unblocked", id);
    })
}).collect();
*barrier.0.lock().unwrap() = true;
barrier.1.notify_all(); // wake all three workers at once
for h in hs { h.join().unwrap(); }
```

Always use `wait_while` (or check the condition in a loop after `wait`) — never assume a wakeup means the condition is true.

## What This Unlocks

- **Producer-consumer queues** — consumers sleep when the queue is empty; producers push and `notify_one()` to wake a consumer.
- **Thread barriers** — all threads wait until a "go" signal; main thread sets the flag and calls `notify_all()`.
- **Rate-limited workers** — worker waits for a token or permission to proceed; controller grants and notifies.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Condvar creation | `Condition.create ()` | `Condvar::new()` |
| Wait | `Condition.wait cond mutex` | `cvar.wait_while(guard, \|s\| predicate(s))` |
| Lock required | yes — same as Rust | yes — `MutexGuard` must be held |
| Spurious wakeup | must check in loop | `wait_while` handles automatically |
| Signal one | `Condition.signal` | `cvar.notify_one()` |
| Broadcast | `Condition.broadcast` | `cvar.notify_all()` |
| Safety | manual discipline | `wait` takes and returns `MutexGuard` — impossible to skip lock |
