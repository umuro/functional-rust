//! Closures in Benchmarking
//!
//! Patterns for measuring performance with closures and black_box.

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Simple benchmark runner: time a closure over N iterations.
pub fn bench<T, F: FnMut() -> T>(name: &str, iterations: usize, mut f: F) -> Duration {
    // Warmup
    for _ in 0..iterations / 10 {
        black_box(f());
    }

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(f());
    }
    let elapsed = start.elapsed();
    let per_iter = elapsed / iterations as u32;
    println!(
        "{}: {:?} per iteration ({} iters)",
        name, per_iter, iterations
    );
    elapsed
}

/// Compare two implementations.
pub fn bench_compare<T, F1, F2>(name1: &str, f1: F1, name2: &str, f2: F2, iterations: usize)
where
    F1: FnMut() -> T,
    F2: FnMut() -> T,
{
    let t1 = bench(name1, iterations, f1);
    let t2 = bench(name2, iterations, f2);

    let ratio = t1.as_nanos() as f64 / t2.as_nanos() as f64;
    println!("Ratio {}/{}: {:.2}x", name1, name2, ratio);
}

/// Prevent value from being optimized away.
pub fn consume<T>(value: T) -> T {
    black_box(value)
}

/// Benchmark with setup and teardown closures.
pub fn bench_with_setup<S, T, Setup, Test, Teardown>(
    name: &str,
    iterations: usize,
    mut setup: Setup,
    mut test: Test,
    mut teardown: Teardown,
) -> Duration
where
    Setup: FnMut() -> S,
    Test: FnMut(S) -> T,
    Teardown: FnMut(T),
{
    // Warmup
    for _ in 0..iterations / 10 {
        let s = setup();
        let t = test(s);
        teardown(t);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let s = black_box(setup());
        let t = black_box(test(s));
        teardown(t);
    }
    let elapsed = start.elapsed();
    println!("{}: {:?} total ({} iters)", name, elapsed, iterations);
    elapsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_basic() {
        let duration = bench("simple_add", 1000, || 1 + 1);
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_consume() {
        let value = consume(42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_bench_with_closure_state() {
        let mut counter = 0;
        let _ = bench("counter", 100, || {
            counter += 1;
            counter
        });
        assert_eq!(counter, 110); // 10 warmup + 100 iterations
    }

    #[test]
    fn test_bench_with_setup() {
        let duration = bench_with_setup(
            "vec_sum",
            100,
            || vec![1, 2, 3, 4, 5],
            |v| v.iter().sum::<i32>(),
            |_| {},
        );
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_black_box_prevents_optimization() {
        // Without black_box, compiler might optimize this away
        let result = black_box(vec![1, 2, 3].iter().sum::<i32>());
        assert_eq!(result, 6);
    }
}
