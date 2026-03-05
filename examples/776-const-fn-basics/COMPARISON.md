# OCaml vs Rust: Const Fn Basics

## Compile-Time Evaluation

### Rust
```rust
pub const fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

// Evaluated at compile time!
pub const FACTORIAL_10: u64 = factorial(10);
```

### OCaml
No compile-time evaluation. Closest is:
```ocaml
(* Evaluated at module init time *)
let factorial_10 = factorial 10
```

## Using Const Values

### Rust
```rust
// Compile-time computed constants
pub const GCD_48_18: u64 = gcd(48, 18);
pub const TWO_TO_10: u64 = pow(2, 10);
pub const IS_17_PRIME: bool = is_prime(17);

// Array sizes from const fn
const SIZE: usize = digit_count(12345); // 5
let arr: [u8; SIZE] = [0; SIZE];
```

## Const Assertions

### Rust
```rust
const _: () = {
    assert!(factorial(5) == 120);  // Compile-time check!
    assert!(gcd(12, 8) == 4);
};
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Compile-time eval | None | `const fn` |
| Static assertions | None | `const { assert!() }` |
| Array size expr | Literal only | `const fn` result |
| Pure guarantee | Not enforced | `const` requires pure |
