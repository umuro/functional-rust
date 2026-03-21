📖 **[View on hightechmind.io →](https://hightechmind.io/rust/444-arc-rwlock-pattern)**

---

# 444: `Arc<RwLock<T>>` — Multiple Readers, One Writer
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`Arc<Mutex<T>>` allows only one thread at a time — even for reads. For read-heavy workloads where many threads query shared state but writes are rare (configuration, routing tables, cached data), this is unnecessary serialization. `RwLock<T>` differentiates: any number of threads can hold a `read()` lock simultaneously, but a `write()` lock requires exclusive access. Combined with `Arc`, this enables high-concurrency reads with safe infrequent writes — the standard pattern for shared caches and configuration.

`Arc<RwLock<T>>` patterns appear in database connection pools, HTTP router tables, in-memory caches, feature flag systems, and any pattern with 95%+ read traffic.

## Learning Outcomes

- Understand the `RwLock` contract: N concurrent readers OR 1 exclusive writer
- Learn how `data.read().unwrap()` acquires a shared read guard
- See how `data.write().unwrap()` acquires an exclusive write guard
- Understand when `RwLock` outperforms `Mutex` (read-heavy) and when it doesn't (write-heavy)
- Learn the writer starvation risk: frequent readers can indefinitely delay writers on some platforms

## Rust Application

In `src/lib.rs`, `SharedConfig` wraps `Arc<RwLock<HashMap<String, String>>>`. The `get` method calls `.read().unwrap()` to acquire a shared read guard — multiple threads can call this concurrently. The `set` method calls `.write().unwrap()` for exclusive access. `clone_handle()` returns the `Arc` clone for sharing. The test demonstrates concurrent reads happening simultaneously while writes are serialized.

## OCaml Approach

OCaml 5.x uses `Rwlock.t` (a readers-writer lock) from the `Thread` module. OCaml 4.x's GIL makes `Rwlock` unnecessary since threads can't run in parallel anyway. The `Core_kernel` library provides `Readers_writer_lock` with similar semantics. OCaml doesn't enforce the read/write discipline through types — you can modify data through a read lock if the reference is mutable.

## Key Differences

1. **Type enforcement**: Rust's `RwLock::read()` returns `RwLockReadGuard` which provides only immutable access; OCaml's rwlock doesn't prevent mutation through read locks.
2. **Poisoning**: Like `Mutex`, Rust's `RwLock` poisons on writer panic; OCaml has no poisoning.
3. **Write starvation**: Rust's `std::sync::RwLock` is platform-dependent and may starve writers; `parking_lot::RwLock` from the parking_lot crate provides fairer scheduling.
4. **Performance**: `RwLock` has higher overhead than `Mutex` per operation; gains only appear when concurrent reads significantly outnumber writes.

## Exercises

1. **Metrics registry**: Build a `MetricsRegistry` using `Arc<RwLock<HashMap<String, f64>>>` where `record(name, value)` updates a metric and `snapshot() -> HashMap<String, f64>` returns a clone of all metrics. Spawn 16 reader threads and 2 writer threads, verifying no data races.
2. **Cache with invalidation**: Implement a `Cache<K, V>` wrapping `Arc<RwLock<HashMap<K, V>>>` with `get`, `set`, and `invalidate_all` methods. Show that `invalidate_all` temporarily blocks readers while it clears, and readers can proceed in parallel after.
3. **Read-through cache**: Build a cache that on miss acquires a write lock, checks again (to handle thundering herd), then computes and stores the value. Test with many concurrent misses to the same key, verifying the computation runs exactly once.
