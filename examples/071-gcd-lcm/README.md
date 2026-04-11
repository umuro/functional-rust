📖 **[View on hightechmind.io →](https://hightechmind.io/rust/071-gcd-lcm)**

---

# 071 — GCD and LCM

## Problem Statement

The Greatest Common Divisor (GCD) and Least Common Multiple (LCM) are among the most ancient and practical algorithms in mathematics. Euclid's algorithm for GCD (circa 300 BC) is one of the oldest known algorithms: `gcd(a, 0) = a` and `gcd(a, b) = gcd(b, a mod b)`. Each recursive call replaces the larger argument with the remainder, which strictly decreases, guaranteeing termination in O(log min(a, b)) steps. LCM follows from GCD: `lcm(a, b) = a * b / gcd(a, b)`.

GCD and LCM appear throughout computer science and mathematics: fraction simplification divides numerator and denominator by their GCD; synchronized clocks or processes that fire every p and q cycles next coincide at lcm(p, q) steps; hash table designers choose prime or coprime sizes to minimize clustering; RSA key generation verifies that the public exponent e is coprime to φ(n) using GCD; and musical tuning theory uses LCM to find the first beat where two rhythms re-align. Understanding Euclid's algorithm deeply is understanding the division algorithm — the foundation of modular arithmetic.

## Learning Outcomes

- Implement Euclidean GCD recursively and iteratively, understanding the termination proof
- Derive LCM from GCD using the safe form `a / gcd(a, b) * b` (dividing first avoids overflow)
- Extend GCD to lists using `reduce`: `gcd_list(v) = v.iter().copied().reduce(gcd)`
- Understand the loop invariant: `a % b < b` strictly decreases, guaranteeing termination
- Recognize that GCD is associative, making it composable with `fold` and `reduce`
- Connect to Bezout's identity and the extended Euclidean algorithm used in cryptography

## Rust Application

`gcd(a: i64, b: i64) -> i64` applies `a.abs()` first, then recurses:
- Base case: `gcd(a, 0) = a` — when b reaches 0, a is the GCD
- Recursive case: `gcd(b, a % b)` — the remainder strictly decreases each step

`lcm(a, b)` computes `(a * b).abs() / gcd(a, b)`, using `.abs()` to handle negative inputs. The divide-before-multiply form `a / gcd(a, b) * b` prevents intermediate overflow for large values.

`gcd_iter` (the iterative form) uses a while loop with two mutable bindings — equivalent behavior, no recursion. `gcd_list` uses `.reduce(gcd)` to apply GCD pairwise across a slice.

## OCaml Approach

OCaml's Euclidean GCD is a direct tail-recursive definition:

```ocaml
let rec gcd a b = if b = 0 then a else gcd b (a mod b)
let lcm a b = if a = 0 || b = 0 then 0 else a / gcd a b * b
```

For a list: `List.fold_left gcd 0 lst` exploits the identity `gcd(0, x) = x`. OCaml's arbitrary-precision integers (Zarith library) avoid all overflow in LCM computation for very large inputs. The standard library includes `Int.gcd` since OCaml 4.14.

## Key Differences

1. **Recursive vs iterative**: OCaml guarantees TCO for the tail-recursive Euclidean GCD. Rust's recursive version may overflow the stack for very large inputs (though GCD terminates in O(log min(a,b)) steps, making stack overflow unlikely in practice).
2. **Signed vs unsigned**: This implementation uses `u64`. For signed integers, absolute values must be taken first: `a.abs()` and `b.abs()`. OCaml's default integers are signed.
3. **`reduce` vs `fold_left`**: Rust's `reduce` does not need an identity element — it uses the first element. OCaml's `fold_left gcd 0 lst` works because `gcd(0, x) = x` (0 is the identity for GCD). Both are equivalent.
4. **Overflow in LCM**: `a * b` overflows `u64` for a, b > 2^32. The `a / gcd(a,b) * b` form avoids this if `a` is divisible by `gcd(a,b)` (which it always is by definition).

## Exercises

1. **Extended Euclidean**: Implement the extended Euclidean algorithm `extended_gcd(a, b) -> (i64, i64, i64)` returning `(gcd, x, y)` where `ax + by = gcd`. Used in modular inverse computation for cryptography.
2. **Modular inverse**: Using extended GCD, write `mod_inverse(a: i64, m: i64) -> Option<i64>` that returns `x` such that `ax ≡ 1 (mod m)`. Returns `None` if `gcd(a, m) != 1`.
3. **Fraction simplification**: Write `simplify(num: i64, den: i64) -> (i64, i64)` that reduces a fraction by dividing both by their GCD.
