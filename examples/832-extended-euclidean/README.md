📖 **[View on hightechmind.io →](https://hightechmind.io/rust/832-extended-euclidean)**

---

# 832: Extended Euclidean Algorithm

**Difficulty:** 4  **Level:** Advanced

Find integers x, y such that ax + by = gcd(a, b) — the foundation of modular inverses and the Chinese Remainder Theorem.

## The Problem This Solves

The extended Euclidean algorithm computes not just gcd(a, b) but also the Bézout coefficients x and y satisfying ax + by = gcd(a, b). These coefficients are the key to modular arithmetic: if gcd(a, m) = 1, then ax ≡ 1 (mod m), making x the modular inverse of a.

Modular inverses are essential in RSA (computing the private key d = e⁻¹ mod φ(n)), in the Chinese Remainder Theorem (CRT) for combining solutions to simultaneous congruences, in polynomial interpolation over finite fields, and in any modular division a/b mod m (computed as a · b⁻¹ mod m). Without extended GCD, you'd need Fermat's little theorem (which only works when m is prime) or Euler's theorem.

This example computes `(gcd, x, y)` iteratively and derives the modular inverse as a clean wrapper, with correct handling of negative coefficients.

## The Intuition

Standard Euclidean: gcd(a, b) = gcd(b, a mod b), unwinding until b = 0.

Extended: as we unwind, track how each remainder is a linear combination of the original a and b. At each step: `r[i] = r[i-2] - q[i] · r[i-1]`, and the same relation holds for the Bézout coefficients: `x[i] = x[i-2] - q[i] · x[i-1]`.

The iterative version maintains `(old_r, r, old_s, s)` where s tracks the x-coefficient (the y-coefficient follows from the equation). When r reaches 0, old_r = gcd and old_s = x.

The result x may be negative — normalise with `(x % m + m) % m` for a positive modular inverse.

O(log min(a,b)) steps — same as standard GCD, with constant extra work per step.

## How It Works in Rust

```rust
// Returns (gcd, x, y) where a*x + b*y = gcd
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0); // a*1 + 0*0 = a
    }
    // Iterative — avoids recursion and is easier to follow
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1i64, 0i64); // x-coefficients
    let (mut old_t, mut t) = (0i64, 1i64); // y-coefficients

    while r != 0 {
        let q = old_r / r;
        // Advance remainder: r_{i+1} = r_{i-1} - q * r_i
        (old_r, r) = (r, old_r - q * r);
        // Same recurrence for Bézout coefficients
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    // old_r = gcd, old_s = x, old_t = y
    (old_r, old_s, old_t)
}

// Modular inverse of a mod m (requires gcd(a,m) = 1)
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a.rem_euclid(m), m);
    if g != 1 {
        None // a and m are not coprime — inverse doesn't exist
    } else {
        Some(x.rem_euclid(m)) // normalise to [0, m)
    }
}
```

Rust's tuple destructuring in assignments `(old_r, r) = (r, old_r - q * r)` is clean and avoids temporary variables. The right-hand side is fully evaluated before assignment — equivalent to a swap with computation, no aliasing risk.

`rem_euclid(m)` is preferred over `% m` for negative numbers: `(-3i64).rem_euclid(7) == 4`, while `(-3i64) % 7 == -3`. Always use `rem_euclid` for modular arithmetic with potentially negative values.

For the Chinese Remainder Theorem: given x ≡ a₁ (mod m₁) and x ≡ a₂ (mod m₂) with gcd(m₁, m₂) = 1, the solution is `x = a₁ + m₁ · (a₂ - a₁) · mod_inverse(m₁, m₂)`.

## What This Unlocks

- **RSA private key**: `d = mod_inverse(e, phi_n)` — the decryption exponent is a modular inverse via extended GCD.
- **Chinese Remainder Theorem**: combine simultaneous congruences using Bézout coefficients to find a unique solution mod (m₁ · m₂ · … · mₖ).
- **Modular division in finite fields**: `a / b mod p` = `a * mod_inverse(b, p)` — used in polynomial interpolation, Reed-Solomon codes, and cryptographic protocols.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Simultaneous assignment | `let (a, b) = (b, a-q*b)` | `(old_r, r) = (r, old_r - q * r)` — same pattern |
| Negative modulo | `((x mod m) + m) mod m` | `x.rem_euclid(m)` — built-in, correct for negative x |
| Recursive vs iterative | Natural recursion | Iterative with `(old_r, r, old_s, s, old_t, t)` |
| Inverse existence check | `if g <> 1 then failwith ...` | `Option<i64>` — idiomatic `None` for non-coprime case |
| i64 overflow | `Int64` or 63-bit int | `i64` — explicit, safe for numbers up to ~9.2 × 10¹⁸ |
