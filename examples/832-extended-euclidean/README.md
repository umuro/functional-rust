📖 **[View on hightechmind.io →](https://hightechmind.io/rust/832-extended-euclidean)**

---

# Extended Euclidean Algorithm

## Problem Statement

The basic Euclidean algorithm finds gcd(a, b) but not the coefficients. The extended version also finds Bezout coefficients x, y such that `a*x + b*y = gcd(a, b)`. These coefficients are essential for computing modular inverses (when gcd(a, m) = 1, x is a's inverse mod m), solving linear Diophantine equations `ax + by = c`, and implementing CRT. Without the extended algorithm, modular inverse requires Fermat's little theorem (only works for prime moduli) or Euler's theorem (requires phi(m), harder to compute). Extended GCD works for any modulus, not just primes.

## Learning Outcomes

- Understand the recursive structure: if `extended_gcd(b, a%b) = (g, x1, y1)`, then for the original pair: `x = y1`, `y = x1 - (a/b)*y1`
- Derive Bezout coefficient update via the recurrence relationship
- Apply to modular inverse: `mod_inv(a, m) = x mod m` where `(g, x, _) = extended_gcd(a, m)` and g = 1
- Solve general linear Diophantine `ax + by = c`: has solution iff gcd(a,b) divides c
- Understand sign handling: Bezout coefficients can be negative

## Rust Application

```rust
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)  // gcd, x, y: a*1 + 0*0 = a
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a.rem_euclid(m), m);
    if g != 1 { None } else { Some(x.rem_euclid(m)) }
}
```

Using `i64` handles negative Bezout coefficients naturally. The `rem_euclid` method computes the mathematical modulo (always non-negative), avoiding negative result issues from Rust's `%` operator on negative numbers. The triple return `(g, x, y)` is a natural Rust pattern for multiple related outputs. The iterative version avoids stack depth issues for large inputs: maintain `(old_r, r)`, `(old_s, s)`, `(old_t, t)` and update simultaneously.

## OCaml Approach

OCaml's recursive extended GCD returns `(int * int * int)` exactly as in Rust. The pattern `let (g, x1, y1) = extended_gcd b (a mod b) in (g, y1, x1 - (a / b) * y1)` is identical in structure. OCaml handles negative remainders from `mod` differently: `(-7) mod 3 = -1` in OCaml (truncating division), requiring explicit adjustment. The `((x mod m) + m) mod m` idiom ensures non-negative modular inverse. OCaml's `Int.rem` has the same truncating behavior.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Negative modulo | `rem_euclid` for mathematical mod | `((x mod m) + m) mod m` |
| Return type | Tuple `(i64, i64, i64)` | Tuple `(int * int * int)` |
| Stack safety | Tail-recursive iter preferred | TCO makes recursion safe |
| Modular inverse | Returns `Option<i64>` | Returns `int option` |
| Bezout signs | Can be negative (correct) | Same |
| Integer size | `i64` for 64-bit inputs | `int` (63-bit) or `Int64` |

## Exercises

1. Verify the extended GCD output: for all tested pairs (a, b), check that `a*x + b*y == gcd(a,b)`.
2. Implement iterative extended GCD to avoid stack depth issues for large inputs.
3. Solve the linear Diophantine equation `ax + by = c` or report no solution, using extended GCD.
4. Use the extended GCD to implement CRT for two congruences without explicitly computing phi.
5. Implement a batch modular inverse computation for a list of values mod p using the recurrence `inv[i] = -(p/i) * inv[p%i] mod p`.
