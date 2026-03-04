# 109: Arc\<T\> — Thread-Safe Shared Ownership

**Difficulty:** 2  **Level:** Intermediate

`Arc<T>` shares immutable data across threads safely — atomic reference counting ensures the value lives until every thread is done with it.

## The Problem This Solves

Multithreaded programming in C is a minefield. You spawn a thread, pass it a pointer, and the parent thread might free the data before the child thread finishes reading it. The result is a use-after-free data race: unpredictable crashes, corrupted data, security vulnerabilities. Adding a mutex helps with mutation, but the lifetime problem — ensuring the data outlives all the threads using it — is separate and still manual.

In Java or Go, the GC handles this: data lives as long as any thread holds a reference. But you're paying GC overhead for all your objects, and data races on mutation are still possible (Java's `synchronized`, Go's `sync.Mutex`).

Rust's `Arc<T>` (Atomic Reference Count) gives you exactly what you need: the data lives until the last thread drops its `Arc`. The atomic reference counting is thread-safe. And because `Arc<T>` gives you shared *immutable* access (`&T` semantics), there are no data races on reads. For mutation, combine with `Mutex<T>` — the compiler requires it.

## The Intuition

`Arc<T>` is `Rc<T>` with atomic (thread-safe) reference counting — clone it to share ownership across threads, and the data is freed only after every thread drops its clone.

## How It Works in Rust

```rust
use std::sync::Arc;
use std::thread;

fn demo_read_sharing() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let mut handles = vec![];
    for i in 0..3 {
        let data_clone = Arc::clone(&data); // clone the Arc, not the data
        let handle = thread::spawn(move || {
            // Each thread gets its own Arc pointing to the same Vec
            println!("Thread {}: sum = {}", i, data_clone.iter().sum::<i32>());
        });
        handles.push(handle);
    }
    
    for h in handles { h.join().unwrap(); }
    println!("Main still has data: {:?}", data); // still valid
}

// For mutation across threads: Arc<Mutex<T>>
use std::sync::Mutex;

fn demo_shared_mutation() {
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = counter.lock().unwrap(); // lock before writing
            *n += 1;
            // lock released when `n` drops
        }));
    }
    
    for h in handles { h.join().unwrap(); }
    println!("Final count: {}", *counter.lock().unwrap()); // 10
}

// The compiler prevents sending non-thread-safe types across threads
// Rc<T> is NOT Send — can't move it to another thread
// Arc<T> IS Send — designed for cross-thread sharing
```

## What This Unlocks

- **Data lives until the last thread is done** — no manual lifetime tracking across threads; the atomic reference count handles it.
- **Zero-cost reads** — multiple threads reading through `Arc<T>` require no locks; the `Arc` itself is the only synchronization (for reads).
- **Explicit mutation protocol** — when you *do* need mutation, `Arc<Mutex<T>>` makes the synchronization visible in the type; the compiler ensures you can't forget the lock.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread-safe shared data | GC manages lifetime automatically | `Arc<T>` — opt-in atomic reference counting |
| Shared mutation across threads | `Mutex.t` wrapping | `Arc<Mutex<T>>` — type encodes the pattern |
| Preventing data races | Programmer's responsibility | Compiler refuses to send non-`Sync` types across threads |
| Difference from single-thread sharing | N/A (GC is always safe) | `Rc` → single-threaded; `Arc` → multi-threaded |
| Performance | GC overhead on all values | Atomic ops only on `Arc` clones/drops; reads are free |
