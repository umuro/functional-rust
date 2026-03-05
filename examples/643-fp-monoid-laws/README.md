# Monoid Laws

A monoid is a semigroup with an identity element (`empty`).

## Laws

1. **Left identity**: `empty <> x = x`
2. **Right identity**: `x <> empty = x`
3. **Associativity**: `(x <> y) <> z = x <> (y <> z)`

## Examples

- `Sum(0)`: Additive identity
- `Product(1)`: Multiplicative identity
- `String::new()`: Empty string
- `All(true)`: Boolean AND
- `Any(false)`: Boolean OR

## Usage

```rust
use example_643_fp_monoid_laws::{Monoid, Sum, mconcat};

let values = vec![Sum(1), Sum(2), Sum(3)];
let total = mconcat(values); // Sum(6)
```
