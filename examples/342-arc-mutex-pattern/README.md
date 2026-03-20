📖 **[View on hightechmind.io →](https://hightechmind.io/rust/342-arc-mutex-pattern)**

---

# 342: Arc<Mutex<T>> Pattern
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Multiple threads need to read and modify the same data safely. `Arc<Mutex<T>>` is Rust's canonical solution: `Arc` provides reference-counted shared ownership across threads, while `Mutex` ensures only one thread accesses the inner value at a time. This pattern implements the classic mutual exclusion concept formalized by Dijkstra (1965) — a lock that serializes access to a critical section. Without it, concurrent writes produce undefined behavior; with it, Rust's type system *statically* prevents data races, something C++ and Go can only detect at runtime with race detectors.

## Learning Outcomes

- Combine `Arc::new(Mutex::new(value))` for thread-safe shared state
- Clone the `Arc` with `Arc::clone(&arc)` to share ownership across threads
- Acquire a lock guard with `mutex.lock().unwrap()` and dereference to access data
- Understand that the lock guard releases automatically when it goes out of scope (RAII)
- Build thread-safe structs that wrap `Arc<Mutex<T>>` for ergonomic APIs
- Recognize deadlock risks when holding multiple locks simultaneously

## Rust Application

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn shared_counter(num_threads: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..num_threads).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || {
            *c.lock().unwrap() += 1;
        })
    }).collect();
    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}

pub struct ThreadSafeCache<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T: Clone> ThreadSafeCache<T> {
    pub fn new() -> Self {
        Self { data: Arc::new(Mutex::new(Vec::new())) }
    }
    pub fn push(&self, item: T) {
        self.data.lock().unwrap().push(item);
    }
    pub fn get_all(&self) -> Vec<T> {
        self.data.lock().unwrap().clone()
    }
}
```

`MutexGuard<T>` implements `Deref`/`DerefMut`, so `*guard += 1` works naturally. The guard's `Drop` impl releases the lock. If a thread panics while holding the lock, the mutex becomes "poisoned" — subsequent `lock()` calls return `Err`, preventing access to potentially inconsistent state.

## OCaml Approach

OCaml 5 uses domains for parallelism with `Mutex` from the standard library:

```ocaml
let m = Mutex.create () in
let counter = ref 0 in
let inc () =
  Mutex.lock m;
  incr counter;
  Mutex.unlock m
in
(* or safer: *)
let with_lock m f =
  Mutex.lock m;
  let result = f () in
  Mutex.unlock m; result
```

OCaml's garbage collector handles the shared reference counting automatically — no `Arc` needed, since the GC tracks liveness. In OCaml 4, threads share the GIL (Global Interpreter Lock), making `Mutex` less critical for pure OCaml data.

## Key Differences

| Aspect | Rust `Arc<Mutex<T>>` | OCaml `ref` + `Mutex` |
|--------|---------------------|----------------------|
| Ownership tracking | Compile-time via `Arc` reference count | GC tracks all references |
| Lock acquisition | `.lock()` returns `Result<Guard>` | `Mutex.lock` may raise |
| Poisoning on panic | Yes — subsequent locks get `Err` | No — state may be corrupt |
| Deadlock detection | None — avoid by design | None |
| `RwLock` variant | `Arc<RwLock<T>>` for read-many | `RwLock` in `Thread` module |

## Exercises

1. **`RwLock` comparison**: Replace `Mutex` with `RwLock` in `ThreadSafeCache` so multiple readers can access simultaneously; benchmark read throughput with 8 concurrent readers.
2. **Deadlock scenario**: Write two threads each holding one of two mutexes and waiting for the other; observe the deadlock, then fix it by enforcing a consistent lock acquisition order.
3. **Mutex-free counter**: Implement the shared counter using `AtomicI32` instead of `Mutex<i32>`; compare performance with the mutex version under contention.
