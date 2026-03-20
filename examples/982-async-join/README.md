[async-join on hightechmind.io](https://hightechmind.io/posts/functional-rust/async-join)

---

## Problem Statement

Demonstrate parallel async execution in Rust using `thread::spawn` + `join()` as a synchronous analog of `Lwt.both` and `Lwt.all`. Spawn multiple threads for independent computations, then join all threads to collect results. Implement both a two-thread join and a parallel map over a vector of tasks.

## Learning Outcomes

- Implement `parallel_both<A, B>` spawning two threads and joining both — analog of `Lwt.both`
- Implement `parallel_map<T, F>` spawning one thread per task and collecting results — analog of `Lwt.all`
- Understand the `Send + 'static` bounds: types crossing thread boundaries must be `Send`; closures must own their data
- Distinguish sequential join (wait then proceed) from `async` join (both run concurrently, resolved together)
- Recognize that `thread::spawn` + `join` is the sync version; `tokio::join!` is the async equivalent

## Rust Application

```rust
fn parallel_both<A, B, F1, F2>(f1: F1, f2: F2) -> (A, B)
where
    A: Send + 'static,
    B: Send + 'static,
    F1: FnOnce() -> A + Send + 'static,
    F2: FnOnce() -> B + Send + 'static,
{
    let h1 = thread::spawn(f1);
    let h2 = thread::spawn(f2);
    let a = h1.join().expect("thread 1 panicked");
    let b = h2.join().expect("thread 2 panicked");
    (a, b)
}

fn parallel_map<T, F>(tasks: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handles: Vec<_> = tasks.into_iter().map(thread::spawn).collect();
    handles.into_iter()
        .map(|h| h.join().expect("task panicked"))
        .collect()
}
```

`thread::spawn` returns a `JoinHandle<T>`. Calling `.join()` on the handle blocks the current thread until the spawned thread completes and returns its result as `Result<T, Box<dyn Any>>`. `.expect("panicked")` propagates panics from worker threads to the caller.

Both threads run concurrently between `spawn` and `join`. The total time is `max(t1, t2)` not `t1 + t2` — the hallmark of parallel execution.

The `T: Send + 'static` bound ensures the result can be moved across thread boundaries. `'static` means the closure cannot borrow from the current stack frame — it must capture owned data.

## OCaml Approach

```ocaml
open Lwt

(* Lwt.both: run two promises concurrently, wait for both *)
let parallel_both f1 f2 =
  Lwt.both (f1 ()) (f2 ())

(* Lwt.all: run a list of promises concurrently *)
let parallel_map tasks =
  Lwt.all (List.map (fun f -> f ()) tasks)

(* Thread-based parallel in OCaml (5.0+ domains) *)
let parallel_both_domain f1 f2 =
  let d1 = Domain.spawn f1 in
  let d2 = Domain.spawn f2 in
  let a = Domain.join d1 in
  let b = Domain.join d2 in
  (a, b)
```

OCaml's `Lwt.both` cooperatively runs two promises on a single thread (via the Lwt scheduler). For true OS-level parallelism, OCaml 5.0+ `Domain.spawn` is the equivalent of `thread::spawn`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| OS threads | `thread::spawn` — always real OS thread | `Thread.create` — real OS thread |
| Async parallel | `tokio::join!(f1, f2)` | `Lwt.both f1 f2` |
| Parallel domains | `std::thread` | `Domain.spawn` (OCaml 5+) |
| `Send` bound | Required for cross-thread values | No equivalent (GC manages) |
| Join result | `Result<T, Box<dyn Any>>` | Raises exception on join if domain panicked |

`thread::spawn` creates a real OS thread. For high-concurrency workloads with many short tasks, prefer a thread pool (example 923) or async runtime over spawning one thread per task.

## Exercises

1. Rewrite `parallel_map` to use a fixed-size thread pool (from example 923) instead of spawning per task.
2. Add error handling: change task return type to `Result<T, String>` and handle panics gracefully.
3. Implement `parallel_filter<T, F>(items: Vec<T>, pred: F) -> Vec<T>` that tests items in parallel.
4. Measure the speedup of `parallel_map` vs sequential map for 8 CPU-bound tasks on an 8-core machine.
5. Rewrite using `tokio::join!` and `tokio::task::spawn` to compare async vs thread-based parallelism.
