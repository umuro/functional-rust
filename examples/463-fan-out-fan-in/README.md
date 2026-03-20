📖 **[View on hightechmind.io →](https://hightechmind.io/rust/463-fan-out-fan-in)**

---

# 463: Fan-Out / Fan-In Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

One slow processing step can bottleneck an entire pipeline. Fan-out distributes work items from one source to N parallel workers; fan-in collects results from all N workers back to one consumer. Together, they parallelize the bottleneck stage without changing the serial stages around it. If one processing step takes 100ms and you have 8 cores, 8 parallel workers reduce the step's throughput contribution to ~12.5ms per item — 8x improvement.

Fan-out/fan-in appears in MapReduce frameworks, parallel database aggregations, web crawler link processing, batch ML inference, and any stage requiring horizontal scaling.

## Learning Outcomes

- Understand fan-out: distributing work items to multiple concurrent workers
- Learn fan-in: collecting results from multiple workers into a single channel
- See how `Arc<Mutex<Iterator>>` enables work stealing among workers
- Understand channel close as the shutdown signal: all workers drop their `tx` clones
- Learn the pattern's relationship to MapReduce (fan-out = map, fan-in = reduce)

## Rust Application

In `src/lib.rs`, `fan_map` wraps the input iterator in `Arc<Mutex<...>>`. Each worker thread locks the mutex, takes the next item, unlocks, processes it, and sends the result. Workers exit when `next()` returns `None`. The original `tx` is dropped after spawning workers so the channel closes when all workers finish. Results are collected from `rx` into a `Vec`. The `Arc<F>` wraps the processing function for sharing across workers.

## OCaml Approach

OCaml's fan-out uses `List.map (fun item -> Domain.spawn (fun () -> process item)) items` in OCaml 5.x, then `List.map Domain.join handles` for fan-in. `Domainslib.Task.parallel_for` is the idiomatic OCaml 5.x approach. In OCaml 4.x, `Thread.create` with channels provides the same pattern. OCaml's list map naturally expresses the fan-out structure.

## Key Differences

1. **Work distribution**: Rust's fan-out uses `Arc<Mutex<Iterator>>` (work stealing); OCaml typically pre-distributes work (static partitioning).
2. **Result order**: Fan-in with `mpsc::channel` collects results in completion order (non-deterministic); OCaml's `List.map Domain.join` collects in spawn order.
3. **Dynamic load**: Work stealing (`Arc<Mutex<Iterator>>`) handles variable-cost items better than static partitioning.
4. **Rayon analogy**: `rayon::par_iter().map(f).collect()` is fan-out/fan-in in one operation; this manual implementation shows the underlying mechanism.

## Exercises

1. **Order-preserving fan-in**: Modify `fan_map` to return results in the same order as the input. Hint: include an index with each work item and sort results by index after fan-in.
2. **Dynamic fan-out**: Instead of fixed N workers, implement adaptive fan-out that spawns new workers when the queue is more than 50% full and lets workers exit when the queue is empty. Track the peak worker count.
3. **Pipeline fan-out**: Integrate fan-out into the pipeline from example 462. Replace a slow single stage with a fan-out stage that has N parallel workers, while the surrounding serial stages remain unchanged.
