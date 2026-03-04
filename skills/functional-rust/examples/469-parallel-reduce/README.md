# 469: Parallel Fold/Reduce

**Difficulty:** 3  **Level:** Intermediate

Split data across threads, reduce each chunk independently, then combine — the MapReduce pattern in miniature.

## The Problem This Solves

A sequential fold over a million-element list visits every item one after another. No matter how fast your closure is, you're using one core. If the reduction is expensive (hashing, parsing, heavy math), you're leaving 90% of your CPU idle.

Parallel reduce exploits the *associativity* of the combining operation: if `f(a, f(b, c)) == f(f(a, b), c)`, then it doesn't matter what order you combine partial results. Split into N chunks, reduce each in parallel, combine the partial results at the end.

Sum, product, maximum, minimum, string concatenation (with careful ordering), and histogram counting are all associative. If your operation qualifies, parallelising is straightforward.

## The Intuition

Counting votes in an election. You don't have one person read every ballot. You divide the ballots into piles, give each pile to a counter, and when all counters are done, add the totals. The final sum is identical regardless of how you divided the piles — because addition is associative.

## How It Works in Rust

**With Rayon (recommended)**:
```rust
use rayon::prelude::*;

let sum: i64 = data.par_iter()
    .map(|x| expensive_hash(x))
    .reduce(|| 0, |a, b| a + b);
```

**Manual parallel reduce** (for understanding the pattern):
1. **Split** — divide the slice into `num_cpus` chunks.
2. **Spawn threads** — each thread reduces its chunk to a partial result.
3. **Collect and combine** — join all threads, fold the partial results.
   ```rust
   let chunks = data.chunks(chunk_size);
   let handles: Vec<_> = chunks.map(|chunk| {
       let chunk = chunk.to_vec();
       thread::spawn(move || chunk.iter().sum::<i64>())
   }).collect();
   let total: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
   ```

**Key constraint**: the combining function must be associative. Rayon's `.reduce(identity, f)` takes an identity element (e.g. `0` for sum, `1` for product) and an associative `f`.

## What This Unlocks

- **CPU-bound throughput** — near-linear speedup for expensive per-element work on multi-core hardware.
- **MapReduce as a local pattern** — the same structure scales from a laptop to a distributed cluster.
- **Functional composition** — chaining `.map()` + `.reduce()` keeps the parallel logic declarative.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parallel reduce | `Domainslib.Task.parallel_for` + shared ref | Rayon `.par_iter().reduce()` |
| Safety | Mutable shared state, careful coding | Ownership prevents races |
| Identity element | Explicit | Explicit (Rayon requires it) |
| Chunk management | Manual or library | Rayon adaptive work-stealing |
