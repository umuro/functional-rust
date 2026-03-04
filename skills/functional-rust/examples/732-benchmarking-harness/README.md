# 732: Benchmarking Harness

**Difficulty:** 3  **Level:** Advanced

Build a Criterion-style micro-benchmark harness from `std` — warm up, iterate, `black_box`, and report statistics.

## The Problem This Solves

Running a function once and timing it tells you almost nothing useful. Your result is contaminated by instruction cache cold starts, branch predictor misses, CPU frequency scaling, OS scheduling jitter, and dynamic compiler optimisations. A single measurement is noise, not signal.

A proper benchmark harness addresses all of these. Warmup runs prime the instruction cache and branch predictor. Many iterations amortise scheduling jitter. `std::hint::black_box` prevents the compiler from optimising away the computation it's supposed to measure — without it, LLVM may realise the result is never used and delete the entire benchmark body. Statistical reporting (mean, standard deviation, min/max) gives you a signal-to-noise picture: a low standard deviation means consistent results you can trust; a high one means something else is interfering.

Understanding how benchmarking frameworks work internally is essential before reaching for `criterion` or `divan`. You need to know why `black_box` is non-optional, why warmup matters, and what the reported numbers actually mean.

## The Intuition

Measuring performance is like weighing yourself: you don't just step on the scale once and declare the number accurate. You step on multiple times, discard the outliers, and average the rest. The scale (timer) has noise. Your weight (computation) has natural variation. `black_box` is making sure you're actually on the scale — not that the scale is printing a cached number from yesterday.

The `Instant::now()` / `elapsed()` pair is `std`'s monotonic wall-clock timer. Wrapping the measured function in `black_box` ensures its inputs and outputs are treated as opaque — the compiler cannot eliminate or constant-fold through the measurement.

## How It Works in Rust

```rust
use std::hint::black_box;
use std::time::{Duration, Instant};

fn bench<T, F: FnMut() -> T>(label: &str, iters: u64, mut f: F) -> Duration {
    // Warmup: prime caches and branch predictor
    for _ in 0..iters / 10 {
        black_box(f());
    }

    let start = Instant::now();
    for _ in 0..iters {
        black_box(f());  // black_box: result treated as observable
    }
    let total = start.elapsed();

    let mean = total / iters as u32;
    println!("{label}: mean={mean:?} (n={iters})");
    total
}

// Usage:
bench("sum_squares(1000)", 10_000, || {
    black_box(sum_squares(black_box(1000u64)))
});
```

Standard deviation over N samples: collect each iteration's duration into a `Vec<f64>` of nanoseconds, compute `mean`, then `sqrt(sum((x - mean)^2) / n)`. High `σ` relative to mean → noisy environment or pathological input dependency.

## What This Unlocks

- **Trustworthy performance numbers**: Warmup + multiple iterations + `black_box` = numbers you can actually compare across code changes.
- **Understanding `criterion`/`divan`**: These frameworks follow the same structure — once you've built the primitives, you understand what the libraries are doing and why.
- **Catching regressions**: A baseline benchmark run committed to the repo lets you detect performance regressions in CI before they reach production.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Wall-clock timer | `Unix.gettimeofday` | `std::time::Instant::now()` |
| Prevent dead-code elimination | `Sys.opaque_identity` | `std::hint::black_box` |
| Benchmark framework | `Core_bench` | `criterion`, `divan` |
| Warmup phase | Manual | Part of harness design |
| Statistical reporting | `Core_bench` columns | Custom mean/stddev from raw durations |
| Closure as workload | `fun () -> expr` | `|| { black_box(expr) }` |
