📖 **[View on hightechmind.io →](https://hightechmind.io/rust/846-monte-carlo-pattern)**

---

# 846: Monte Carlo Methods — π Estimation and Sampling

**Difficulty:** 3  **Level:** Intermediate

Use random sampling to approximate integrals and probabilities — the technique behind physics simulations, financial models, and machine learning.

## The Problem This Solves

Many integrals have no closed form. Numerical quadrature (grid-based integration) works in 1-3 dimensions but becomes exponentially expensive in higher dimensions — the "curse of dimensionality." Monte Carlo integration sidesteps this: sample random points, evaluate the function, average. The error is O(1/√n) regardless of dimension.

This same idea underlies neural network training (minibatch SGD is Monte Carlo gradient estimation), Bayesian inference (MCMC), option pricing (Black-Scholes simulation), and physical simulations (radiosity, ray tracing). The technique is indispensable for any computation where exact enumeration is intractable.

The π estimation example is the cleanest illustration: a unit square contains a quarter-circle of radius 1. Uniformly sample (x, y) pairs; the fraction satisfying x²+y²≤1 converges to π/4.

## The Intuition

Throw darts at a square board with a circle drawn on it. Count how many land inside the circle. That ratio is π/4. Throw more darts → better estimate. The law of large numbers guarantees convergence; the central limit theorem tells you the error: ±1.96/√n at 95% confidence.

Monte Carlo integration generalizes this: to integrate f(x) over [a,b], pick random x values in [a,b] and average f(x). The average converges to the integral divided by (b-a).

Acceptance-rejection sampling goes further: to sample from a non-uniform distribution, sample uniformly and reject points with probability proportional to how unlikely they are under the target distribution.

## How It Works in Rust

1. **LCG PRNG** — a simple linear congruential generator gives reproducible pseudorandom `f64`s in [0,1) with no dependencies. Constants are Knuth's recommended LCG parameters.
2. **π estimation** — `(0..n).filter(|_| x²+y² ≤ 1.0).count()` with random `x`, `y`. Ratio × 4 = estimate.
3. **`mc_integrate`** — sample `n` random `x` in `[a,b]`, sum `f(x)`, multiply by `(b-a)/n`. Works for any `Fn(f64) -> f64`.
4. **Acceptance-rejection** — loop until `n` samples accepted. For `sin(x)` on `[0,π]`, accept `x` if `uniform() < sin(x)`.

```rust
// π estimation — core loop
let inside = (0..n)
    .filter(|_| { let (x, y) = (rng.next_f64(), rng.next_f64()); x*x + y*y <= 1.0 })
    .count();
4.0 * inside as f64 / n as f64
```

## What This Unlocks

- **Numerical integration without calculus** — approximate any integral with three lines of code.
- **Dimension-independent error** — O(1/√n) holds in 2D, 100D, or 10,000D; deterministic grids cannot match this.
- **Foundation for probabilistic algorithms** — MCMC, particle filters, and Monte Carlo tree search all build on this primitive.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| PRNG | `Random.float 1.0` (stdlib) | LCG / xorshift (no deps needed) |
| Loop accumulate | `for` + mutable `ref` | `(0..n).filter().count()` |
| Convergence rate | O(1/√n) | Same — math doesn't change |
| Higher-order integrate | `let mc f a b n` | `fn mc_integrate(f: impl Fn(f64) -> f64, ...)` |
| Acceptance-rejection | `while count < n` | Same imperative pattern |
