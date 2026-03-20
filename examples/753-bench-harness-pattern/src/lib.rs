#![allow(clippy::all)]
//! # Benchmark Harness Pattern
//!
//! Measuring hot functions with percentiles (std-only).

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Statistics from a benchmark run
#[derive(Debug)]
pub struct Stats {
    pub mean: Duration,
    pub min: Duration,
    pub p50: Duration,
    pub p90: Duration,
    pub p99: Duration,
    pub max: Duration,
}

/// Compute statistics from a vector of samples
pub fn compute_stats(mut samples: Vec<Duration>) -> Stats {
    assert!(!samples.is_empty());
    samples.sort_unstable();
    let n = samples.len();

    let sum: Duration = samples.iter().sum();
    let mean = sum / n as u32;

    Stats {
        mean,
        min: samples[0],
        p50: samples[n / 2],
        p90: samples[n * 90 / 100],
        p99: samples[n * 99 / 100],
        max: samples[n - 1],
    }
}

/// Run a benchmark
pub fn bench<F, R>(name: &str, iterations: usize, warmup: usize, mut f: F) -> Stats
where
    F: FnMut() -> R,
{
    // Warmup
    for _ in 0..warmup {
        black_box(f());
    }

    // Measure
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        black_box(f());
        samples.push(start.elapsed());
    }

    let stats = compute_stats(samples);
    println!(
        "[{}] mean={:?} min={:?} p50={:?} p90={:?} p99={:?} max={:?}",
        name, stats.mean, stats.min, stats.p50, stats.p90, stats.p99, stats.max
    );
    stats
}

/// Format duration in human-readable form
pub fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2}µs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

// Functions to benchmark

/// Fibonacci recursive (slow)
pub fn fib_recursive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

/// Fibonacci iterative (fast)
pub fn fib_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// Sum a slice
pub fn sum_slice(data: &[i64]) -> i64 {
    data.iter().sum()
}

/// Sum using fold
pub fn sum_fold(data: &[i64]) -> i64 {
    data.iter().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_recursive() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(10), 55);
    }

    #[test]
    fn test_fib_iterative() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(10), 55);
        assert_eq!(fib_iterative(20), 6765);
    }

    #[test]
    fn test_fib_equivalence() {
        for n in 0..20 {
            assert_eq!(fib_recursive(n), fib_iterative(n));
        }
    }

    #[test]
    fn test_compute_stats() {
        let samples: Vec<Duration> = (1..=100).map(|i| Duration::from_nanos(i * 100)).collect();
        let stats = compute_stats(samples);
        assert_eq!(stats.min, Duration::from_nanos(100));
        assert_eq!(stats.max, Duration::from_nanos(10000));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_nanos(500)), "500ns");
        assert_eq!(format_duration(Duration::from_micros(100)), "100.00µs");
        assert_eq!(format_duration(Duration::from_millis(50)), "50.00ms");
        assert_eq!(format_duration(Duration::from_secs(2)), "2.00s");
    }

    #[test]
    fn test_sum_functions() {
        let data: Vec<i64> = (1..=100).collect();
        assert_eq!(sum_slice(&data), 5050);
        assert_eq!(sum_fold(&data), 5050);
    }

    #[test]
    fn test_bench_runs() {
        let stats = bench("test", 10, 2, || fib_iterative(10));
        assert!(stats.mean > Duration::ZERO);
    }
}
