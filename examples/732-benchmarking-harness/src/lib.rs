/// 732: Benchmarking Harness — Criterion-style, std-only
use std::hint::black_box;
use std::time::{Duration, Instant};

// ── Core Harness ──────────────────────────────────────────────────────────────

struct BenchResult {
    label: &'static str,
    mean: Duration,
    min: Duration,
    max: Duration,
    stddev_ns: f64,
    iters: u64,
}

impl BenchResult {
    fn print(&self) {
        println!(
            "{:40} mean={:>10.2?} min={:>10.2?} max={:>10.2?} σ={:.0}ns  (n={})",
            self.label, self.mean, self.min, self.max, self.stddev_ns, self.iters,
        );
    }
}

fn bench<F, R>(label: &'static str, warmup: u64, iters: u64, mut f: F) -> BenchResult
where
    F: FnMut() -> R,
{
    // Warmup — fill CPU caches, allow CPU to ramp up frequency
    for _ in 0..warmup {
        black_box(f());
    }

    let mut samples = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let t0 = Instant::now();
        let result = f();
        let elapsed = t0.elapsed();
        black_box(result); // prevent dead-code elimination
        samples.push(elapsed);
    }

    let total_ns: u128 = samples.iter().map(|d| d.as_nanos()).sum();
    let mean_ns = total_ns / iters as u128;
    let mean = Duration::from_nanos(mean_ns as u64);

    let min = *samples.iter().min().unwrap();
    let max = *samples.iter().max().unwrap();

    let variance_ns: f64 = samples
        .iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - mean_ns as f64;
            diff * diff
        })
        .sum::<f64>()
        / iters as f64;

    BenchResult {
        label,
        mean,
        min,
        max,
        stddev_ns: variance_ns.sqrt(),
        iters,
    }
}

// ── Functions to Benchmark ────────────────────────────────────────────────────

fn sum_naive(n: u64) -> u64 {
    (0..n).sum()
}

fn sum_formula(n: u64) -> u64 {
    n * (n - 1) / 2
}

fn string_push(n: usize) -> String {
    let mut s = String::with_capacity(n);
    for _ in 0..n {
        s.push('x');
    }
    s
}

fn vec_collect(n: usize) -> Vec<u64> {
    (0..n as u64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_runs_warmup_and_iters() {
        let mut call_count = 0u64;
        let result = bench("test", 5, 10, || {
            call_count += 1;
            call_count
        });
        // warmup(5) + iters(10) = 15 calls
        assert_eq!(call_count, 15);
        assert_eq!(result.iters, 10);
    }

    #[test]
    fn bench_min_le_mean_le_max() {
        let r = bench("sleep_0ns", 2, 20, || sum_naive(black_box(100)));
        assert!(r.min <= r.mean);
        assert!(r.mean <= r.max);
    }

    #[test]
    fn sum_naive_correct() {
        assert_eq!(sum_naive(5), 10); // 0+1+2+3+4
        assert_eq!(sum_naive(0), 0);
    }

    #[test]
    fn sum_formula_matches_naive() {
        for n in 1..=20u64 {
            assert_eq!(sum_naive(n), sum_formula(n), "n={}", n);
        }
    }
}
