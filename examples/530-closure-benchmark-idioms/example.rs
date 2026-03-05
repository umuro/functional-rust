//! # 530. Closures in Benchmarking
//! Patterns for measuring performance with closures and black_box.

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Simple benchmark runner: time a closure over N iterations
fn bench<T, F: FnMut() -> T>(name: &str, iterations: usize, mut f: F) -> Duration {
    // Warmup
    for _ in 0..iterations / 10 {
        black_box(f());
    }

    let start = Instant::now();
    for _ in 0..iterations {
        // black_box prevents compiler from optimizing away the call
        black_box(f());
    }
    let elapsed = start.elapsed();
    let per_iter = elapsed / iterations as u32;
    println!("{}: {:?} total, {:?}/iter ({} iters)", name, elapsed, per_iter, iterations);
    elapsed
}

/// Parameterized benchmark: test same code with different input sizes
fn bench_scaling<T, F: Fn(usize) -> T>(name: &str, sizes: &[usize], f: F) {
    println!("\n{} — scaling:", name);
    for &size in sizes {
        let iters = 1000.max(100_000 / size);
        bench(&format!("  n={}", size), iters, || f(black_box(size)));
    }
}

/// Compare two implementations
fn compare<T: std::fmt::Debug, A: Fn() -> T, B: Fn() -> T>(
    name_a: &str, a: A,
    name_b: &str, b: B,
    iterations: usize,
) {
    println!("\nComparing:");
    let t_a = bench(name_a, iterations, &a);
    let t_b = bench(name_b, iterations, &b);

    let ratio = t_a.as_nanos() as f64 / t_b.as_nanos().max(1) as f64;
    let winner = if ratio > 1.0 { name_b } else { name_a };
    println!("  {} is {:.2}x faster", winner, ratio.max(1.0 / ratio));
}

fn main() {
    println!("=== Basic timing ===");

    // Closure captures the workload — black_box prevents dead-code elimination
    bench("sum_1_to_1000", 100_000, || {
        (1..=1000i64).sum::<i64>()
    });

    bench("string_concat_10", 10_000, || {
        (0..10).map(|i| i.to_string()).collect::<String>()
    });

    bench("vec_sort_100", 1_000, || {
        let mut v: Vec<i32> = (0..100).rev().collect();
        v.sort();
        v
    });

    // Scaling analysis
    bench_scaling("vec_sort", &[10, 100, 1000, 10000], |n| {
        let mut v: Vec<i32> = (0..n as i32).rev().collect();
        v.sort();
        v
    });

    // Compare implementations
    compare(
        "naive_sum",   || (0..1000i64).fold(0, |a, b| a + b),
        "iter_sum",    || (0..1000i64).sum::<i64>(),
        50_000,
    );

    // Closure factory for parameterized benchmarks
    let make_bench = |n: usize| move || {
        black_box((0..n).map(|x| x * x).sum::<usize>())
    };

    println!("\n=== Parameterized benches ===");
    for n in [100, 1000, 10000] {
        bench(&format!("sum_of_squares_n={}", n), 10_000, make_bench(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_runs() {
        // Verify bench function doesn't panic
        let elapsed = bench("test", 10, || 42i32);
        assert!(elapsed > Duration::ZERO);
    }

    #[test]
    fn test_black_box_identity() {
        // black_box returns its argument
        let x = black_box(42);
        assert_eq!(x, 42);
    }

    #[test]
    fn test_bench_closure_state() {
        // Closure can accumulate state — each bench call resets
        let mut call_count = 0usize;
        bench("count", 100, || {
            call_count += 1;
            call_count
        });
        // 100 warmup/10 + 100 actual calls
        assert!(call_count > 0);
    }
}
