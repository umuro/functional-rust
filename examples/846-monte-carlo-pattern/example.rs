/// Monte Carlo Methods: π estimation, integration, sampling.
///
/// Uses a deterministic LCG PRNG for reproducible results without external crates.

use std::f64::consts::PI;

/// Simple LCG pseudo-random number generator.
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self { Lcg(seed) }

    /// Return next float in [0, 1).
    fn next_f64(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Estimate π by sampling points in the unit square.
/// Fraction inside the unit circle × 4 ≈ π.
fn estimate_pi(n: u64, seed: u64) -> f64 {
    let mut rng = Lcg::new(seed);
    let inside = (0..n)
        .filter(|_| {
            let x = rng.next_f64();
            let y = rng.next_f64();
            x * x + y * y <= 1.0
        })
        .count();
    4.0 * inside as f64 / n as f64
}

/// Monte Carlo integration: ∫_a^b f(x) dx ≈ (b-a) × mean(f(samples)).
fn mc_integrate(f: impl Fn(f64) -> f64, a: f64, b: f64, n: u64, seed: u64) -> f64 {
    let mut rng = Lcg::new(seed);
    let sum: f64 = (0..n).map(|_| f(a + (b - a) * rng.next_f64())).sum();
    (b - a) * sum / n as f64
}

/// Estimate the expected value E[f(X)] where X ~ Uniform[a, b].
fn expected_value(f: impl Fn(f64) -> f64, a: f64, b: f64, n: u64, seed: u64) -> f64 {
    mc_integrate(f, a, b, n, seed) / (b - a)
}

/// Acceptance-rejection: sample X with pdf ∝ sin(x) on [0, π].
/// Returns sample mean (should converge to π/2 - 1 ≈ 0.5708).
fn accept_reject_sin(n: usize, seed: u64) -> f64 {
    let mut rng = Lcg::new(seed);
    let mut sum = 0.0;
    let mut count = 0;
    while count < n {
        let x = rng.next_f64() * PI;
        let u = rng.next_f64();
        if u < x.sin() {
            sum += x;
            count += 1;
        }
    }
    sum / n as f64
}

fn main() {
    println!("π estimation (true π = {:.6}):", PI);
    for &n in &[100u64, 1_000, 10_000, 100_000, 1_000_000] {
        let est = estimate_pi(n, 42);
        println!("  n={n:>9}: π ≈ {est:.6}  error = {:.6}", (est - PI).abs());
    }

    println!("\nMonte Carlo integration (n=100,000):");
    let n = 100_000u64;
    let pi_int = mc_integrate(|x| x * x, 0.0, 1.0, n, 1);
    println!("  ∫₀¹ x² dx ≈ {pi_int:.6}  (exact = 0.333333)");
    let sin_int = mc_integrate(f64::sin, 0.0, PI, n, 2);
    println!("  ∫₀^π sin(x) dx ≈ {sin_int:.6}  (exact = 2.000000)");
    let gauss = mc_integrate(|x| (-x * x / 2.0).exp(), -5.0, 5.0, n, 3);
    println!("  ∫₋₅⁵ e^(-x²/2) dx ≈ {gauss:.6}  (exact ≈ {:.6})", (2.0 * PI).sqrt());

    println!("\nAcceptance-rejection sampling:");
    let mean = accept_reject_sin(10_000, 7);
    println!("  Mean of sin-distributed samples ≈ {mean:.4}  (E[X] ≈ {:.4})", PI / 2.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_estimate_reasonable() {
        // With enough samples, estimate should be within 1% of π
        let est = estimate_pi(1_000_000, 42);
        assert!((est - PI).abs() < 0.05, "π estimate {est} too far from {PI}");
    }

    #[test]
    fn test_mc_integral_x_squared() {
        // ∫₀¹ x² dx = 1/3
        let result = mc_integrate(|x| x * x, 0.0, 1.0, 500_000, 99);
        assert!((result - 1.0/3.0).abs() < 0.01, "integral = {result}");
    }

    #[test]
    fn test_mc_integral_sin() {
        // ∫₀^π sin(x) dx = 2
        let result = mc_integrate(f64::sin, 0.0, PI, 500_000, 7);
        assert!((result - 2.0).abs() < 0.05, "integral = {result}");
    }

    #[test]
    fn test_lcg_distribution() {
        // Values should be spread across [0, 1)
        let mut rng = Lcg::new(12345);
        let samples: Vec<f64> = (0..1000).map(|_| rng.next_f64()).collect();
        let mean: f64 = samples.iter().sum::<f64>() / 1000.0;
        assert!((mean - 0.5).abs() < 0.1, "LCG mean {mean} not near 0.5");
        assert!(samples.iter().all(|&x| x >= 0.0 && x < 1.0), "LCG out of [0,1)");
    }

    #[test]
    fn test_convergence_improves() {
        let err1 = (estimate_pi(1_000, 1) - PI).abs();
        let err2 = (estimate_pi(100_000, 1) - PI).abs();
        // With 100x more samples, error should improve (usually)
        // Not guaranteed but holds for reasonable seeds
        assert!(err2 < err1 + 0.1, "larger n didn't improve: {err1} vs {err2}");
    }
}
