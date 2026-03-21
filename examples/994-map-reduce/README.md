**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[map-reduce on hightechmind.io](https://hightechmind.io/posts/functional-rust/map-reduce)

---

## Problem Statement

Implement parallel map-reduce: a `parallel_map` that spawns one thread per item, then a sequential `fold` that combines results. Also implement a `chunked_parallel_map` that divides work into `num_workers` chunks for better efficiency on large datasets. These patterns mirror the Google MapReduce model at thread granularity.

## Learning Outcomes

- Implement `parallel_map<T, U, F: Fn(T) -> U + Send + Sync>(items, f) -> Vec<U>` using `Arc<F>` and thread spawning
- Implement `map_reduce<T, U, R>(items, map_fn, reduce_fn, init)` as `parallel_map` then `fold`
- Use `Arc<F>` to share the map function across threads (required because `Fn` is `Sync` but threads need owned `Arc`)
- Implement chunked parallel map: split items into `num_workers` chunks, process each chunk in a thread
- Compare `parallel_map` (one thread per item) vs `chunked_parallel_map` (one thread per worker) for overhead tradeoffs

## Rust Application

```rust
fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static, U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let handles: Vec<_> = items.into_iter()
        .map(|item| {
            let f = Arc::clone(&f);
            thread::spawn(move || f(item))
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn map_reduce<T, U, R, F, G>(items: Vec<T>, map_fn: F, reduce_fn: G, init: R) -> R
where
    T: Send + 'static, U: Send + 'static, R: 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
    G: Fn(R, U) -> R,
{
    let mapped = parallel_map(items, map_fn);
    mapped.into_iter().fold(init, reduce_fn)
}

fn chunked_parallel_map<T, U, F>(items: Vec<T>, f: F, num_workers: usize) -> Vec<U>
where
    T: Send + 'static, U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let chunk_size = items.len().div_ceil(num_workers);
    // ... chunk into num_workers groups, spawn per chunk, collect in order
}
```

`Arc<F>` allows the closure to be shared across threads. `F: Sync` is required (implied by `Arc<F>: Send`) because multiple threads call it concurrently. `F: Fn(T) -> U` means `f` can be called multiple times — closures capturing only immutable state satisfy this.

`map_reduce` is: parallel map → collect all results → sequential fold. The `reduce_fn` is not parallelized because it processes results in order (fold requires sequential accumulation). For commutative, associative reductions, a parallel tree-reduce would be more efficient.

## OCaml Approach

```ocaml
(* OCaml 5.0+: Domainslib *)
let parallel_map pool items f =
  let tasks = List.map (fun item ->
    Domainslib.Task.async pool (fun () -> f item)
  ) items in
  List.map (Domainslib.Task.await pool) tasks

let map_reduce pool items map_fn reduce_fn init =
  let mapped = parallel_map pool items map_fn in
  List.fold_left reduce_fn init mapped

(* Pre-5.0 with Thread *)
let parallel_map_thread items f =
  let handles = List.map (fun item ->
    Thread.create f item
  ) items in
  List.map Thread.join handles
```

`Domainslib.Task.async` submits a task to the pool; `Task.await` blocks until the result is available — equivalent to `thread::spawn` + `JoinHandle::join`. The map-reduce structure is identical.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Function sharing | `Arc<F>` required for cross-thread shared fn | GC shares function closures automatically |
| `Fn` vs `FnOnce` | `Fn` for reusable across threads | Closures are GC values, always reusable |
| Chunk parallelism | Manual chunk + thread split | `Domainslib.Task.parallel_for` |
| Result ordering | Maintained (collect in spawn order) | Must explicitly maintain order |

`parallel_map` with one thread per item has high thread-creation overhead for small items. `chunked_parallel_map` with `num_workers = cpu_count` amortizes this. `rayon::par_iter()` provides the production-grade version with work stealing.

## Exercises

1. Implement `parallel_filter<T, F>(items, pred) -> Vec<T>` where `pred` runs in parallel.
2. Implement a tree-parallel reduce: split items in half, reduce each half in parallel, combine.
3. Benchmark one-thread-per-item vs chunked vs `rayon::par_iter()` for 1,000 items of varying computation cost.
4. Implement `parallel_sort<T: Ord + Send>` using parallel merge sort with thread spawning.
5. Implement a word-frequency count over a large text file using map-reduce: map = per-line word count, reduce = merge counts.
