📖 **[View on hightechmind.io →](https://hightechmind.io/rust/469-parallel-reduce)**

---

# Parallel Reduce
**Difficulty:** ⭐  
**Category:** Functional Programming  



A divide-and-conquer aggregation pattern that splits a slice in half, reduces each half in a separate thread, and combines the results — exposing parallelism for associative operations.

## Problem Statement

Sequential `fold` over a million-element slice processes one element at a time. If the combining operation is **associative** (addition, multiplication, min, max, logical AND/OR), the computation can be split into independent subtrees and evaluated in parallel. MapReduce (Dean & Ghemawat, 2004), parallel prefix scans in GPU computing, and Spark's `aggregate` all rely on this algebraic observation. The wall-clock time drops from O(N) to O(N/P + log P) where P is the number of processors.

## Learning Outcomes

- Apply divide-and-conquer recursion to parallel aggregation
- Use `thread::scope` to borrow slice halves without `Arc` or `'static` bounds
- Express the `T: Send + Sync + Clone` constraints needed for cross-thread data
- Implement parallel sum, product, min, max, all, any, and count as specialisations
- Choose a sequential threshold to amortise thread-spawn overhead

## Rust Application

`parallel_reduce` bisects the slice at `mid`, spawns a scoped thread for the left half, computes the right half on the current thread, then joins and combines:

```rust
thread::scope(|s| {
    let left_handle = s.spawn(|| parallel_reduce(left, id_left, op));
    let right_result = parallel_reduce(right, identity, op);
    op(left_handle.join().unwrap(), right_result)
})
```

A `THRESHOLD` of 100 elements prevents excessive thread creation on small inputs. `parallel_sum` delegates to `parallel_reduce` with `T::default()` as the identity and `a + b` as the combiner. `parallel_all` and `parallel_any` are short-circuit variants that do not need the reduce infrastructure — they call `iter().all/any` below the threshold.

## OCaml Approach

Multicore OCaml uses `Domainslib.Task.parallel_for_reduce`:

```ocaml
let pool = Domainslib.Task.setup_pool ~num_additional_domains:3 ()
let sum = Domainslib.Task.run pool (fun () ->
  Domainslib.Task.parallel_for_reduce pool
    ~start:0 ~finish:(Array.length arr - 1)
    ~body:(fun i -> arr.(i))
    (+) 0)
```

Functional OCaml without Domainslib uses immutable arrays and `List.fold_left` sequentially — parallel reduction requires explicit domain creation via `Domain.spawn`.

## Key Differences

1. **Scoped threads**: Rust's `thread::scope` borrows the slice directly without cloning; OCaml's `Domain.spawn` requires values to be shared across domain boundaries, usually via `Atomic` refs.
2. **Trait bounds**: Rust's `T: Send + Sync + Clone` is checked at compile time for every call site; OCaml has no equivalent — unsynchronised domain access is a runtime data race.
3. **Work stealing**: The example does static bisection; Rayon (examples 448-449) uses work-stealing to dynamically balance uneven workloads.
4. **Associativity requirement**: Rust's type system cannot enforce that `op` is associative; the algorithm is correct only when it is. OCaml has the same gap.

## Exercises

1. **Chunked parallelism**: Replace recursive bisection with a fixed chunk count equal to `available_parallelism()`. Measure whether this reduces thread-spawn overhead.
2. **Generic identity**: Implement a `Monoid<T>` trait with `identity() -> T` and `combine(T, T) -> T`, then rewrite `parallel_reduce` to accept `M: Monoid<T>` instead of separate `identity` and `op` arguments.
3. **Prefix scan**: Implement `parallel_prefix_sum(data: &[i64]) -> Vec<i64>` that returns the inclusive prefix sum array using a parallel up-sweep and down-sweep (Blelloch scan algorithm).
