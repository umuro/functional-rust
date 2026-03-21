📖 **[View on hightechmind.io →](https://hightechmind.io/rust/753-bench-harness-pattern)**

---

# 753-bench-harness-pattern — Benchmark Harness Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Percentile latency matters more than mean latency for user-facing systems. A p99 of 50ms means 1% of requests are slow — unacceptable for interactive applications. Production benchmark frameworks like Criterion report p50, p90, p95, and p99. This example builds a stdlib-only harness that computes full percentile statistics from per-iteration samples, demonstrating the statistical foundation beneath tools like Criterion, Divan, and `hyperfine`.

## Learning Outcomes

- Collect per-iteration `Duration` samples and sort them for percentile computation
- Compute p50, p90, p99 as array index lookups after sorting
- Use `std::hint::black_box` to prevent dead-code elimination of benchmarked functions
- Understand why p99 matters more than mean for user-facing latency
- Structure a reusable `bench(name, iterations, warmup, f)` function

## Rust Application

`compute_stats` takes a `Vec<Duration>`, sorts it, and extracts `min`, `p50`, `p90`, `p99`, `max`, and `mean`. The `bench` function runs warmup iterations (discarded), then measures each real iteration with `Instant::now()`. Results are passed through `black_box` to prevent optimization. The harness is used to compare `bubble_sort` and `stdlib sort` on random data, showing how the harness reveals performance differences across the distribution.

## OCaml Approach

OCaml's `core_bench` library provides `Bench.Test.create` with built-in percentile reporting. `Jane Street` uses it extensively in their trading systems. `bechamel` is an alternative with more statistical sophistication (R² goodness of fit). OCaml's GC adds noise to benchmarks that Rust avoids; `core_bench` accounts for this by measuring GC pressure separately.

## Key Differences

1. **GC noise**: Rust benchmarks are not affected by garbage collection; OCaml benchmarks must account for minor and major GC pauses.
2. **Percentile support**: This example implements percentiles from scratch; Rust's `criterion` provides them via `Criterion::bench_function`. OCaml's `core_bench` provides similar output.
3. **Warmup**: Both harnesses implement warmup phases; Criterion's warmup duration is configurable, while this example uses a fixed iteration count.
4. **Statistical analysis**: Criterion performs regression analysis and outlier detection; this example provides raw percentiles without statistical significance testing.

## Exercises

1. Add an outlier detection step: remove samples more than 3 standard deviations from the mean and recompute statistics on the cleaned dataset.
2. Implement a `compare_stats` function that prints a speedup table: for each percentile, shows the ratio between two `Stats` values and whether the difference is significant.
3. Add throughput reporting: given a `bytes_processed` count per iteration, compute and display MB/s or million-ops/s alongside latency percentiles.
