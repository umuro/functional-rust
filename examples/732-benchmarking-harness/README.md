📖 **[View on hightechmind.io →](https://hightechmind.io/rust/732-benchmarking-harness)**

---

# 732-benchmarking-harness — Benchmarking Harness
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Micro-benchmarking is surprisingly hard to do correctly. The compiler may eliminate "dead" computations, the CPU may boost frequency during warmup, and a single outlier can skew the mean. Production benchmark frameworks like Criterion address these problems with warmup phases, statistical analysis, and outlier rejection. This example builds a Criterion-inspired harness using only `std`, demonstrating the core techniques that make benchmarks trustworthy.

## Learning Outcomes

- Use `std::hint::black_box` to prevent dead-code elimination of benchmark subjects
- Implement a warmup phase to stabilize CPU frequency and fill caches before measuring
- Collect per-iteration `Duration` samples and compute mean, min, max, and standard deviation
- Understand why the standard deviation matters more than the mean for latency-sensitive code
- Structure benchmark results in a `BenchResult` struct for comparison across runs

## Rust Application

The `bench` function takes a `label`, `warmup` iteration count, `iters` count, and a `FnMut() -> R` closure. It runs the warmup silently, then measures each iteration with `Instant::now()` / `elapsed()`. Each result is passed through `black_box` to prevent the optimizer from removing it. The `BenchResult` struct stores mean, min, max, and `stddev_ns` and provides a `print` method with aligned columns. A `BenchSuite` groups multiple benchmarks and prints a comparison table.

## OCaml Approach

OCaml's standard library has no built-in benchmarking framework. The `benchmark` opam package and Jane Street's `core_bench` library fill this role. `core_bench` uses a similar warmup + sample approach with GC-pause awareness: it forces a minor GC collection before each measurement to reduce noise from accumulated garbage. OCaml's `Sys.time` and `Unix.gettimeofday` are the primitives; `Mtime_clock` provides monotonic wall-clock time similar to `Instant`.

## Key Differences

1. **Dead-code prevention**: Rust has `std::hint::black_box`; OCaml's `Sys.opaque_identity` serves the same purpose in `core_bench`.
2. **GC noise**: OCaml benchmarks must account for GC pauses; Rust has no GC, so samples are more consistent but cache warm-up still matters.
3. **Closures**: Both languages pass closures to the harness; Rust closures capture by reference or move with explicit annotation, while OCaml closures always capture by reference.
4. **Ecosystem**: Rust has `criterion` (statistical, HTML reports) and `divan`; OCaml has `core_bench` and `bechamel`.

## Exercises

1. Add a `p99` latency field to `BenchResult` by sorting samples and indexing at `0.99 * iters`.
2. Implement a `compare` function that takes two `BenchResult` values and prints the speedup ratio and whether the difference is statistically significant (|Δmean| > 2σ).
3. Extend the harness to detect and discard outliers (samples more than 3 standard deviations from the mean) and recompute statistics on the cleaned dataset.
