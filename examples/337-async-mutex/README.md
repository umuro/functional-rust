📖 **[View on hightechmind.io →](https://hightechmind.io/rust/337-async-mutex)**

---

# 337: Async Mutex
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Shared mutable state across concurrent threads or tasks requires mutual exclusion. `std::sync::Mutex<T>` provides this but blocks the OS thread when locked. In async contexts, blocking a thread blocks all tasks on that thread — a major performance problem. The correct pattern is to use `tokio::sync::Mutex` for async code (yields instead of blocks) and `std::sync::Mutex` only for brief critical sections that never span `.await` points.

## Learning Outcomes

- Use `Arc<Mutex<T>>` for shared mutable state across synchronous threads
- Understand why holding a `std::sync::Mutex` guard across `.await` is a deadlock risk
- Use `Arc<Mutex<T>>` with brief lock-and-release for async contexts
- Implement thread-safe caches and counters using `Mutex`

## Rust Application

Thread-safe shared counter using `Arc<Mutex<T>>`:

```rust
pub fn concurrent_increment(num_threads: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..num_threads).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || {
            *c.lock().unwrap() += 1;  // Lock briefly, unlock immediately
        })
    }).collect();
    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}
```

The `ThreadSafeCache<T>` pattern builds a write-once, read-many cache on top of `Mutex<HashMap>`.

## OCaml Approach

OCaml uses `Mutex.t` from the standard library for threading, and `Lwt_mutex.t` for async-aware locking:

```ocaml
let counter = ref 0
let mutex = Mutex.create ()

let increment () =
  Mutex.lock mutex;
  incr counter;
  Mutex.unlock mutex
```

OCaml 5's multi-core support uses `Mutex` from `Thread` + `Domain`.

## Key Differences

1. **Lock guard RAII**: Rust's `lock().unwrap()` returns a `MutexGuard` that unlocks on drop; OCaml requires explicit `Mutex.unlock()`.
2. **Poisoning**: Rust mutexes are "poisoned" if a thread panics while holding the lock — `lock()` returns `Err` thereafter; OCaml has no poisoning concept.
3. **Async mutex**: `tokio::sync::Mutex` is async-aware — `lock().await` yields instead of blocking; `std::sync::Mutex` should not span `.await` points.
4. **RwLock alternative**: For read-heavy workloads, `RwLock<T>` allows multiple concurrent readers and one exclusive writer.

## Exercises

1. Implement a thread-safe LRU cache using `Arc<Mutex<LruCache<K, V>>>`.
2. Show the deadlock risk of holding a `MutexGuard` across `.await` — demonstrate the issue and the fix.
3. Benchmark `Arc<Mutex<T>>` vs `Arc<RwLock<T>>` for a read-heavy workload with 10 readers and 1 writer.
