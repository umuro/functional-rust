📖 **[View on hightechmind.io →](https://hightechmind.io/rust/441-thread-basics)**

---

# 441: Thread Basics — Spawn and Join

**Difficulty:** 3  **Level:** Intermediate

Launch OS threads with `std::thread::spawn`, collect results with `JoinHandle::join`, and catch panics without crashing the process.

## The Problem This Solves

Without threads, your program uses one CPU core. A 4-core machine sitting at 25% utilisation while your computation churns through a list is a missed opportunity. The classic fix — "just use a thread" — comes with a trap: data races. Two threads writing the same memory without coordination produces corrupted results that are silent and timing-dependent.

In Python or Java you reach for a lock and hope you remember to release it. In Go you get goroutines and channels but the race detector is optional and ships separately. Rust takes a different approach: if your code doesn't statically guarantee that no two threads touch the same data unsafely, **it doesn't compile**. Thread safety is a property proved at compile time, not checked at runtime.

`JoinHandle` also gives you panic containment for free. A child thread panic does not crash the parent — it becomes an `Err` on `.join()`. Your process stays alive and can report the failure cleanly.

## The Intuition

`thread::spawn` maps directly to an OS thread — heavier than Go goroutines, lighter than processes. The closure captures its environment by value (`move`) so the thread owns everything it needs. The `JoinHandle` is your receipt: call `.join()` to wait for the thread and get its return value back. Drop the handle without joining and the thread keeps running detached.

In Python you'd write `t = threading.Thread(target=f); t.start(); t.join()`. The Rust version is similar in shape but the compiler statically verifies the captured data is safe to send to another thread.

## How It Works in Rust

```rust
use std::thread;

// spawn returns a JoinHandle<T> where T is the closure's return type
let handles: Vec<_> = (0..4u32).map(|i| {
    thread::spawn(move || {   // move: closure takes ownership of i
        i * i                 // return value — available via .join()
    })
}).collect();

// join blocks until the thread finishes; returns Result<T, Box<dyn Any>>
let results: Vec<u32> = handles
    .into_iter()
    .map(|h| h.join().unwrap())
    .collect();

// Panic in a child thread → Err, not a process crash
let h = thread::spawn(|| -> i32 { panic!("boom") });
match h.join() {
    Ok(v)  => println!("got {}", v),
    Err(_) => println!("child panicked — caught safely"),
}
```

`move` before the closure is required whenever you capture variables — the thread might outlive the current stack frame. The compiler rejects non-`move` captures that would leave dangling references.

## What This Unlocks

- **Parallel computation** — split a workload into N chunks, spawn N threads, join and merge results.
- **Background tasks** — spawn a thread to handle I/O or logging while the main thread continues.
- **Panic isolation** — run untrusted or unstable code in a thread and handle failures gracefully without aborting the process.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Spawn | `Thread.create f arg` | `thread::spawn(move \|\| { ... })` |
| Join | `Thread.join handle` | `handle.join().unwrap()` |
| Return value | unit only | any `Send + 'static` type |
| Panic safety | uncaught exception crashes domain | `Err(Box<dyn Any>)` returned to joiner |
| Data capture | GC handles lifetimes | `move` closure — compiler enforces ownership |
