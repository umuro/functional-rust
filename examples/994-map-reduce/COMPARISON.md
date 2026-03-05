# MapReduce — Comparison

## Core Insight
MapReduce separates **what to compute** (map: pure, parallel) from **how to combine** (reduce: sequential, order-dependent). Because the map phase is pure, all elements can run in parallel with zero synchronization.

## OCaml Approach
- `parallel_map f xs`: spawn one thread per element, store results in array by index
- Index-based array avoids ordering issues — `results.(i) <- Some (f arr.(i))`
- `fold_left` for the reduce phase (sequential)
- Chunked variant: divide list into N chunks, one thread per chunk
- `List.filter_map Fun.id` to unwrap `option` results

## Rust Approach
- `parallel_map`: `items.into_iter().map(|x| spawn(|| f(x))).collect()` then join all
- `Arc<F>` to share the function across threads without copying
- Results come back in **spawn order** (join order preserves it)
- `map_reduce` = `parallel_map` + `fold`
- For large N: use Rayon's `par_iter()` for automatic chunking

## Comparison Table

| Concept             | OCaml                                  | Rust                                |
|---------------------|----------------------------------------|-------------------------------------|
| Parallel map        | `List.mapi (fun i -> Thread.create)` | `items.map(|x| spawn(|| f(x)))`    |
| Preserve order      | Array index: `results.(i) <- v`        | Join order matches spawn order      |
| Reduce              | `List.fold_left reduce_fn init`        | `mapped.into_iter().fold(init, f)`  |
| Chunking            | Manual `chunk_size` + slice            | Rayon `chunks(n).par_bridge()`      |
| Generic signature   | `('a -> 'b) -> 'a list -> 'b list`    | `F: Fn(T)->U + Send + Sync + 'static` |
| Pure map required   | Yes — no shared mutation in map        | Yes — `FnOnce` moves data           |
| Production          | `Domains.parallel_map` (OCaml 5)       | Rayon `par_iter().map().sum()`      |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
