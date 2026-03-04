# 459: Thread-Local Storage — Per-Thread Private State

**Difficulty:** 3  **Level:** Intermediate

Give each thread its own independent copy of a variable — zero synchronisation, no locks, no sharing by design.

## The Problem This Solves

Not all shared state needs to be shared. Thread IDs, per-thread counters, error accumulators, logging buffers, random number generators — these are values that are conceptually global but belong exclusively to one thread. Forcing them into `Arc<Mutex<T>>` adds unnecessary locking overhead and couples threads that have no need to coordinate.

Thread-local storage (TLS) is the inverse of shared state: each thread gets its own copy, completely isolated. Thread A incrementing its counter has no effect on Thread B's counter. No locks. No atomics. No `Arc`. The variable looks global in the code but behaves like a local.

A concrete example: a per-thread error counter that accumulates during a batch job. At the end, you sum them up. With shared state, every increment contends on a mutex. With TLS, each thread increments freely with zero synchronisation; you only need coordination at the final aggregation.

## The Intuition

`thread_local!` defines a static that is not one variable but N variables — one per thread, created on first access by that thread and destroyed when the thread exits. Each thread sees only its own copy. The macro works with any type, but because TLS values live inside a thread (not on the heap), they can't be `Send` — you can't give your thread-local to another thread.

In Python: `threading.local()`. In Java: `ThreadLocal<T>`. In Go: no direct equivalent (goroutines are too lightweight for TLS; the idiom is passing context explicitly). In C/C++: `thread_local` keyword. Rust's `thread_local!` is the macro form, using `RefCell<T>` for interior mutability since you can't get `&mut T` through a shared reference.

## How It Works in Rust

```rust
use std::cell::RefCell;
use std::thread;

// Declare thread-local variables — each thread gets its own instance
thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
    static LOG: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

// Helper functions — clean API hiding the thread_local! boilerplate
fn inc()           { COUNTER.with(|c| *c.borrow_mut() += 1); }
fn count() -> u64  { COUNTER.with(|c| *c.borrow()) }
fn log(s: &str)    { LOG.with(|l| l.borrow_mut().push(s.to_string())); }

fn main() {
    let handles: Vec<_> = (0..4).map(|id| {
        thread::spawn(move || {
            for _ in 0..100 { inc(); }        // each thread increments its own counter
            log(&format!("thread {} done", id)); // each thread writes its own log
            println!("thread {}: count={}", id, count()); // always 100
        })
    }).collect();
    for h in handles { h.join().unwrap(); }

    // Main thread's counter is completely independent
    inc(); inc();
    println!("main count: {}", count()); // 2 — not affected by thread increments
    println!("main log: {:?}", log_get()); // empty — threads wrote to their own logs
}
```

`RefCell<T>` provides interior mutability (needed because `thread_local!` gives you `&T`, not `&mut T`). Since TLS values are accessed by only one thread, `RefCell` panics (instead of `Mutex` blocking) if you try to hold two `borrow_mut()` guards at once — which would be a logic error within a single thread.

## What This Unlocks

- **Per-thread accumulators** — each thread collects results locally, avoiding all lock contention; aggregate in the main thread at the end.
- **Per-thread random number generators** — seed once per thread, generate with no synchronisation overhead.
- **Request-scoped context in servers** — store the current request ID or user session in TLS so it's accessible throughout the call stack without passing it explicitly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread-local | `Domain.DLS` key/value (OCaml 5) | `thread_local! { static V: RefCell<T> = ... }` |
| Access | `Domain.DLS.get key` | `V.with(\|v\| { ... })` |
| Mutability | via ref | `RefCell<T>` for interior mutability |
| Sharing across threads | impossible by design | impossible — TLS values are not `Sync` |
| Destruction | GC | on thread exit (RAII) |
| Java equivalent | `ThreadLocal<T>` | `thread_local!` macro |
