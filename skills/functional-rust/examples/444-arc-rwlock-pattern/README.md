# 444: Arc<RwLock<T>> — Multiple Readers, One Writer

**Difficulty:** 3  **Level:** Intermediate

Allow many threads to read shared data simultaneously, while guaranteeing exclusive access for writes — better throughput than `Mutex` for read-heavy workloads.

## The Problem This Solves

`Mutex` is correct but conservative: one thread at a time, period. If your data is read 100 times for every write — a configuration map, a DNS cache, a lookup table — `Mutex` serialises all those reads unnecessarily. Four threads trying to read the same immutable snapshot of data block each other for no reason.

`RwLock` captures the natural distinction: reading doesn't mutate, so reads don't need to exclude each other. Multiple readers holding `RwLockReadGuard` simultaneously is safe; they all see a consistent snapshot. Only writes require exclusivity — `write()` blocks until all current readers finish, and new readers block until the writer releases.

The failure mode of getting this wrong in other languages is subtle. A `HashMap` read in Java while another thread writes it causes `ConcurrentModificationException` — at runtime. In Python, the GIL happens to protect you for pure CPython, but that disappears with extensions or alternative runtimes. Rust gives you the `RwLock<T>` invariant at compile time: you cannot call `write()` and hold a `ReadGuard` at the same time in the same thread, because that would be a deadlock the borrow checker catches.

## The Intuition

Think of `RwLock` as a library book: many people can read it at once, but if someone wants to write in the margins, everyone else has to put their copy down first. `Mutex` is the same book but with a rule that only one person can look at it at a time even to read.

In Java you'd use `ReentrantReadWriteLock`. In Go, `sync.RWMutex`. The Rust version wraps the data directly — the same "data inside the lock" guarantee as `Mutex`. You get `RwLockReadGuard` (shared, like `&T`) or `RwLockWriteGuard` (exclusive, like `&mut T`). The guard types make the access pattern visible in code.

## How It Works in Rust

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::thread;

let cfg: Arc<RwLock<HashMap<&str, &str>>> =
    Arc::new(RwLock::new(HashMap::from([("host", "localhost")])));

// Spawn 4 readers — all run concurrently, no blocking between them
let readers: Vec<_> = (0..4).map(|id| {
    let c = Arc::clone(&cfg);
    thread::spawn(move || {
        let guard = c.read().unwrap(); // shared — many readers OK at once
        let _ = guard.get("host");
        // guard drops here — read lock released
        println!("reader {} done", id);
    })
}).collect();

// Writer runs concurrently — blocks until all readers finish
let writer = {
    let c = Arc::clone(&cfg);
    thread::spawn(move || {
        let mut guard = c.write().unwrap(); // exclusive — waits for readers
        guard.insert("host", "example.com");
        // guard drops — write lock released, pending readers unblock
    })
};

for r in readers { r.join().unwrap(); }
writer.join().unwrap();
```

Prefer `RwLock` only when reads genuinely dominate. On Linux (pthreads), `RwLock` has slightly higher overhead than `Mutex` per operation. The win comes only when concurrent reads happen frequently enough to offset that cost.

## What This Unlocks

- **Shared configuration** — many threads read application config while an infrequent reload updates it.
- **In-process caches** — multiple request-handling threads read cache entries; a background thread writes new entries or invalidates stale ones.
- **Read-heavy lookup tables** — static data loaded once and queried thousands of times per second by concurrent workers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Multiple readers | blocked by any lock | simultaneous — all hold `RwLockReadGuard` |
| Exclusive write | same as Mutex | `write()` waits for all readers to release |
| Guard types | one Mutex guard | `RwLockReadGuard` / `RwLockWriteGuard` |
| Writer starvation | possible | possible on some platforms — OS-dependent |
| When to use | N/A | reads >> writes; otherwise prefer `Mutex` |
