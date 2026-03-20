📖 **[View on hightechmind.io →](https://hightechmind.io/rust/109-arc-threads)**

---

# 109-arc-threads — Arc<T>: Thread-Safe Shared Ownership
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When multiple threads need read access to the same data, you need a mechanism for shared ownership with thread-safe reference counting. Rust's `Rc<T>` is single-threaded — it uses non-atomic reference counting that is not safe to clone across threads. `Arc<T>` (Atomically Reference-Counted) uses atomic operations for the count, making it safe to share across threads.

`Arc<T>` is the Rust equivalent of `shared_ptr` in C++ (thread-safe variant) and the GC-managed heap references in OCaml, but with explicit reference counting visible to the programmer.

## Learning Outcomes

- Understand when to use `Arc<T>` versus `Rc<T>` (thread-safe vs single-threaded)
- Clone `Arc<T>` to share ownership across thread boundaries
- Combine `Arc<Mutex<T>>` for shared mutable state across threads
- Understand the overhead: atomic operations on the reference count
- Use `Arc` in map-reduce patterns for parallel data processing

## Rust Application

`src/lib.rs` demonstrates two patterns. `parallel_sum` wraps a `Vec<i32>` in `Arc`, clones it for two threads (each clone increments the atomic count), and sums each half in parallel. `map_reduce` shares an `Arc<Config>` across multiple worker threads, each applying the configuration independently.

`Arc<Mutex<T>>` enables shared mutable state: all threads can call `lock()`, which provides exclusive access. The mutex provides the mutual exclusion that `Arc` alone does not.

## OCaml Approach

OCaml's GC is not concurrent by default (the global interpreter lock in pre-5.0 OCaml), but OCaml 5 introduces Domains for true parallelism:

```ocaml
(* OCaml 5 with Domains *)
let parallel_sum data =
  let mid = Array.length data / 2 in
  let d = Domain.spawn (fun () -> Array.fold_left (+) 0 (Array.sub data 0 mid)) in
  let right_sum = Array.fold_left (+) 0 (Array.sub data mid (Array.length data - mid)) in
  Domain.join d + right_sum
```

OCaml's GC handles shared data automatically — no explicit reference counting needed. Data sharing across domains is safe for immutable values.

## Key Differences

1. **Explicit vs implicit**: Rust's `Arc::clone` explicitly increments the reference count; OCaml's GC manages shared references automatically.
2. **Atomic overhead**: Rust's `Arc` uses atomic CAS operations for the count (more expensive than `Rc`'s simple increment); OCaml's GC has its own overhead but it is amortized.
3. **Mutation safety**: Rust requires `Arc<Mutex<T>>` for shared mutation; OCaml 5 uses `Mutex` or `Atomic` from the standard library.
4. **Move vs clone**: In Rust, you must `Arc::clone` before moving into a thread; OCaml values can be used in multiple domains without explicit cloning.

## Exercises

1. Implement a thread pool using `Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>` where worker threads pull and execute tasks.
2. Build a concurrent frequency counter using `Arc<Mutex<HashMap<String, usize>>>` and verify correct counting from multiple threads.
3. Demonstrate that using `Arc<T>` without `Mutex` for shared immutable data is both safe and faster than `Arc<Mutex<T>>` for read-heavy workloads.
