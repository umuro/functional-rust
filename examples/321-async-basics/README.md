📖 **[View on hightechmind.io →](https://hightechmind.io/rust/321-async-basics)**

---

# 321: async fn and .await Fundamentals
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Network servers, database clients, and file-processing applications spend most of their time waiting for I/O. Blocking threads during waits is wasteful — a single server handling 10,000 connections would need 10,000 threads. Async/await enables concurrent I/O on a small thread pool by pausing and resuming tasks when they would otherwise block. This example demonstrates the fundamental concepts using synchronous thread-based analogies before introducing true async syntax.

## Learning Outcomes

- Understand the difference between sequential blocking and concurrent execution
- Recognize that `async fn` creates a future that is lazy until `.await`ed
- See how `join!` or thread spawning enables concurrent execution vs sequential
- Understand why concurrency reduces total wall-clock time for I/O-bound work

## Rust Application

The examples demonstrate sequential vs concurrent execution patterns. Thread-based simulation shows the concept before async runtimes (Tokio) are introduced:

```rust
// Sequential: total time = sum of all delays
let user = fetch_user(1);   // 10ms
let posts = fetch_posts(1); // 8ms
// Total: ~18ms

// Concurrent: total time = max of all delays
let handle_user = thread::spawn(|| fetch_user(1));
let handle_posts = thread::spawn(|| fetch_posts(1));
let user = handle_user.join().unwrap();
let posts = handle_posts.join().unwrap();
// Total: ~10ms
```

## OCaml Approach

OCaml's `Lwt` and `Async` libraries provide similar async/await functionality. `Lwt.both` is the equivalent of `join!`:

```ocaml
(* Lwt: concurrent fetch *)
let* (user, posts) = Lwt.both
  (fetch_user 1)
  (fetch_posts 1)
```

OCaml 5.0's `Effect` system and `Domain` provide even lower-level concurrency primitives.

## Key Differences

1. **Runtime required**: Rust async requires a runtime (`tokio`, `async-std`); OCaml's `Lwt`/`Async` are also libraries, not language builtins.
2. **Syntax**: Rust uses `async fn` + `.await`; OCaml uses `let*` / `>>=` with promise types.
3. **Compilation**: Rust transforms `async fn` into state machines at compile time; OCaml's `Lwt` uses continuation closures at runtime.
4. **Thread model**: Rust's async is cooperative (explicit `.await` points); OCaml 5's `Domain` uses OS threads with shared memory.

## Exercises

1. Measure the wall-clock time difference between sequential and concurrent thread-based fetches for 5 operations of varying latency.
2. Implement a `concurrent_map(items: Vec<T>, f: Fn(T) -> R) -> Vec<R>` that processes all items in parallel using threads.
3. Identify which operations in a sequential workflow are independent (can be parallelized) vs dependent (must be sequential).
