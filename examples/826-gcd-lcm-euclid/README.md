📖 **[View on hightechmind.io →](https://hightechmind.io/rust/826-gcd-lcm-euclid)**

---

# 826: GCD, LCM, and the Euclidean Algorithm

**Difficulty:** 3  **Level:** Intermediate

The oldest non-trivial algorithm (300 BC): compute GCD in O(log min(a,b)) — the cornerstone of all modular arithmetic and number theory.

## The Problem This Solves

GCD and LCM appear in every corner of computer science: fraction simplification, scheduling (when do two periodic events coincide?), modular inverse computation, Chinese Remainder Theorem, RSA key generation, and competitive programming problem after problem. The Euclidean algorithm is the prerequisite for all of these.

The seemingly simple `gcd(a, b) = gcd(b, a mod b)` recurrence is deceptively powerful: it runs in O(log min(a, b)) steps because the remainder shrinks by at least half every two steps (Fibonacci analysis). For 64-bit integers, this means at most ~93 iterations. No algorithm with fewer comparisons exists.

The Extended Euclidean algorithm (see #832) produces Bézout coefficients — integers x, y such that ax + by = gcd(a, b) — enabling modular inverses and CRT. Understanding plain GCD first makes the extension straightforward.

## The Intuition

`gcd(a, b)` = any common divisor of a and b also divides `a mod b` (because `a mod b = a - ⌊a/b⌋ × b`), and the common divisors of (b, a mod b) are exactly the common divisors of (a, b). So `gcd(a, b) = gcd(b, a mod b)`. Base case: `gcd(a, 0) = a`. Efficiency: `a mod b < a/2` whenever `b ≤ a/2`, so the problem halves every two recursive calls.

LCM: `lcm(a, b) = a / gcd(a, b) × b`. Divide *before* multiplying to prevent overflow — a common bug to know and avoid.

## How It Works in Rust

```rust
// Recursive: mirrors OCaml's one-liner elegantly
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Iterative: preferred for large inputs (no stack concerns)
// Rust does NOT guarantee tail-call optimization — always provide iterative version
fn gcd_iter(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;  // Save remainder before overwriting
        a = b;
        b = r;
    }
    a
}

// LCM: divide first to prevent overflow — critical for large u64 values
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b  // NOT: a * b / gcd(a, b) — would overflow for large a, b
}

// GCD of a slice: fold with neutral element 0 (gcd(0, x) = x)
fn gcd_slice(xs: &[u64]) -> u64 {
    xs.iter().fold(0u64, |acc, &x| gcd(acc, x))
}

// Binary GCD (Stein's): replaces modulo with bit shifts
// Faster on CPUs where division is expensive; same O(log n) asymptotically
fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();          // Remove factors of 2
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;                             // b = b - a, now even
        if b == 0 { break; }
    }
    a << shift
}
```

Note: Rust does not guarantee tail-call optimization. The recursive `gcd` is fine in practice (max ~93 stack frames for u64), but for production code handling untrusted inputs, prefer `gcd_iter`.

## What This Unlocks

- **Modular inverse and cryptography**: GCD = 1 is the prerequisite for modular inverse; the Extended Euclidean algorithm (built on this) is the foundation of RSA, ElGamal, and ECDSA.
- **Fraction arithmetic**: Simplify `a/b` to lowest terms with `gcd(a, b)` in O(log min(a,b)) — used in symbolic math, exact arithmetic, and rendering pipelines.
- **Scheduling and LCM**: "When do two tasks with period p1 and p2 synchronize?" → LCM(p1, p2). Used in real-time OS scheduling and animation frame synchronization.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive GCD | `let rec gcd a b = if b=0 then a else gcd b (a mod b)` | Identical structure; same risk of deep recursion |
| Tail recursion | OCaml compiler optimizes tail calls | Rust does NOT guarantee TCO; use iterative for safety |
| LCM overflow | `a * b / gcd a b` (risky) | `a / gcd(a, b) * b` — divide first |
| Slice fold | `List.fold_left gcd 0` | `iter().fold(0u64, gcd)` |
| Binary GCD | External implementation | `trailing_zeros()` makes Stein's idiomatic |
