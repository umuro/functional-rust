## Core Insight

The Euclidean algorithm: `gcd(a, b) = gcd(b, a mod b)` with base case `gcd(a, 0) = a`. LCM derives from GCD. The recursive structure is identical in both languages.

## OCaml Approach
- Recursive function with pattern matching
- Tail-recursive naturally (last call is recursive)
- `abs` for handling negative inputs

## Rust Approach
- Recursive or iterative (loop with swap)
- No guaranteed TCO but small depth for GCD
- Can use `.abs()` method on integers

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| GCD | `let rec gcd a b = ...` | `fn gcd(a, b) -> ...` |
| Pattern | `match b with 0 -> a` | `if b == 0 { a }` |
| LCM | `a * b / gcd a b` | `a * b / gcd(a, b)` |
| Overflow | No (arbitrary precision with Zarith) | Possible with i32/i64 |
