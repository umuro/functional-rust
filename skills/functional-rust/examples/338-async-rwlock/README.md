# 338: Async RwLock

**Difficulty:** 3  **Level:** Advanced

Multiple concurrent readers, one exclusive writer — the right lock for read-heavy shared state.

## The Problem This Solves

A plain `Mutex` allows only one accessor at a time — even if multiple threads just want to *read* the same data simultaneously. For read-heavy workloads (caches, configuration, shared databases), this creates unnecessary contention. Every read blocks every other read, even though reads are perfectly safe to execute concurrently.

`RwLock` (Read-Write Lock) solves this: any number of readers can hold the lock simultaneously, but a writer gets exclusive access — no readers or other writers. This is the right primitive for data that's read often and written rarely: in-memory caches, configuration state, route tables, or — as in this example — a shared key-value store.

In async Rust, the same cross-await problem applies as with `Mutex`. `tokio::sync::RwLock` provides `read().await` and `write().await` that suspend the task, not the thread.

## The Intuition

Think of a library's book catalog: many people can browse it simultaneously (concurrent reads), but when the librarian updates it (write), everyone has to wait. That's an RwLock.

Python's `asyncio` has `asyncio.Lock` but no built-in `RwLock`. JavaScript is single-threaded and doesn't need it. In Rust it's a first-class primitive because the ownership system makes the reader/writer distinction valuable enough to enforce at the type level.

## How It Works in Rust

```rust
struct SharedDb { data: RwLock<HashMap<String, i32>> }

impl SharedDb {
    // Any number of threads can read() simultaneously
    fn read(&self, k: &str) -> Option<i32> {
        self.data.read().unwrap().get(k).copied()
    }

    // write() blocks until all readers are done, then gives exclusive access
    fn write(&self, k: &str, v: i32) {
        self.data.write().unwrap().insert(k.to_string(), v);
    }

    // update() also takes a write lock — read + modify in one lock acquisition
    fn update(&self, k: &str, f: impl Fn(i32) -> i32) {
        if let Some(v) = self.data.write().unwrap().get_mut(k) {
            *v = f(*v);
        }
    }
}
```

Concurrent reads don't block each other:
```rust
// All 5 threads read simultaneously — no contention
let handles: Vec<_> = (0..5).map(|_| {
    let db = Arc::clone(&db);
    thread::spawn(move || db.read("x"))
}).collect();
```

For async: `tokio::sync::RwLock` — `db.data.read().await` yields to other tasks while waiting.

## What This Unlocks

- **Shared caches** — populate once on startup, read from many async tasks without locking overhead.
- **Configuration reload** — writer acquires exclusive lock to reload config, readers continue unblocked afterward.
- **Route tables / DNS caches** — high read frequency, occasional updates — perfect RwLock fit.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| RwLock | No stdlib RwLock; use Mutex or custom | `std::sync::RwLock` in stdlib |
| Read guard | N/A | `RwLockReadGuard` — many can coexist |
| Write guard | N/A | `RwLockWriteGuard` — exclusive |
| Async version | `Lwt_rwlock` (community crate) | `tokio::sync::RwLock` |
