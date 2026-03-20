📖 **[View on hightechmind.io →](https://hightechmind.io/rust/449-rayon-join)**

---

# 449: Rayon Join — Fork-Join Parallelism
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Divide-and-conquer algorithms (merge sort, quicksort, parallel tree traversal) split a problem into two independent sub-problems that can be solved concurrently. The fork-join model captures this: `join(f, g)` runs both `f` and `g`, potentially in parallel, waiting for both to complete. `rayon::join` is the idiomatic way to express this in Rust — it automatically decides whether to run in parallel (if threads are available) or sequentially (if the thread pool is saturated), adapting to load.

Fork-join appears in parallel merge sort, Fibonacci computation, tree operations, and any divide-and-conquer algorithm where sub-problems are independent.

## Learning Outcomes

- Understand the fork-join concurrency primitive: run two closures in parallel, wait for both
- Learn how `thread::spawn` + `join` implements fork-join for `'static` data
- See how `thread::scope` implements fork-join for borrowed data
- Understand `rayon::join`'s adaptive behavior (parallel or sequential based on pool state)
- Learn when fork-join produces speedup: sub-problems must be large enough to justify overhead

## Rust Application

In `src/lib.rs`, `join` spawns one thread for `f` and runs `g` on the current thread, then joins the handle. `scoped_join` uses `thread::scope` for borrowed data. These are the building blocks for `parallel_sort` which implements a parallel merge sort: sort left half in one thread, right half in another, merge the results. The threshold prevents spawning threads for tiny slices.

## OCaml Approach

OCaml 5.x's `Domain.spawn f` + `Domain.join h` implements fork-join: `let h = Domain.spawn f in let b = g () in let a = Domain.join h in (a, b)`. `Domainslib.Task.async`/`await` provide a higher-level composable version. For recursive divide-and-conquer, `Domainslib.Task.parallel_for` handles the recursion internally. OCaml 4.x's threads achieve fork-join but without parallelism.

## Key Differences

1. **Adaptive**: `rayon::join` runs sequentially when the thread pool is saturated; manual `thread::spawn` always creates a thread.
2. **Borrowed data**: `thread::scope` enables fork-join with borrowed slices; `rayon::join` on borrowed data requires `Sync` bounds.
3. **Overhead**: Thread spawn for `join` costs ~100μs; `rayon::join` uses the existing pool with near-zero overhead.
4. **Nesting**: Recursive `rayon::join` calls build a dynamic tree of tasks; manual recursive spawn causes thread count to grow exponentially.

## Exercises

1. **Parallel merge sort**: Implement complete parallel merge sort using the `join` function. Set a threshold below which sequential sort is used. Benchmark against `slice::sort()` for arrays of 1M, 10M, and 100M elements.
2. **Parallel tree fold**: Given a binary tree of values, implement `parallel_fold(tree, identity, combine)` using fork-join: fold left subtree in one branch, right subtree in another, combine results. Test with a balanced tree of 65535 nodes.
3. **Parallel search**: Use fork-join to implement parallel binary search across a sorted `Vec`. Split at the midpoint, search left and right in parallel, return the first match found. Compare with sequential binary search.
