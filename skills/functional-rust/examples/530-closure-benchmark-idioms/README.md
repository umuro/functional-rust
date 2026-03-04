# 530: Closures in Benchmarking

**Difficulty:** 2  **Level:** Beginner-Intermediate

Use closures to wrap workloads for repeatable measurement — and `black_box` to keep the compiler honest.

## The Problem This Solves

Writing a benchmark is easy. Writing a *correct* benchmark is surprisingly hard. The most common mistake: timing code that the compiler has silently deleted. LLVM sees that your computation's result is never used, concludes the entire computation is dead code, and removes it. Your benchmark reports sub-nanosecond timings for "doing nothing" — and you publish those numbers.

The second most common mistake: running the workload once and reporting that single measurement. One sample has no statistical validity. Cache effects, OS scheduling, CPU throttling, and branch predictor state all introduce variance that a single measurement cannot distinguish from signal.

Closures solve both problems elegantly. A closure wraps the workload into a repeatable unit the harness can call N times. `std::hint::black_box` wraps the inputs and outputs, creating an opaque barrier the compiler cannot see through — it cannot prove the result is unused, so it cannot delete the computation. Combined with warmup iterations, you get a statistically sound measurement with minimal boilerplate.

## The Intuition

A benchmark closure is a promise: "here is a piece of work; run it as many times as you need." The harness holds the closure, calls it repeatedly with a timer around the loop, and reports statistics. The closure captures setup data (input arrays, parameters) so each iteration gets fresh-looking inputs without reconstructing them from scratch.

`black_box(x)` is a fence that says "x is observable." The compiler treats it as if x was sent to some external system it can't inspect. Wrapping both inputs and the result ensures the computation isn't optimised away: `black_box(f(black_box(input)))`.

## How It Works in Rust

```rust
use std::hint::black_box;
use std::time::Instant;

fn bench<T, F: FnMut() -> T>(name: &str, iters: usize, mut f: F) {
    // Warmup: prime instruction cache and branch predictor
    for _ in 0..iters / 10 {
        black_box(f());
    }

    let start = Instant::now();
    for _ in 0..iters {
        black_box(f());  // result is "observed" — compiler keeps the computation
    }
    let per_iter = start.elapsed() / iters as u32;
    println!("{name}: {per_iter:?}/iter");
}

// Usage: closure captures the input data, called once per iteration.
let data: Vec<i64> = (0..1000).collect();
bench("sum", 100_000, || {
    black_box(data.iter().sum::<i64>())
});

// Parameterised bench: test the same algorithm at different input sizes.
fn bench_scaling<F: Fn(usize) -> i64>(name: &str, sizes: &[usize], f: F) {
    for &n in sizes {
        bench(&format!("{name} n={n}"), 10_000, || f(black_box(n)));
    }
}
```

For production benchmarking, use `criterion` (statistical analysis, HTML reports, regression detection) or `divan` (attribute macros, lower boilerplate). Both use the same `black_box` + warmup + many-iterations model internally.

## What This Unlocks

- **Correct measurements**: `black_box` on inputs and outputs ensures the benchmark actually executes the code you intend to measure.
- **Composable benchmark suites**: A bench function that accepts `FnMut() -> T` works for any workload — sort, hash, parse, compute. One harness, many benchmarks.
- **Scalability analysis**: Closure factories for different input sizes let you plot complexity curves — does your algorithm scale as `O(n)` or `O(n log n)`?

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Benchmark closure | `fun () -> computation` | `|| { black_box(computation()) }` |
| Prevent optimisation | `Sys.opaque_identity` | `std::hint::black_box` |
| Repeated execution | Manual `for` loop | Framework calls `iter(|| ...)` |
| Setup data in closure | Closure captures | `move ||` captures owned data |
| Parameterised bench | Multiple separate functions | Closure factory per parameter |
| Production framework | `Core_bench` | `criterion`, `divan` |
