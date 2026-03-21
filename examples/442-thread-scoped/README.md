📖 **[View on hightechmind.io →](https://hightechmind.io/rust/442-thread-scoped)**

---

# 442: Scoped Threads — Borrowing Across Threads
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

`thread::spawn` requires `'static` data — you can't borrow a local variable across threads because the spawned thread might outlive the caller's stack frame. This forces `Arc<T>` and cloning even when you just want to process slices of a local array in parallel. `thread::scope` (stabilized in Rust 1.63) solves this: scoped threads are guaranteed to complete before the scope exits, so they can safely borrow any data from the enclosing scope — including stack-allocated slices, without `Arc` or `clone`.

Scoped threads enable efficient parallel processing of local data: parallel prefix sums, parallel sorting passes, parallel data transformation — all with zero heap allocation overhead.

## Learning Outcomes

- Understand why `thread::spawn` requires `'static` but `thread::scope` does not
- Learn how `scope.spawn(|| borrowed_data)` borrows data safely within the scope lifetime
- See how `parallel_sum` splits a slice and processes halves concurrently
- Understand the scope guarantee: all threads are joined before `thread::scope` returns
- Learn when to prefer scoped threads over `Arc<T>` + `send` threads

## Rust Application

In `src/lib.rs`, `parallel_sum` uses `thread::scope(|s| { let t1 = s.spawn(|| left.iter().sum()); ... })`. The `left` and `right` slices are borrowed from the enclosing function's `data` parameter — no `Arc`, no clone. Both threads are joined within the scope, ensuring the borrows remain valid. `parallel_map` chunks the data and processes each chunk with a scoped thread, collecting results back into a `Vec`.

## OCaml Approach

OCaml's `Thread.create` requires heap-allocated data — OCaml's GC manages lifetimes so there's no stack-lifetime restriction. Any OCaml value can be shared across threads without the `'static` requirement. However, mutable state still requires synchronization (`Mutex.t`). OCaml 5.x's `Domain.spawn` has similar freedom — domains share the heap and can access any allocated value.

## Key Differences

1. **Lifetime restriction**: Rust's `spawn` requires `'static`; scoped threads lift this. OCaml has no lifetime restriction since GC manages all values.
2. **Allocation overhead**: Rust's scoped threads avoid `Arc` allocation; OCaml always uses heap allocation.
3. **Guarantee mechanism**: Rust's scope is a closure that joins all threads on exit — enforced by the borrow checker; OCaml has no equivalent guarantee.
4. **Rayon comparison**: `rayon::scope` extends this pattern with work stealing for better load balancing; `std::thread::scope` is the simpler no-dependency version.

## Exercises

1. **Parallel prefix sum**: Use `thread::scope` to compute prefix sums in parallel: split the array into N chunks, compute each chunk's sum in parallel, then do a sequential pass to add the previous chunk's total to each chunk's elements.
2. **Parallel quicksort**: Implement in-place parallel quicksort using scoped threads: partition the array, then sort both partitions in separate threads using `thread::scope`. Stop spawning threads when partitions are smaller than a threshold.
3. **Parallel matrix multiply**: Use `thread::scope` to multiply two matrices by assigning each output row to a separate thread. Verify results match sequential multiplication.
