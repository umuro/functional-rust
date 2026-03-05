# 338: Async RwLock

**Difficulty:** 3  **Level:** Advanced

Multiple concurrent readers, one exclusive writer — the right lock for read-heavy shared state.

## The Problem This Solves

A plain `Mutex` allows only one accessor at a time, even for reads. `RwLock` allows any number of concurrent readers, but writers get exclusive access. Perfect for read-heavy caches and configuration.

## The Intuition

Think of a library catalog: many people can browse it simultaneously (concurrent reads), but when the librarian updates it (write), everyone waits.

## How It Works in Rust

```rust
struct SharedDb { data: RwLock<HashMap<String, i32>> }

impl SharedDb {
    // Any number of threads can read() simultaneously
    fn read(&self, k: &str) -> Option<i32> {
        self.data.read().unwrap().get(k).copied()
    }

    // write() blocks until all readers are done
    fn write(&self, k: &str, v: i32) {
        self.data.write().unwrap().insert(k.to_string(), v);
    }
}
```

Concurrent reads don't block each other:
```rust
let handles: Vec<_> = (0..5).map(|_| {
    let db = Arc::clone(&db);
    thread::spawn(move || db.read("x"))
}).collect();
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| RwLock | No stdlib | `std::sync::RwLock` |
| Read guard | N/A | `RwLockReadGuard` |
| Write guard | N/A | `RwLockWriteGuard` |
| Async version | `Lwt_rwlock` | `tokio::sync::RwLock` |
