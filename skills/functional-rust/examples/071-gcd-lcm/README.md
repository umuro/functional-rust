# 071: GCD and LCM

**Difficulty:** ⭐  **Level:** Foundations

Compute greatest common divisor and least common multiple — the Euclidean algorithm is 2300 years old and fits in one line of Rust.

## The Problem This Solves

You need to simplify fractions, find the scheduling period for two recurring events, or reduce a set of numbers to their common factor. These all reduce to GCD and LCM.

The brute-force approach tries every number from 1 up to `min(a, b)`. That's O(n) and slow for large inputs. Euclid's algorithm is O(log n) and looks almost magical: `gcd(a, b) = gcd(b, a mod b)`. The modulo repeatedly reduces the problem until one number becomes 0.

Both OCaml and Rust express this in nearly identical code. The algorithm itself is the lesson — the language just gets out of the way.

## The Intuition

Why does `gcd(a, b) = gcd(b, a % b)` work? Because any common divisor of `a` and `b` must also divide `a - b`, `a - 2b`, ..., and `a mod b`. So the set of common divisors doesn't change when you replace `a` with `a mod b`. You keep reducing until one number is 0, at which point the other number is the GCD.

LCM follows from a simple identity: `lcm(a, b) = a * b / gcd(a, b)`. Divide before multiplying to avoid integer overflow.

For lists of numbers, fold the binary GCD/LCM across the list: `gcd(a, gcd(b, gcd(c, d)))`.

## How It Works in Rust

```rust
// Euclid's algorithm — clean recursion
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// LCM via GCD identity — divide first to prevent overflow
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 }
    else { a / gcd(a, b) * b }
}

// GCD of a whole list — fold with reduce
pub fn gcd_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(gcd).unwrap_or(0)
}

// LCM of a list — same pattern
pub fn lcm_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(lcm).unwrap_or(0)
}
```

`reduce()` is like `fold()` but uses the first element as the initial accumulator. Returns `None` for empty slices — `unwrap_or(0)` handles that.

## What This Unlocks

- **Fraction arithmetic** — simplify `a/b + c/d` by computing LCM of denominators
- **Scheduling** — find when two periodic events next coincide (LCM of their periods)
- **Number theory** — GCD is the foundation of modular arithmetic, RSA encryption, and more

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive GCD | `let rec gcd a b = if b = 0 then a else gcd b (a mod b)` | Identical structure |
| LCM | `abs (a * b) / gcd a b` | `a / gcd(a, b) * b` (divide first) |
| List GCD | `List.fold_left gcd h t` | `.reduce(gcd).unwrap_or(0)` |
| Tail recursion | OCaml optimizes automatically | Rust may stack-overflow on huge inputs; use iterative version |
