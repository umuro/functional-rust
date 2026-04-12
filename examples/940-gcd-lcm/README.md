**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[gcd-lcm on hightechmind.io](https://hightechmind.io/posts/functional-rust/gcd-lcm)

---

## Problem Statement

Implement GCD (greatest common divisor) and LCM (least common multiple) using Euclid's algorithm. Extend the scalar operations to lists using `fold`/`reduce`, and compare a recursive implementation against an iterative one. Both algorithms are elegant in functional style and highlight Rust's tail-call behavior relative to OCaml's TCO guarantee.

## Learning Outcomes

- Implement recursive Euclid's GCD: `gcd(a, b) = gcd(b, a % b)` with base case `b == 0`
- Derive LCM from GCD: `lcm(a, b) = a / gcd(a, b) * b` (divide before multiplying to avoid overflow)
- Extend scalar GCD/LCM to lists using `Iterator::reduce`
- Understand the difference between recursive and iterative implementations and when each matters in Rust
- Recognize Rust's lack of guaranteed TCO and when to prefer iterative implementations for safety

## Rust Application

```rust
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b }
}

// Extend to lists via reduce
pub fn gcd_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(gcd).unwrap_or(0)
}

pub fn lcm_list(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(lcm).unwrap_or(0)
}

// Iterative version avoids stack overflow for very deep inputs
pub fn gcd_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
```

The LCM formula divides `a` by `gcd(a, b)` before multiplying by `b`. This order matters: `a / gcd * b` never overflows when the result fits in `u64`, but `a * b / gcd` overflows `u64` for large inputs.

`reduce` returns `None` for empty slices, so `unwrap_or(0)` provides a sensible identity (GCD of zero elements is 0 by convention). The identity for GCD is 0 (since `gcd(0, n) = n`), and the identity for LCM is 1.

## OCaml Approach

```ocaml
let rec gcd a b =
  if b = 0 then a else gcd b (a mod b)

let lcm a b =
  if a = 0 || b = 0 then 0
  else a / gcd a b * b

let gcd_list = List.fold_left gcd 0
let lcm_list = List.fold_left lcm 1

(* Tail position: OCaml guarantees TCO here *)
(* gcd is already in tail position — the recursive call is the last action *)
```

OCaml guarantees tail-call optimization for functions in tail position. The recursive `gcd` above calls `gcd b (a mod b)` as its final action — this is a proper tail call, so OCaml compiles it to a loop. No stack frames accumulate regardless of input size.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Tail-call optimization | Not guaranteed; use iterative for safety | Guaranteed for tail-recursive functions |
| `reduce` vs `fold_left` | `reduce` uses first element as init; `fold_left` needs explicit identity | `fold_left` — same semantics |
| Integer overflow | `u64` saturates silently at limits; divide-first avoids overflow | Same; `Int64` or arbitrary precision for big numbers |
| Pattern match style | `if b == 0 { a } else { ... }` | `if b = 0 then a else ...` |

The recursive implementation reads almost identically in both languages. The practical difference is that OCaml's TCO guarantee means you can safely use recursion for arbitrarily large inputs, while Rust's iterative version is the production-safe choice for unknown input sizes.

## Exercises

1. Verify the GCD algebraic identities: `gcd(a, 0) = a`, `gcd(0, a) = a`, `gcd(a, b) = gcd(b, a)`.
2. Implement `coprime(a, b) -> bool` using GCD and use it to filter pairs from a list.
3. Compute `lcm(1..=20)` — the smallest number divisible by 1 through 20 (Project Euler problem 5).
4. Implement a version that accepts `i64` with proper sign handling (`gcd(|a|, |b|)`).
5. Benchmark the recursive vs iterative GCD for large inputs to measure the practical stack impact in Rust.
