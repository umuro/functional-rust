📖 **[View on hightechmind.io →](https://hightechmind.io/rust/448-rayon-parallel)**

---

# 448: Rayon Parallel Iterators — Data Parallelism
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Converting sequential code to parallel is usually complex: thread management, load balancing, result collection. Rayon's parallel iterators solve this: replace `.iter()` with `.par_iter()` and Rayon handles thread spawning, work distribution, and result aggregation. The library implements parallel `map`, `filter`, `fold`, `sum`, and 40+ other operations. This example demonstrates the underlying pattern using `thread::scope` to show what Rayon does internally.

Rayon is used in data processing pipelines, image rendering, scientific simulations, build systems (Cargo uses Rayon for compilation), and any CPU-bound iteration over large datasets.

## Learning Outcomes

- Understand the parallel iterator pattern: chunk data, process in parallel, collect results
- Learn how `thread::available_parallelism()` determines the optimal thread count
- See how chunk-based parallel map avoids the overhead of one-thread-per-element
- Understand how parallel reduce/fold works: local reductions joined in a tree
- Learn the data parallelism model vs. task parallelism (different granularity)

## Rust Application

In `src/lib.rs`, `parallel_map` chunks input data into `num_threads` chunks using `thread::available_parallelism()`. Each chunk is processed by a scoped thread using the function `f`. Results are pre-allocated and each thread fills its slice in-place. `parallel_sum` splits the array in half recursively down to a threshold, then sums sequentially. This demonstrates the divide-and-conquer reduction tree used by `rayon::sum()`.

## OCaml Approach

OCaml 5.x's `Domainslib.Task.parallel_for` divides a range among domains: `Task.parallel_for pool ~start:0 ~finish:n ~body:(fun i -> process arr.(i))`. `Domainslib.Task.async` + `Task.await` provide future-style composition. OCaml 4.x has no true parallel iteration due to the GIL. The functional style makes parallel operations natural — pure functions with no shared state are trivially parallelizable.

## Key Differences

1. **API ergonomics**: Rayon's `.par_iter()` requires zero code change beyond the method name; this example requires manual chunking. OCaml's `Domainslib.parallel_for` is also low-level.
2. **Work stealing**: Rayon uses work stealing for load balancing; this example's fixed-chunk approach can be imbalanced.
3. **Composability**: Rayon's parallel iterators compose: `.par_iter().filter(pred).map(f).sum()` is all parallel; manual chunking requires re-chunking at each stage.
4. **Overhead**: Rayon's global thread pool amortizes startup costs; `thread::scope` creates threads per scope (but with scoped thread caching in some implementations).

## Exercises

1. **Image processing**: Load a grayscale image as `Vec<u8>`. Apply a blur filter to each pixel in parallel using the chunked parallel_map pattern. Verify the result matches sequential processing.
2. **Word count**: Given `Vec<String>` of sentences, count total words in parallel using parallel_map (count per sentence) followed by a parallel sum. Compare performance vs. sequential for 1M sentences.
3. **Matrix transpose**: Implement parallel matrix transpose using `thread::scope`. Divide rows among threads; each thread writes its assigned rows to the transposed positions. Verify correctness and benchmark vs. sequential.
