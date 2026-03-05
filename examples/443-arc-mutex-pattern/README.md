📖 **[View on hightechmind.io →](https://hightechmind.io/rust/443-arc-mutex-pattern)**

---

# 443: Arc<Mutex<T>> — Shared Mutable State Across Threads

**Difficulty:** 3  **Level:** Intermediate

Share a single mutable value across multiple threads using `Arc` for ownership and `Mutex` for exclusive access — the compile-time enforced lock pattern.

## The Problem This Solves

Multiple threads writing to shared state is the source of most concurrency bugs. A counter incremented by 10 threads without coordination will silently produce the wrong total — because increment is three operations (read, add, write) and threads interleave arbitrarily. This is a data race: undefined behavior in C/C++, a runtime check failure in Java, an occasional wrong answer in Python (which only avoids the worst because of the GIL, at the cost of true parallelism).

In languages with locks you write the correct-looking code and hope you remembered to acquire the lock, hope you don't hold it across a function that also acquires it (deadlock), and hope you release it even in error paths. In Rust, the `Mutex<T>` wraps the data itself — you cannot touch the data without going through the lock. The type system makes this structural: the value is inside the mutex. There's no way to "accidentally forget to lock" because you can't reach the data without calling `.lock()`.

The `MutexGuard` that `.lock()` returns implements `Drop` — when it goes out of scope (including on panic, via unwinding), the lock is released. You cannot forget to unlock. The borrow checker ensures the guard's lifetime bounds all access to the inner value.

## The Intuition

`Arc` is "Atomically Reference Counted" — like `Rc` but thread-safe. Clone the `Arc` to share ownership across threads; the value is freed when the last clone drops. `Mutex` is the lock: only one thread can hold the `MutexGuard` at a time. Together, `Arc<Mutex<T>>` is Rust's canonical "shared mutable state" pattern.

In Python you'd write `lock = threading.Lock(); lock.acquire(); counter += 1; lock.release()` — data and lock are separate, bugs hide in the gap. In Java, `synchronized(obj)` locks on an arbitrary object. In Rust, the data IS inside the lock — there is no gap.

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0u64)); // data lives inside the Mutex

let handles: Vec<_> = (0..10).map(|_| {
    let c = Arc::clone(&counter); // clone the Arc — increments ref count
    thread::spawn(move || {
        for _ in 0..100 {
            // lock() blocks until we hold the lock, returns MutexGuard
            // *guard dereferences to &mut u64
            *c.lock().unwrap() += 1;
            // guard drops here — lock released automatically
        }
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap()); // 1000 — guaranteed correct
```

`.unwrap()` on `.lock()` handles "poisoned" mutexes — a mutex is poisoned if a thread panics while holding it. In most cases, propagating the panic is correct; production code may call `.unwrap_or_else(|e| e.into_inner())` to recover the data.

## What This Unlocks

- **Shared counters and accumulators** — multiple threads safely increment a counter, push to a `Vec`, or update a `HashMap`.
- **Coordinated state machines** — threads check and update a shared status value with mutual exclusion.
- **Job results collection** — worker threads push completed results into a shared `Arc<Mutex<Vec<T>>>` for the main thread to process.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared ownership | GC automatic | `Arc::new(...)` + `Arc::clone(&a)` |
| Lock | `Mutex.lock m` (separate from data) | `m.lock().unwrap()` — data IS in the mutex |
| Unlock | `Mutex.unlock m` — manual | `MutexGuard` drops automatically (RAII) |
| Forget to lock | possible — data and lock separate | impossible — data unreachable without `lock()` |
| Poisoning | N/A | panic while holding = mutex poisoned |
