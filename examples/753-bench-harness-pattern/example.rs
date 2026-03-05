/// 753: Benchmark Harness — measuring hot functions with percentiles

use std::hint::black_box;
use std::time::{Duration, Instant};

// ── Statistics ────────────────────────────────────────────────────────────────

struct Stats {
    mean:  Duration,
    min:   Duration,
    p50:   Duration,
    p90:   Duration,
    p99:   Duration,
    max:   Duration,
}

fn compute_stats(mut samples: Vec<Duration>) -> Stats {
    assert!(!samples.is_empty());
    samples.sort_unstable();
    let n = samples.len();
    let total: Duration = samples.iter().sum();
    Stats {
        mean: total / n as u32,
        min:  samples[0],
        p50:  samples[n * 50 / 100],
        p90:  samples[n * 90 / 100],
        p99:  samples[n * 99 / 100],
        max:  *samples.last().unwrap(),
    }
}

// ── Harness ────────────────────────────────────────────────────────────────────

fn bench<F, R>(name: &str, warmup: usize, iters: usize, mut f: F) -> Stats
where F: FnMut() -> R
{
    // Warmup: fill CPU caches, let frequency scale up
    for _ in 0..warmup { black_box(f()); }

    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        black_box(f());
        samples.push(t0.elapsed());
    }

    let stats = compute_stats(samples);
    print_bench(name, &stats);
    stats
}

fn print_bench(name: &str, s: &Stats) {
    println!(
        "{:<35} mean={:>8.2?} min={:>8.2?} p50={:>8.2?} p90={:>8.2?} p99={:>8.2?} max={:>8.2?}",
        name, s.mean, s.min, s.p50, s.p90, s.p99, s.max
    );
}

// ── Functions to benchmark ────────────────────────────────────────────────────

fn sum_loop(n: u64) -> u64 {
    let mut acc = 0u64;
    for i in 0..=n { acc = acc.wrapping_add(i); }
    acc
}

fn sum_iter(n: u64) -> u64 {
    (0..=n).sum()
}

fn sum_formula(n: u64) -> u64 {
    n.wrapping_mul(n.wrapping_add(1)) / 2
}

fn string_alloc_per_call(n: usize) -> String {
    (0..n).map(|i| format!("{}", i)).collect::<Vec<_>>().join(",")
}

fn string_prealloc(n: usize) -> String {
    let mut s = String::with_capacity(n * 4);
    for i in 0..n {
        if i > 0 { s.push(','); }
        s.push_str(&i.to_string());
    }
    s
}

fn main() {
    println!("{:=<80}", "");
    println!(" Benchmark Harness — hot function comparison");
    println!("{:=<80}", "");
    println!();

    bench("sum_loop(1000)",    50, 5000, || sum_loop(black_box(1000)));
    bench("sum_iter(1000)",    50, 5000, || sum_iter(black_box(1000)));
    bench("sum_formula(1000)", 50, 5000, || sum_formula(black_box(1000)));
    println!();
    bench("string_alloc(50)",    20, 1000, || string_alloc_per_call(black_box(50)));
    bench("string_prealloc(50)", 20, 1000, || string_prealloc(black_box(50)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_sum_functions_agree() {
        for n in [0u64, 1, 10, 100, 1000] {
            let loop_r    = sum_loop(n);
            let iter_r    = sum_iter(n);
            let formula_r = sum_formula(n);
            assert_eq!(loop_r, iter_r,    "loop vs iter at n={}", n);
            assert_eq!(iter_r, formula_r, "iter vs formula at n={}", n);
        }
    }

    #[test]
    fn string_functions_produce_same_output() {
        for n in [0, 1, 5, 20] {
            let alloc = string_alloc_per_call(n);
            let pre   = string_prealloc(n);
            assert_eq!(alloc, pre, "mismatch at n={}", n);
        }
    }

    #[test]
    fn bench_runs_without_panic() {
        bench("test_bench", 5, 20, || sum_formula(black_box(42)));
    }

    #[test]
    fn stats_percentiles_ordered() {
        let stats = compute_stats(
            (0..1000).map(|i| Duration::from_nanos(i)).collect()
        );
        assert!(stats.min <= stats.p50);
        assert!(stats.p50 <= stats.p90);
        assert!(stats.p90 <= stats.p99);
        assert!(stats.p99 <= stats.max);
    }
}
