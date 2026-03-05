📖 **[View on hightechmind.io →](https://hightechmind.io/rust/829-chinese-remainder-theorem)**

---

# 829: Chinese Remainder Theorem

**Difficulty:** 4  **Level:** Advanced

Reconstruct a unique integer from its residues mod several moduli — the mathematical foundation of RSA speedup, arbitrary-precision arithmetic, and range query tricks.

## The Problem This Solves

The Chinese Remainder Theorem answers: given a system of congruences x ≡ r₁ (mod m₁), x ≡ r₂ (mod m₂), …, find x. For pairwise coprime moduli, the solution is unique mod M = m₁ × m₂ × … × mₖ. This is far more than a mathematical curiosity.

In practice: RSA with CRT decomposes decryption into two smaller exponentiations (mod p and mod q separately) and combines them — giving a 4× speedup that every RSA implementation uses. In competitive programming, CRT appears in problems where you need to find when two cyclical events coincide, or reconstruct a value from partial observations. In computer arithmetic, multi-precision multiplication uses CRT (via NTT) to split large polynomial multiplications into smaller modular ones.

The implementation here handles the general case including non-coprime moduli: when `gcd(m₁, m₂) > 1`, a solution exists only if `r₁ ≡ r₂ (mod gcd(m₁, m₂))`, and the result is mod `lcm(m₁, m₂)` rather than mod `m₁ × m₂`.

## The Intuition

For two congruences: x ≡ a₁ (mod m₁) and x ≡ a₂ (mod m₂). We need x = a₁ + m₁ × t such that `a₁ + m₁ × t ≡ a₂ (mod m₂)`. Solving for t: `m₁ × t ≡ (a₂ - a₁) (mod m₂)`. This is a linear congruence — solvable iff `gcd(m₁, m₂) | (a₂ - a₁)`. Find t via Extended Euclidean; then x = a₁ + m₁ × t mod lcm(m₁, m₂). Apply this pairwise to combine all congruences via `fold`.

OCaml uses i128 naturally; Rust has native `i128` and `u128` since 1.26 — no external library needed for CRT arithmetic.

## How It Works in Rust

```rust
// Extended GCD: returns (g, x, y) where a*x + b*y = g
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)  // Standard back-substitution
    }
}

// Combine x ≡ a1 (mod m1) and x ≡ a2 (mod m2)
// Returns Some((remainder, lcm)) or None if incompatible
fn crt_combine(a1: i128, m1: i128, a2: i128, m2: i128) -> Option<(i128, i128)> {
    let (g, p, _) = extended_gcd(m1, m2);
    if (a2 - a1) % g != 0 { return None; }  // No solution if g ∤ (a2 - a1)
    let lcm = m1 / g * m2;
    let m2g = m2 / g;                         // Effective modulus for t
    let diff = ((a2 - a1) / g) % m2g;
    let x = (a1 + m1 * ((diff * p % m2g + m2g) % m2g)) % lcm;
    Some(((x + lcm) % lcm, lcm))             // Normalize to [0, lcm)
}

// Solve a full system via pairwise folding
fn crt(congruences: &[(i128, i128)]) -> Option<(i128, i128)> {
    // try_fold: stops and returns None on first incompatible pair
    congruences.iter().try_fold((0i128, 1i128), |(r, m), &(a, mi)| {
        crt_combine(r, m, a, mi)
    })
}
// Example: x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → x = 23 (mod 105)
```

`try_fold` is the idiomatic Rust way to fold that can short-circuit on `None` — cleaner than explicit early return with `?`.

## What This Unlocks

- **RSA-CRT optimization**: Compute `m^d mod p` and `m^d mod q` separately (half the bit width → 4× faster exponentiation), combine with CRT — standard in all RSA implementations.
- **NTT-based polynomial multiplication**: Split polynomial coefficients into several NTT-friendly primes, multiply mod each, CRT-reconstruct — how `num-bigint` and competitive libraries do large polynomial multiplication.
- **Competitive programming**: "Find the smallest x such that x mod m1 = r1, x mod m2 = r2…" — direct CRT application, appears in calendar/scheduling problems.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 128-bit arithmetic | `Zarith` library or manual `Int64` | `i128` / `u128` native since Rust 1.26 |
| Extended GCD | `let rec egcd a b = ...` recursive | Identical recursive structure |
| Fold with early exit | `List.fold_left` + exception or `Option` | `iter().try_fold(...)` — idiomatic |
| Normalize modular result | `((x mod m) + m) mod m` | Same; add `+ lcm` to handle negatives |
| General (non-coprime) | Same CRT combine with GCD check | `if (a2 - a1) % g != 0 { return None }` |
