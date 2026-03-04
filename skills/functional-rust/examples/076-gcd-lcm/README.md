# 076: GCD and LCM — Euclidean Algorithm

**Difficulty:** 2  **Level:** Beginner

Recursive Euclidean GCD, derived LCM, and fold-based variants for sequences — pure math with `Copy` types.

## The Problem This Solves

Finding the greatest common divisor and least common multiple are fundamental number theory operations — used in fraction reduction, scheduling (find a common period), cryptography, and anywhere you need to simplify ratios.

The Euclidean algorithm is also the canonical example of a pure recursive algorithm with no ownership complexity: integers are `Copy`, no heap allocation, and the recursive structure maps 1:1 from OCaml to Rust with zero changes to the logic.

## The Intuition

The Euclidean algorithm: `gcd(a, 0) = a`, `gcd(a, b) = gcd(b, a % b)`. That's it. Each call reduces the problem: `gcd(48, 18)` → `gcd(18, 12)` → `gcd(12, 6)` → `gcd(6, 0)` = 6.

LCM follows from GCD: `lcm(a, b) = a / gcd(a, b) * b`. The division before multiplication avoids overflow — divide first to reduce the magnitude, then multiply.

For a list: fold with `gcd` as the combining function. `gcd_list([12, 18, 24]) = gcd(gcd(12, 18), 24) = gcd(6, 24) = 6`.

## How It Works in Rust

```rust
// Euclidean algorithm — integers are Copy, so ownership is trivial
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// LCM via GCD — divide first to prevent overflow
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b }
}

// GCD of a slice — fold pattern: OCaml's List.fold_left gcd 0
pub fn gcd_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(gcd).unwrap_or(0)
    //          ^^^^^^ Copy out each &u64 so we pass u64 to gcd directly
    //                              ^^^^^^ reduce = fold without initial value
}

// Accept any iterable — more flexible signature
pub fn gcd_iter(nums: impl IntoIterator<Item = u64>) -> u64 {
    nums.into_iter().reduce(gcd).unwrap_or(0)
}

// LCM of a list
pub fn lcm_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(lcm).unwrap_or(0)
}
```

`.copied()` on an iterator of `&u64` turns `Option<&u64>` into `Option<u64>` — necessary because `gcd` takes `u64`, not `&u64`. With `Copy` types, `.copied()` is idiomatic and free.

`.reduce()` is like `.fold()` but uses the first element as the initial accumulator — appropriate when there's no sensible default (what's `gcd` of an empty list? We use 0 as a convention).

## What This Unlocks

- **Fraction simplification**: `Fraction { num / gcd(num, den), den / gcd(num, den) }`.
- **Scheduling**: LCM of periods gives the first time all periodic events coincide.
- **Modular arithmetic**: GCD tells you if `ax ≡ b (mod n)` has a solution.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive GCD | `let rec gcd a b = if b = 0 then a else gcd b (a mod b)` | `fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }` |
| Fold over list | `List.fold_left gcd 0` | `.iter().copied().reduce(gcd)` |
| Ownership cost | None (GC) | None (Copy types) |
| Overflow guard | Manual | `a / gcd(a, b) * b` pattern |
| Empty case | `0` or exception | `.unwrap_or(0)` |
