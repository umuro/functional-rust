📖 **[View on hightechmind.io →](https://hightechmind.io/rust/827-modular-arithmetic)**

---

# 827: Modular Arithmetic

**Difficulty:** 4  **Level:** Advanced

Build a type-safe `ModInt` wrapper that enforces invariants and overloads operators — the foundation of all modular number theory in Rust.

## The Problem This Solves

Every cryptographic primitive, every competitive programming problem with "answer mod 10^9+7", every hash function — they all do arithmetic in ℤ_m. The challenge is engineering correctness: forgetting a `% m` here, overflowing a multiply there, getting a negative result from subtraction. These bugs are silent (values stay in the right range until they don't) and hard to track down.

The solution is a newtype wrapper `ModInt` that enforces `v ∈ [0, m)` by construction and overloads `+`, `-`, `*` to automatically stay in range. This is idiomatic Rust: encode invariants in the type system so the compiler catches violations, not your test suite.

Modular inverse is the key derived operation. When m is prime, Fermat's little theorem gives `a^(-1) ≡ a^(p-2) (mod p)` — computable with fast exponentiation. For general m, use the Extended Euclidean algorithm. Both are shown here.

## The Intuition

Modular arithmetic "wraps around" at m. Addition and subtraction stay safe with a single `% m` and a correction for negative subtraction results. Multiplication requires care: two `u64` values in [0, m) can have a product up to (10^9)² ≈ 10^18, which exceeds `u64::MAX` (≈ 1.8 × 10^19) — barely safe for m ≤ 10^9+7, but not for arbitrary u64 moduli. The safe idiom: widen to `u128` for the multiply, then reduce mod m.

Fermat inverse: `a × a^(p-2) ≡ a^(p-1) ≡ 1 (mod p)` by Fermat's little theorem. So `a^(-1) = a^(p-2) mod p`. Requires p prime. For general m, Extended Euclidean finds x such that `a×x + m×y = gcd(a,m) = 1` — then x mod m is the inverse.

## How It Works in Rust

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ModInt { v: u64, m: u64 }

impl ModInt {
    fn new(v: i64, m: u64) -> Self {
        // Handle negative inputs: (v % m + m) % m normalizes to [0, m)
        let v = ((v % m as i64) + m as i64) as u64 % m;
        ModInt { v, m }
    }

    // Modular exponentiation: base^exp mod m in O(log exp)
    fn pow(self, mut exp: u64) -> Self {
        let (mut base, mut result) = (self, ModInt::new(1, self.m));
        while exp > 0 {
            if exp & 1 == 1 { result = result * base; }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    // Fermat inverse: O(log m), requires m prime
    fn inv_fermat(self) -> Self {
        assert!(self.v != 0, "no inverse for 0");
        self.pow(self.m - 2)  // a^(p-2) mod p
    }

    // Extended Euclidean inverse: works for any coprime (v, m)
    fn inv(self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.v as i64, self.m as i64);
        if g != 1 { None } else { Some(ModInt::new(x, self.m)) }
    }
}

impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // u128 widening prevents overflow: (u64::MAX)^2 > u64::MAX
        let v = (self.v as u128 * rhs.v as u128 % self.m as u128) as u64;
        ModInt { v, m: self.m }
    }
}
// Add and Sub: (a + b) % m and (a + m - b) % m
```

The `impl Trait for Type` pattern is Rust's answer to OCaml's lack of operator overloading — `+`, `-`, `*` on `ModInt` now look like regular arithmetic in calling code.

## What This Unlocks

- **Competitive programming**: Combinatorics mod 10^9+7 — factorials, binomial coefficients, inverse factorials — all cleanly expressed with `ModInt`.
- **Cryptography implementation**: Modular inverse is the core of RSA key generation (`d = e^(-1) mod φ(n)`) and every elliptic curve point operation.
- **Polynomial arithmetic**: FFT-based polynomial multiplication works mod a prime; `ModInt` makes the coefficient arithmetic clean and safe.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Wrapper type | Record `{v: int; m: int}` or module | `struct ModInt { v: u64, m: u64 }` |
| Operator overload | Cannot overload `+`, use functions | `impl Add for ModInt` — full overloading |
| u128 multiply | `Int64` or Zarith for big values | `(a as u128 * b as u128) % m as u128` |
| Negative normalization | `((v mod m) + m) mod m` | Same; cast through `i64` then `u64` |
| Fermat inverse | `pow_mod a (m-2) m` function call | `self.pow(self.m - 2)` method |
