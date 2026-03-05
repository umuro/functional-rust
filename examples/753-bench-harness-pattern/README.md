# 753: Benchmark Harness: Measuring Hot Functions

**Difficulty:** 3  **Level:** Advanced

Measure function performance with warmup, per-iteration timing, and percentile statistics — the pattern behind `cargo bench` and Criterion.

## The Problem This Solves

"My code is slow" is not actionable. "The p99 latency of `string_alloc` is 3× higher than `string_prealloc`" is. Performance measurement requires more care than functional testing: CPU frequency scaling, CPU cache warmup, branch predictor state, and OS scheduling all introduce noise. A naive timing loop produces misleading results. The standard solution is warmup iterations (let the CPU reach peak frequency and fill caches), many measurement iterations (statistical stability), and percentile reporting (p99 catches tail latency that mean hides).

This pattern is exactly what Criterion (the standard Rust benchmarking crate) implements. Understanding the raw pattern — even without Criterion — teaches you what `#[bench]` and `criterion::Criterion::bench_function()` are doing internally. For quick comparisons between two implementations, a hand-rolled harness is often sufficient.

`std::hint::black_box` is critical: it prevents the compiler from optimizing away the function call entirely when the result isn't used. Without it, the benchmark measures nothing.

## The Intuition

Run the function `warmup` times to stabilize CPU state. Then run it `iters` times, recording the elapsed time for each call. Sort the timings and compute percentiles: min (best case), p50 (median), p90, p99, max (worst case). The mean can be misleading if there are occasional slow outliers; p99 reveals tail latency that affects real users in production systems.

## How It Works in Rust

```rust
use std::hint::black_box;   // prevents dead-code elimination
use std::time::Instant;

fn bench<F, R>(name: &str, warmup: usize, iters: usize, mut f: F) -> Stats
where F: FnMut() -> R
{
    // Warmup: allow CPU frequency scaling and cache filling
    for _ in 0..warmup { black_box(f()); }

    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        black_box(f());    // black_box prevents optimizing f() away
        samples.push(t0.elapsed());
    }
    compute_stats(samples)
}

fn compute_stats(mut samples: Vec<Duration>) -> Stats {
    samples.sort_unstable();
    let n = samples.len();
    Stats {
        mean: samples.iter().sum::<Duration>() / n as u32,
        min:  samples[0],
        p50:  samples[n * 50 / 100],
        p90:  samples[n * 90 / 100],
        p99:  samples[n * 99 / 100],
        max:  *samples.last().unwrap(),
    }
}

// Compare two implementations
bench("sum_loop(1000)",    50, 5000, || sum_loop(black_box(1000)));
bench("sum_formula(1000)", 50, 5000, || sum_formula(black_box(1000)));
```

For real-world benchmarking, use the `criterion` crate: it adds statistical significance testing, warm cache management, and HTML reports. The `cargo bench` command runs functions marked `#[bench]` in nightly, but Criterion works on stable.

## What This Unlocks

- **`black_box` discipline** — always wrap benchmark inputs in `black_box()` to prevent the compiler from constant-folding or dead-code-eliminating the code under test; without this, you're measuring zero.
- **Percentile thinking** — mean latency is for dashboards; p99 is for SLAs; max is for debugging worst cases; understanding which to look at for which question is fundamental to performance engineering.
- **Before/after comparison methodology** — always run both implementations in the same harness, on the same machine, with the same warmup, to get meaningful comparisons; micro-benchmark results are only valid relative to each other.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Prevent optimization | `Sys.opaque_identity` | `std::hint::black_box` — stable since Rust 1.66 |
| High-res timing | `Unix.gettimeofday` (µs) | `std::time::Instant` — nanosecond resolution |
| Benchmark framework | `core_bench` (Jane Street) | `criterion` crate (de facto standard) |
| Percentile computation | Manual sort + index | `sort_unstable()` then indexed access — same pattern |
