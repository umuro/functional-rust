📖 **[View on hightechmind.io →](https://hightechmind.io/rust/846-monte-carlo-pattern)**

---

# Monte Carlo Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Some problems are too complex for exact algorithms but can be approximated by random sampling. Monte Carlo methods estimate quantities by sampling: estimate pi by generating random points in a unit square and counting those inside the unit circle; estimate integrals by averaging function values at random points; approximate Nash equilibria in game theory; simulate financial option pricing. Error decreases as O(1/sqrt(n)) — independent of dimensionality, which is the key advantage over deterministic quadrature for high-dimensional integration. Monte Carlo underpins financial risk models, particle physics simulation, and probabilistic algorithms.

## Learning Outcomes

- Implement pi estimation: sample (x,y) uniformly in [-1,1]^2, count points where x^2+y^2 < 1
- Understand O(1/sqrt(n)) convergence: 4x more samples gives 2x more accuracy
- Implement Monte Carlo integration: estimate `integral(f, a, b)` by averaging f at random points
- Apply to high-dimensional integration where deterministic methods are exponentially expensive
- Recognize the importance of a good random number generator (LCG vs PCG vs Mersenne Twister)

## Rust Application

```rust
pub fn estimate_pi(n: u64, rng: &mut impl RngCore) -> f64 {
    let inside = (0..n).filter(|_| {
        let x: f64 = rng.gen_range(-1.0..=1.0);
        let y: f64 = rng.gen_range(-1.0..=1.0);
        x * x + y * y <= 1.0
    }).count();
    4.0 * inside as f64 / n as f64
}
```

The `impl RngCore` parameter makes the function testable with a deterministic seeded RNG. The `filter` + `count` pattern is idiomatic Rust for conditional counting. Using `f64` for the coordinates gives sufficient precision for the statistical error that dominates at typical sample sizes. Parallelizing with Rayon (`par_iter().filter().count()`) gives near-linear speedup since samples are independent. The `rand` crate's `gen_range` handles the uniform sampling cleanly.

## OCaml Approach

OCaml's Monte Carlo uses `Random.float 2.0 -. 1.0` for uniform samples. The pi estimation is: `let count = ref 0 in for _ = 1 to n do let x = ... and y = ... in if x*.x +. y*.y <= 1.0 then incr count done; 4.0 *. float !count /. float n`. OCaml's `Random.self_init ()` seeds from system entropy. The `Seq` module creates an infinite sample sequence for lazy Monte Carlo. OCaml's `Float.sqrt` and `.*.` operators handle the computation.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| RNG | `rand::RngCore` trait | `Random` module (LCG-based) |
| Testability | Seeded `SmallRng::seed_from_u64` | `Random.init seed` |
| Parallelism | `rayon::par_iter()` | `Domain.spawn` (OCaml 5.0) |
| Sample counting | `filter().count()` | Mutable counter in loop |
| Convergence | O(1/sqrt n) | Same |
| Higher dimensions | Same complexity advantage | Same |

## Exercises

1. Implement Monte Carlo integration for a 1D function and compare with the exact analytical result.
2. Extend to 10D integration of `x1^2 + ... + x10^2 <= 1` to demonstrate the dimensionality advantage.
3. Use stratified sampling (divide the domain into strata, sample uniformly within each) and compare convergence with naive Monte Carlo.
4. Implement parallel Monte Carlo using Rayon with independent per-thread RNGs and measure linear speedup.
5. Estimate the variance of your pi estimator as a function of n and verify it matches the theoretical O(1/n) variance.
