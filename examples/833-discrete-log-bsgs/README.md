📖 **[View on hightechmind.io →](https://hightechmind.io/rust/833-discrete-log-bsgs)**

---

# 833: Discrete Logarithm — Baby-Step Giant-Step

**Difficulty:** 5  **Level:** Master

Solve a^x ≡ b (mod p) in O(√p) time and space — meet-in-the-middle applied to cyclic groups.

## The Problem This Solves

The discrete logarithm problem (DLP): given a, b, and prime p, find x such that a^x ≡ b (mod p). This is the computational hard problem underlying Diffie-Hellman key exchange, ElGamal encryption, and the Digital Signature Algorithm (DSA). While modular exponentiation is fast (O(log x)), the inverse — finding x from a^x — has no known polynomial algorithm for large p.

Baby-Step Giant-Step (BSGS) is the canonical O(√p) algorithm: impractical for cryptographic primes (p ≈ 2²⁵⁶) but essential for p up to ~10¹² in competitive programming and for pedagogical understanding of why DLP is hard. It also appears in factoring algorithms (Pohlig-Hellman reduces DLP to BSGS on prime-order subgroups).

The algorithm returns `Some(x)` if a solution exists, or `None` if b is not a power of a modulo p.

## The Intuition

Write x = i · m - j, where m = ⌈√p⌉ and 0 ≤ i, j < m. Then a^x ≡ b becomes a^(im) ≡ b · a^j (mod p).

**Baby steps**: compute b · a^j for j = 0, 1, ..., m-1. Store in a hash map: value → j.
**Giant steps**: compute a^(im) for i = 1, 2, ..., m. If this value is in the hash map, you have i · m - j = x.

This is meet-in-the-middle: instead of trying all x from 0 to p-1 (O(p) work), you meet from both sides in O(√p) work and O(√p) space.

O(√p) time for both phases. The hash map lookup makes giant steps O(1) per query. OCaml uses `Hashtbl`. Rust uses `HashMap<u64, u64>` — same concept, but with explicit hash function choices affecting performance significantly for large m.

## How It Works in Rust

```rust
use std::collections::HashMap;

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % modulus; }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn baby_step_giant_step(a: u64, b: u64, p: u64) -> Option<u64> {
    let m = (p as f64).sqrt().ceil() as u64 + 1;

    // Baby steps: store b * a^j → j for j in [0, m)
    let mut table: HashMap<u64, u64> = HashMap::with_capacity(m as usize);
    let mut baby = b % p;
    for j in 0..m {
        table.insert(baby, j);
        baby = baby * a % p; // baby = b * a^j
    }

    // Giant steps: compute a^(i*m) for i in [1, m]
    let am = mod_pow(a, m, p); // a^m mod p
    let mut giant = am;        // a^(1*m), a^(2*m), ...
    for i in 1..=m {
        if let Some(&j) = table.get(&giant) {
            // a^(i*m) = b * a^j  =>  x = i*m - j
            let x = i * m - j;
            return Some(x % (p - 1)); // reduce mod group order
        }
        giant = giant * am % p;
    }
    None // b is not a power of a mod p
}
```

`HashMap::with_capacity(m)` pre-allocates to avoid rehashing during the baby-step phase — a significant speedup for large m (avoid ~log m reallocations).

The `mod_pow` function uses fast exponentiation (square-and-multiply): O(log exp) multiplications. For p up to 10¹², intermediate products fit in u64 only if p < 2³² — for larger p, use u128 or modular multiplication via u128 intermediate.

The reduction `x % (p - 1)` handles the case where x > p-1 — since a^(p-1) ≡ 1 (mod p) by Fermat's little theorem, solutions repeat with period p-1.

## What This Unlocks

- **Diffie-Hellman security analysis**: BSGS directly attacks DH key exchange when p is small (p < 10¹²); for cryptographic p (~2²⁵⁶), only index calculus methods scale.
- **Pohlig-Hellman reduction**: when p-1 has small prime factors, DLP reduces to BSGS on subgroups — explaining why safe primes (p = 2q+1, q prime) are required in DH.
- **Competitive programming**: discrete log problems with p ≤ 10¹² are standard in ICPC and Codeforces — BSGS is the expected solution.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hash map | `Hashtbl.create m` | `HashMap::with_capacity(m)` — pre-allocated |
| Modular arithmetic | `Int64` or native (63-bit, risky) | `u64` for p < 2³², u128 for larger |
| Fast exponentiation | Recursive or iterative with `Int64` | `mod_pow` with `base %= modulus` guard |
| Square root ceiling | `int_of_float (sqrt (float p)) + 1` | `(p as f64).sqrt().ceil() as u64 + 1` |
| Group order reduction | Manual `mod (p-1)` | `x % (p - 1)` — same, but type-checked u64 |
