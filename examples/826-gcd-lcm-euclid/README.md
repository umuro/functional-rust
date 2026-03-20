📖 **[View on hightechmind.io →](https://hightechmind.io/rust/826-gcd-lcm-euclid)**

---

# GCD and LCM — Euclidean Algorithm
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The greatest common divisor (GCD) and least common multiple (LCM) are foundational operations in arithmetic: reducing fractions, synchronizing periodic events, solving linear Diophantine equations, and implementing modular arithmetic all depend on them. The Euclidean algorithm computes GCD in O(log min(a,b)) — exponentially faster than trial factorization. Extended Euclidean finds Bezout coefficients (x, y such that ax + by = gcd(a,b)), enabling modular inverse computation essential for RSA and other cryptographic operations. These functions appear in virtually every number theory library.

## Learning Outcomes

- Implement the recursive Euclidean algorithm: `gcd(a, b) = gcd(b, a % b)`, base case `gcd(a, 0) = a`
- Derive LCM from GCD: `lcm(a, b) = (a / gcd(a, b)) * b` (divide before multiply to avoid overflow)
- Implement the extended Euclidean algorithm returning Bezout coefficients
- Understand the connection: `gcd(a,b) = 1` implies a has a modular inverse mod b
- Apply to: fraction reduction, Stern-Brocot tree, continued fractions, CRT

## Rust Application

```rust
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { (a / gcd(a, b)) * b }
}
```

The recursive `gcd` is tail-recursive; Rust doesn't guarantee TCO but the compiler often optimizes it. For guaranteed no-stack-overflow, an iterative version with `while b != 0 { let t = b; b = a % b; a = t; }` is preferred. The LCM formula divides `a` by GCD first (not last) to prevent overflow when `a * b` would exceed `u64::MAX`. Rust's type system enforces `u64` throughout, preventing signed/unsigned mixups. The `gcd` of signed integers needs careful handling of the sign; using `u64` avoids this entirely.

## OCaml Approach

OCaml's recursive GCD is identical in structure: `let rec gcd a b = if b = 0 then a else gcd b (a mod b)`. OCaml's optimizer performs tail call elimination, so deep recursion is stack-safe. The `Int.abs` handles negative inputs for signed `int`. OCaml's standard library includes `Int.gcd` in recent versions. The `Zarith` library provides GCD for arbitrary-precision integers. OCaml's `let lcm a b = (a / gcd a b) * b` mirrors Rust exactly. The extended Euclidean algorithm uses a pair return type `(int * int * int)` for `(gcd, x, y)`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Recursion | Tail-recursive, TCO not guaranteed | TCO guaranteed |
| Signed integers | Use `i64` with `abs()` | `Int.abs` or `abs` |
| Standard library | `num::integer::gcd` (crate) | `Int.gcd` (recent OCaml) |
| Return type | Single `u64` | Single `int` |
| Extended GCD | Returns `(i64, i64, i64)` | `(int * int * int)` |
| Overflow prevention | Divide before multiply | Same idiom |

## Exercises

1. Implement the extended Euclidean algorithm returning `(gcd, x, y)` such that `a*x + b*y = gcd(a,b)`.
2. Use extended GCD to compute the modular inverse of a mod m when gcd(a, m) = 1.
3. Compute GCD of a list of numbers: verify `gcd(a, b, c) = gcd(gcd(a, b), c)`.
4. Implement a `Fraction` type with automatic reduction using GCD on construction.
5. Benchmark recursive vs. iterative GCD and the binary GCD algorithm (using bit operations) on 64-bit integers.
