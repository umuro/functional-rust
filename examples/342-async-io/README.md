📖 **[View on hightechmind.io →](https://hightechmind.io/rust/342-async-io)**

---

# 342: Async I/O Concepts
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Servers that handle thousands of simultaneous network connections cannot dedicate one OS thread per connection — thread stacks alone would consume gigabytes of memory. Async I/O solves this by decoupling waiting from threads: while one operation waits for a disk read or network packet, the same thread processes other work. This model traces back to the C10K problem (Dan Kegel, 1999) and the design of event loops in Node.js, nginx, and later Tokio. Rust's `async`/`await` brings this efficiency without sacrificing type safety or requiring a garbage collector, achieving C-level throughput with safe, readable code.

## Learning Outcomes

- Understand the difference between blocking and non-blocking I/O at the system call level
- See how polling-based I/O avoids wasting threads on waiting
- Use channels to simulate the async fan-out pattern with threads
- Recognize that `async fn` compiles to a state machine that calls `poll()`
- Understand that an executor drives futures by repeatedly polling them until `Ready`
- Compare async overhead (task scheduling) vs thread overhead (stack allocation)

## Rust Application

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Blocking: one thread waits, doing nothing
fn blocking_read() -> String {
    thread::sleep(Duration::from_millis(10));
    "data".to_string()
}

// Parallel: multiple threads, overlap waiting
fn parallel_reads() -> Vec<String> {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    thread::spawn(move || { thread::sleep(Duration::from_millis(10)); tx1.send("r1".into()).unwrap(); });
    thread::spawn(move || { thread::sleep(Duration::from_millis(10)); tx2.send("r2".into()).unwrap(); });
    (0..2).map(|_| rx.recv().unwrap()).collect()
}

// Polling simulation: check readiness without blocking
enum PollResult<T> { Ready(T), Pending }

fn simulate_poll(ready: bool, value: i32) -> PollResult<i32> {
    if ready { PollResult::Ready(value) } else { PollResult::Pending }
}
```

In real async Rust with Tokio: `tokio::fs::read_to_string` is non-blocking — the calling task suspends, the thread is freed to run other tasks, and the task resumes when the OS signals readiness via `epoll`/`kqueue`/`io_uring`.

## OCaml Approach

OCaml's `Lwt` library implements cooperative async I/O via promises:

```ocaml
let%lwt content1 = Lwt_io.read_file "a.txt" in
let%lwt content2 = Lwt_io.read_file "b.txt" in
(* sequential - overlapping requires Lwt.both *)
let%lwt (c1, c2) = Lwt.both
  (Lwt_io.read_file "a.txt")
  (Lwt_io.read_file "b.txt") in
Lwt_io.printf "%s %s\n" c1 c2
```

`Lwt.both` runs two I/O operations concurrently on one thread, analogous to `tokio::join!`. Both Lwt and Tokio use an event loop driven by OS-level readiness notifications.

## Key Differences

| Aspect | Rust async/await | OCaml Lwt |
|--------|-----------------|-----------|
| Concurrency model | Multi-threaded (Tokio) or single-thread | Single-threaded by default |
| Syntax | `async fn` / `await` built into language | `let%lwt` PPX rewriter macro |
| Error propagation | `?` operator in async context | `Lwt_result.bind` or `let*` |
| Cancellation | `tokio::select!` / `CancellationToken` | `Lwt.cancel` |
| Zero-cost | Yes (state machines, no allocation) | No (heap-allocated continuations) |

## Exercises

1. **Timer overlap**: Simulate 5 independent 10ms delays overlapping: spawn 5 threads, each sleeping 10ms and sending to a channel; measure that total elapsed time is ~10ms not ~50ms.
2. **Polling loop**: Implement a simple polling loop that calls `simulate_poll` every millisecond until it returns `Ready` — then refactor to use a `Condvar` to avoid busy-waiting.
3. **Async port**: Using `tokio`, rewrite `parallel_reads` as an async function using `tokio::join!` with two `tokio::time::sleep` futures; verify it completes in ~10ms not ~20ms.
