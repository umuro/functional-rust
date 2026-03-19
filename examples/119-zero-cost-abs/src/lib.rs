#![allow(clippy::all)]
//! Example 119: Zero-Cost Abstractions
//!
//! Iterator chains, closures, and newtypes compile to the same machine code
//! as hand-written equivalents — abstraction with no runtime penalty.

// ── Approach 1: Iterator chains ──────────────────────────────────────────────
//
// `.filter().map().sum()` fuses into a single loop at compile time.
// No intermediate `Vec` is ever allocated.

/// Sum of squares of all even numbers in `0..n`.
pub fn sum_even_squares(n: i64) -> i64 {
    (0..n).filter(|x| x % 2 == 0).map(|x| x * x).sum()
}

/// Same computation written as an explicit loop — produces identical assembly.
pub fn sum_even_squares_manual(n: i64) -> i64 {
    let mut acc = 0i64;
    for x in 0..n {
        if x % 2 == 0 {
            acc += x * x;
        }
    }
    acc
}

// ── Approach 2: Closures monomorphised at the call site ───────────────────────
//
// `impl Fn(f64) -> f64` is a zero-sized type; LLVM inlines the closure body
// completely. No heap allocation, no indirect call.

/// Returns a closure that evaluates the polynomial `c₀ + c₁x + c₂x² + …`.
pub fn make_polynomial(coeffs: Vec<f64>) -> impl Fn(f64) -> f64 {
    move |x| {
        coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c * x.powi(i as i32))
            .sum()
    }
}

// ── Approach 3: Newtypes — compile-time safety, zero runtime cost ─────────────
//
// `Meters` and `Seconds` are distinct types that the compiler enforces,
// yet at runtime each is just a bare `f64`.

/// Distance measured in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub f64);

/// Time measured in seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Seconds(pub f64);

/// Speed measured in metres per second (derived from newtypes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetersPerSecond(pub f64);

impl Meters {
    pub fn value(self) -> f64 {
        self.0
    }
}

impl Seconds {
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Compute speed — the type system prevents accidentally passing `Seconds`
/// where `Meters` is expected, at zero runtime cost.
pub fn speed(distance: Meters, time: Seconds) -> MetersPerSecond {
    MetersPerSecond(distance.0 / time.0)
}

// ── Approach 4: Higher-order pipeline — idiomatic functional style ────────────

/// Apply a pipeline of transformations to a slice, returning a collected `Vec`.
/// Each `fn` pointer is monomorphised; the chain is inlined by the optimiser.
pub fn pipeline<T, U, F>(data: &[T], transform: F) -> Vec<U>
where
    T: Copy,
    F: Fn(T) -> U,
{
    data.iter().copied().map(transform).collect()
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_chain_equals_manual_loop() {
        // The two implementations must produce identical results for all n.
        for n in [0, 1, 10, 100, 1000] {
            assert_eq!(
                sum_even_squares(n),
                sum_even_squares_manual(n),
                "mismatch at n={n}"
            );
        }
    }

    #[test]
    fn test_sum_even_squares_known_value() {
        // 0..10 even numbers: 0,2,4,6,8  → squares: 0,4,16,36,64 → sum=120
        assert_eq!(sum_even_squares(10), 120);
    }

    #[test]
    fn test_polynomial_closure() {
        // p(x) = 1 + 2x + 3x²
        // p(0) = 1, p(1) = 6, p(2) = 17
        let poly = make_polynomial(vec![1.0, 2.0, 3.0]);
        assert!((poly(0.0) - 1.0).abs() < 1e-10);
        assert!((poly(1.0) - 6.0).abs() < 1e-10);
        assert!((poly(2.0) - 17.0).abs() < 1e-10);
    }

    #[test]
    fn test_polynomial_constant() {
        // p(x) = 5  (single coefficient)
        let poly = make_polynomial(vec![5.0]);
        assert!((poly(0.0) - 5.0).abs() < 1e-10);
        assert!((poly(99.0) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_newtype_speed() {
        let d = Meters(100.0);
        let t = Seconds(9.58); // Usain Bolt world record
        let s = speed(d, t);
        assert!((s.0 - 100.0 / 9.58).abs() < 1e-10);
    }

    #[test]
    fn test_newtype_zero_cost_value_access() {
        // Newtype wrapper adds no overhead — .value() is an identity function.
        let m = Meters(42.0);
        let s = Seconds(7.0);
        assert_eq!(m.value(), 42.0);
        assert_eq!(s.value(), 7.0);
    }

    #[test]
    fn test_pipeline_transform() {
        let data = [1i32, 2, 3, 4, 5];
        let doubled = pipeline(&data, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_pipeline_empty() {
        let data: [i32; 0] = [];
        let result = pipeline(&data, |x| x + 1);
        assert_eq!(result, Vec::<i32>::new());
    }
}
