📖 **[View on hightechmind.io →](https://hightechmind.io/rust/071-gcd-lcm)**

---

# 071 — GCD and LCM
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The Greatest Common Divisor (GCD) and Least Common Multiple (LCM) are fundamental number theory operations. Euclid's algorithm for GCD (300 BC) is one of the oldest algorithms in existence: `gcd(a, 0) = a`, `gcd(a, b) = gcd(b, a % b)`. LCM follows from GCD: `lcm(a, b) = a * b / gcd(a, b)`.

GCD and LCM appear in fraction simplification, synchronizing periodic events (when two clocks with different periods next align), computing hash table sizes (coprime for good distribution), musical tuning theory, and cryptography (RSA key generation uses GCD to verify coprimeness of e and φ(n)).

## Learning Outcomes

- Implement Euclidean GCD recursively and iteratively
- Derive LCM from GCD: `lcm(a, b) = a / gcd(a, b) * b` (divide first to avoid overflow)
- Extend to lists using `fold`: `gcd_list(v) = v.iter().copied().reduce(gcd)`
- Understand the invariant: `gcd(b, a % b)` where `a % b < b`, guaranteeing termination
- Recognize the connection to Bezout's identity and extended Euclidean algorithm

## Rust Application

`gcd(a, b)` uses `if b == 0 { a } else { gcd(b, a % b) }` — direct recursive Euclidean algorithm. `lcm(a, b)` divides before multiplying: `a / gcd(a, b) * b` prevents overflow that `a * b / gcd(a, b)` could cause. `gcd_list` uses `.reduce(gcd)` — applies gcd pairwise across the slice. `gcd_iter` accepts any `IntoIterator<Item=u64>` for generality.

## OCaml Approach

OCaml's version: `let rec gcd a b = if b = 0 then a else gcd b (a mod b)`. `lcm a b = if a = 0 || b = 0 then 0 else a / gcd a b * b`. For a list: `List.fold_left gcd 0 lst` (using GCD identity: `gcd(0, x) = x`). OCaml's arbitrary-precision integers (with Zarith) avoid overflow in LCM computation for large inputs.

## Key Differences

1. **Recursive vs iterative**: OCaml guarantees TCO for the tail-recursive Euclidean GCD. Rust's recursive version may overflow the stack for very large inputs (though GCD terminates in O(log min(a,b)) steps, making stack overflow unlikely in practice).
2. **Signed vs unsigned**: This implementation uses `u64`. For signed integers, absolute values must be taken first: `a.abs()` and `b.abs()`. OCaml's default integers are signed.
3. **`reduce` vs `fold_left`**: Rust's `reduce` does not need an identity element — it uses the first element. OCaml's `fold_left gcd 0 lst` works because `gcd(0, x) = x` (0 is the identity for GCD). Both are equivalent.
4. **Overflow in LCM**: `a * b` overflows `u64` for a, b > 2^32. The `a / gcd(a,b) * b` form avoids this if `a` is divisible by `gcd(a,b)` (which it always is by definition).

## Exercises

1. **Extended Euclidean**: Implement the extended Euclidean algorithm `extended_gcd(a, b) -> (i64, i64, i64)` returning `(gcd, x, y)` where `ax + by = gcd`. Used in modular inverse computation for cryptography.
2. **Modular inverse**: Using extended GCD, write `mod_inverse(a: i64, m: i64) -> Option<i64>` that returns `x` such that `ax ≡ 1 (mod m)`. Returns `None` if `gcd(a, m) != 1`.
3. **Fraction simplification**: Write `simplify(num: i64, den: i64) -> (i64, i64)` that reduces a fraction by dividing both by their GCD.
