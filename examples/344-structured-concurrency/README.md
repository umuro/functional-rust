📖 **[View on hightechmind.io →](https://hightechmind.io/rust/344-structured-concurrency)**

---

# 344: Structured Concurrency
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Spawning threads that outlive their creator creates "orphan threads" — tasks that may read freed memory, hold resources after the owning scope exits, or silently swallow panics. Structured concurrency (Nathaniel J. Smith, 2018; popularized by Kotlin coroutines and Python's `anyio`) ensures that spawned tasks are strictly scoped: they cannot outlive the block that created them. Rust's `thread::scope` implements this at the language level — borrowed references are valid for the entire scope, and the scope blocks until all spawned threads complete. No raw `Arc` needed for data shared with child threads; borrows work directly.

## Learning Outcomes

- Use `thread::scope` to spawn threads that borrow data from the enclosing scope
- Understand that the scope automatically joins all threads before returning
- Recognize that scoped threads can borrow `&[T]` without cloning to `Arc`
- Implement divide-and-conquer parallelism (parallel reduce) using scoped threads
- Distinguish structured (`thread::scope`) from unstructured (`thread::spawn`) concurrency
- See how structured concurrency prevents resource leaks and dangling references

## Rust Application

```rust
use std::thread;

pub fn parallel_sum(nums: &[i32]) -> i32 {
    if nums.len() < 100 {
        return nums.iter().sum();
    }
    let (left, right) = nums.split_at(nums.len() / 2);
    thread::scope(|s| {
        let l = s.spawn(|| parallel_sum(left));   // borrows left directly
        let r = s.spawn(|| parallel_sum(right));  // borrows right directly
        l.join().unwrap() + r.join().unwrap()
    })
    // scope blocks here until both threads finish
    // left and right borrows are guaranteed valid throughout
}
```

Without `thread::scope`, you would need `Arc<[i32]>` to share slices across `thread::spawn` boundaries. Scoped threads eliminate this: the compiler proves through lifetime analysis that `nums` outlives both child threads because the scope is a nested block within the function.

## OCaml Approach

OCaml 5 domains provide similar structured parallelism:

```ocaml
let parallel_sum nums =
  let n = Array.length nums in
  let mid = n / 2 in
  let left = Array.sub nums 0 mid in
  let right = Array.sub nums mid (n - mid) in
  let d = Domain.spawn (fun () -> Array.fold_left (+) 0 left) in
  let r_sum = Array.fold_left (+) 0 right in
  r_sum + Domain.join d
```

OCaml copies subarrays (GC-safe) rather than borrowing slices. `Domain.join` plays the role of `scope` exit — it blocks until the spawned domain finishes. Unlike Rust's scope, OCaml doesn't statically prevent domains from escaping their creation context, but `Domain.join` achieves the same runtime guarantee.

## Key Differences

| Aspect | Rust `thread::scope` | OCaml `Domain.spawn` + `join` |
|--------|---------------------|-------------------------------|
| Borrow across threads | Yes — compiler-verified | No — must copy or use `Arc`-equivalent |
| Auto-join on exit | Yes — scope guarantees it | Manual `Domain.join` required |
| Panic propagation | Propagated on `join().unwrap()` | `Domain.join` re-raises |
| Nesting | Works recursively | Works recursively |
| Overhead | OS threads | Domains (lighter than OS threads) |

## Exercises

1. **Parallel map**: Implement `parallel_map<T, R>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R>` using `thread::scope`, splitting the slice into as many chunks as CPU cores.
2. **Early termination**: Modify `parallel_sum` to abort early if either half panics — propagate the panic correctly instead of panicking twice.
3. **Depth limit**: Add a `depth` parameter to the recursive `parallel_sum` that falls back to sequential when depth reaches 0; find experimentally at what depth the overhead of spawning exceeds the parallel benefit.
