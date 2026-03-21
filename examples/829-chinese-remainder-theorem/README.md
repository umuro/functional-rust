📖 **[View on hightechmind.io →](https://hightechmind.io/rust/829-chinese-remainder-theorem)**

---

# Chinese Remainder Theorem
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

The Chinese Remainder Theorem (CRT) solves systems of simultaneous congruences: given `x ≡ a1 (mod m1)`, `x ≡ a2 (mod m2)`, ..., find x mod (m1 * m2 * ...) when the moduli are pairwise coprime. This has applications in: cryptography (RSA-CRT speedup), number theory, calendar calculations, and distributed hashing. CRT also enables working with multiple small moduli instead of one large modulus, which is the key technique in Number Theoretic Transform (NTT) for polynomial multiplication. In competitive programming, CRT reconstructs a number from its remainders under several moduli, enabling range queries on large integers.

## Learning Outcomes

- State the CRT: system `x ≡ ai (mod mi)` with pairwise coprime mi has unique solution mod M = m1*m2*...*mk
- Implement the constructive proof: for each i, compute Mi = M/mi, then `yi = Mi^(-1) mod mi` via extended GCD
- Combine: `x = sum(ai * Mi * yi) mod M`
- Handle the case of non-coprime moduli using the generalized CRT
- Apply CRT to RSA decryption speedup: compute separately mod p and mod q, combine with CRT

## Rust Application

```rust
pub fn crt(remainders: &[i64], moduli: &[i64]) -> Option<i64> {
    let m: i64 = moduli.iter().product();
    let mut x = 0i64;
    for (&a, &mi) in remainders.iter().zip(moduli.iter()) {
        let m_i = m / mi;
        let y_i = mod_inverse(m_i, mi)?; // extended GCD
        x = (x + a * m_i % m * y_i) % m;
    }
    Some(((x % m) + m) % m)
}
```

The `?` operator propagates `None` when `mod_inverse` fails (moduli not coprime), making the function total and safe. The `((x % m) + m) % m` idiom ensures the result is non-negative when intermediate products go negative. Rust's `i64` handles the arithmetic, but the product `m` can overflow for many large moduli — production code should use `i128` or `BigInt`. The `zip` iterator pairs remainders with moduli cleanly. The function returns `Option<i64>` to signal failure gracefully.

## OCaml Approach

OCaml's CRT uses `List.fold_left` to accumulate the sum: `List.fold_left2 (fun acc a mi -> ...) 0 remainders moduli`. The modular inverse uses the extended Euclidean algorithm returning `int * int`. OCaml's `Int64` or `Zarith` handles large products. The `Option` type signals failure when moduli are not coprime. OCaml's pattern matching on the extended GCD result tuple is clean: `let (g, x, _) = extended_gcd mi m_i in if g <> 1 then None else Some x`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Modular inverse | Extended GCD returning `Option<i64>` | Returns `(int * int * int)` |
| Accumulation | `zip` iterator loop | `List.fold_left2` |
| Overflow | `i128` for large product M | `Zarith` for arbitrary precision |
| Error handling | `?` operator for `None` propagation | `Option.bind` chain |
| Non-negative result | `((x % m) + m) % m` | Same idiom |
| Return type | `Option<i64>` | `int option` |

## Exercises

1. Implement CRT for two congruences iteratively and verify on the classic example: x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7).
2. Extend to handle non-coprime moduli using the generalized CRT with GCD-based compatibility checking.
3. Implement RSA-CRT decryption: given ciphertext c, p, q, d, compute p-component and q-component separately, then combine.
4. Use CRT with NTT-friendly primes to multiply polynomials with large coefficients.
5. Benchmark CRT-based RSA decryption against naive modular exponentiation with the full modulus.
