# 448: Rayon Parallel Iterators

**Difficulty:** 3  **Level:** Intermediate

`.par_iter()` — swap one word and your loop runs on all CPU cores.

## The Problem This Solves

Sequential iterators are elegant but single-threaded. When you're processing millions of items — image pixels, log lines, simulation steps — you're leaving N−1 cores idle. Manually splitting work across threads, joining them, and merging results is tedious and error-prone.

Rayon's `par_iter()` parallelises iterator pipelines automatically. It uses a global work-stealing thread pool tuned to your hardware. You write the same functional chain you'd write for serial code; Rayon decides how to split and schedule it.

Because ownership and `Send`/`Sync` bounds are enforced at compile time, data races are impossible. The compiler rejects parallel code that would be unsafe.

## The Intuition

You have a conveyor belt (your iterator) processing items one at a time. With `.par_iter()` you're replacing it with N parallel conveyor belts, each handling a slice of items, feeding results into a single final collection. You don't manage the belts — you just describe what to do with each item.

## How It Works in Rust

1. **Switch to parallel** — call `.par_iter()` instead of `.iter()` (requires `rayon::prelude::*`).
2. **Same API** — `.map()`, `.filter()`, `.flat_map()`, `.fold()`, `.reduce()` all work identically.
3. **Collect results** — `.collect::<Vec<_>>()` merges partial results from all threads.
4. **Automatic chunking** — Rayon splits the input adaptively using its work-stealing scheduler. You never size chunks manually.

```rust
use rayon::prelude::*;

let sum: i64 = (0..1_000_000_i64)
    .into_par_iter()
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .sum();
```

5. **Custom thread pool** — `rayon::ThreadPoolBuilder::new().num_threads(4).build_global()` if defaults don't suit.

## What This Unlocks

- **Zero-boilerplate parallelism** — the migration from serial to parallel is often a one-character change.
- **Composability** — parallel and sequential iterators chain together; switch back with `.collect()` then `.iter()`.
- **Safety by construction** — if your closures aren't `Send`, Rayon won't compile. The type system enforces thread safety.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parallel iteration | `Parmap` / `Domainslib.Task.parallel_for` | `rayon::par_iter()` |
| Thread pool | `Domainslib` pool, explicit | Implicit global pool |
| Safety | Runtime checks | Compile-time `Send`/`Sync` bounds |
| Chunk sizing | Manual or library heuristic | Adaptive work-stealing |
