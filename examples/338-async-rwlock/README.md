📖 **[View on hightechmind.io →](https://hightechmind.io/rust/338-async-rwlock)**

---

# 338: Async RwLock — Multiple Readers, One Writer

## Problem Statement

Many shared data structures are read frequently and written rarely — caches, configuration, routing tables. A `Mutex` serializes all access (both reads and writes), creating unnecessary contention. `RwLock<T>` allows multiple concurrent readers OR one exclusive writer, matching the actual access pattern. This is the correct concurrency primitive for read-heavy workloads in database caches, HTTP routers, and configuration stores.

## Learning Outcomes

- Understand `RwLock<T>` as allowing many concurrent readers OR one exclusive writer
- Use `read()` for shared access and `write()` for exclusive access
- Implement a `SharedDb` with concurrent read access and exclusive write
- Recognize when `RwLock` beats `Mutex`: read-heavy workloads with infrequent writes

## Rust Application

`RwLock<HashMap<String, i32>>` allows concurrent reads, exclusive writes:

```rust
pub struct SharedDb {
    data: RwLock<HashMap<String, i32>>,
}

impl SharedDb {
    pub fn read(&self, key: &str) -> Option<i32> {
        self.data.read().unwrap().get(key).copied()
        // Multiple threads can call read() simultaneously
    }

    pub fn write(&self, key: String, value: i32) {
        self.data.write().unwrap().insert(key, value);
        // Only one thread can call write() at a time
    }
}
```

## OCaml Approach

OCaml 5's `RWMutex` from `Thread` provides the same semantics. For Lwt, `Lwt_mutex.with_lock` serializes writes, and reads from immutable snapshots avoid locking:

```ocaml
let db = ref (Hashtbl.create 16)
let rwlock = RWMutex.create ()

let read key = RWMutex.read_lock rwlock;
  let v = Hashtbl.find_opt !db key in
  RWMutex.read_unlock rwlock; v
```

## Key Differences

1. **RAII guards**: Rust's `read().unwrap()` returns a `RwLockReadGuard` that drops automatically; OCaml requires explicit unlock calls.
2. **Writer starvation**: `std::sync::RwLock` may starve writers if there are always readers; `parking_lot::RwLock` provides fairer scheduling.
3. **Poisoning**: Like `Mutex`, Rust's `RwLock` is poisoned on writer panic; subsequent `read()` or `write()` returns `Err`.
4. **Async variant**: `tokio::sync::RwLock` is the async-aware version with `.read().await` and `.write().await`.

## Exercises

1. Benchmark `Arc<Mutex<T>>` vs `Arc<RwLock<T>>` with 8 reader threads and 1 writer thread — measure throughput.
2. Implement a configuration store that allows hot-reload: a background thread periodically acquires the write lock and updates the config.
3. Show a writer starvation scenario: many readers continuously holding read locks prevent a writer from acquiring the write lock.
