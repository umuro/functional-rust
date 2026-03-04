# 342: Arc<Mutex<T>> Pattern

**Difficulty:** 3  **Level:** Advanced

Thread-safe shared mutable state: `Arc` gives shared ownership across threads, `Mutex` ensures only one thread modifies at a time.

## The Problem This Solves

Sometimes you genuinely need multiple threads to share and modify the same piece of data — a counter, a cache, a connection pool. Channels work when you can model the problem as message passing, but not everything fits that mold. A shared hit counter updated by dozens of concurrent request handlers needs shared mutable state.

Rust's ownership model normally prevents this entirely: one owner, or many readers, but never multiple mutable references. `Arc<Mutex<T>>` is the escape hatch — explicitly opt into shared mutability with guaranteed safety. The compiler forces you to lock before accessing, making data races impossible. If you forget to lock, it won't compile.

The pattern appears everywhere in Rust code dealing with concurrency: web servers maintaining connection state, databases with connection pools, caches shared across request handlers.

## The Intuition

Two separate problems, two separate types:

**`Arc<T>` — shared ownership**: Multiple threads need to *hold* the same data. `Rc<T>` does this for single-threaded code, but isn't safe across threads. `Arc` (Atomic Reference Count) uses atomic operations for the reference count — thread-safe. When the last Arc is dropped, the allocation is freed.

**`Mutex<T>` — exclusive access**: Multiple threads need to *modify* the same data, but not simultaneously. `Mutex` is a lock: only one thread can hold it at a time. In Rust, the data lives *inside* the mutex — you can't access it without locking. This is different from many other languages where the mutex and the data are separate.

```python
# Python: lock and data are separate — easy to access data without locking
self.lock = threading.Lock()
self.data = []
# Oops: self.data.append(x) — forgot to lock, race condition

# Rust: data is inside the mutex — can't access without locking
let shared = Arc::new(Mutex::new(Vec::new()));
shared.lock().unwrap().push(x);  // lock() required, no way around it
```

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0i32));

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);  // clone the Arc (cheap: just ref count++)
        thread::spawn(move || {
            let mut val = counter.lock().unwrap();  // lock — blocks if another thread holds it
            *val += 1;
            // lock released here when `val` (MutexGuard) is dropped
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    println!("Final: {}", *counter.lock().unwrap());  // prints 10
}
```

`counter.lock().unwrap()` returns a `MutexGuard<T>` — a smart pointer that dereferences to `T` and releases the lock when dropped (RAII). The `unwrap()` handles the poisoned mutex case: if a thread panicked while holding the lock, future `lock()` calls return `Err`.

**Avoid holding the lock across awaits in async code** — that blocks the entire async thread. In async contexts, use `tokio::sync::Mutex` instead of `std::sync::Mutex`.

## What This Unlocks

- **Shared counters / metrics**: Concurrent request handlers incrementing the same counter safely.
- **Lazy initialization**: Compute an expensive value once, cache it in `Arc<Mutex<Option<T>>>`, subsequent calls get the cached value.
- **Shared connection pools**: Multiple threads borrowing from the same pool of database connections.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared ownership | `ref` / `Hashtbl` / manual | `Arc<T>` — atomic ref count |
| Mutual exclusion | `Mutex.create ()` (separate from data) | `Mutex<T>` (data inside the lock) |
| Lock acquisition | `Mutex.lock m` | `arc.lock().unwrap()` → `MutexGuard<T>` |
| Auto-unlock | explicit `Mutex.unlock m` | automatic on `MutexGuard` drop (RAII) |
| Shared mutable state | `ref` with manual synchronization | `Arc<Mutex<T>>` — compiler-enforced |
