# Semigroup Laws

A semigroup is a type with an associative binary operation (`combine`).

## Law

**Associativity**: `(a <> b) <> c = a <> (b <> c)`

## Examples

- `Sum`: Addition semigroup
- `Product`: Multiplication semigroup
- `String`: Concatenation semigroup
- `Max/Min`: Maximum/Minimum selection

## Usage

```rust
use example_642_fp_semigroup_laws::{Semigroup, Sum, Product, Max};

let sum = Sum(1).combine(Sum(2)); // Sum(3)
let prod = Product(3).combine(Product(4)); // Product(12)
let max = Max(5).combine(Max(3)); // Max(5)
```

## Running Tests

```bash
cargo test
```
