# Contravariant Functor

Contravariant functors reverse morphism direction with `contramap`.

## Examples

- `Predicate<A>`: Test values, contramap transforms input
- `Comparator<A>`: Compare by projection
- `Equivalence<A>`: Equivalence by projection
- `Sink<A>`: Consumer, transforms before consuming

## Usage

```rust
use example_648_fp_contravariant::{Predicate, Comparator};

// Predicate composition
let is_even = Predicate::new(|x: &i32| x % 2 == 0);
let is_positive = Predicate::new(|x: &i32| *x > 0);
let combined = is_even.and(is_positive);

// Comparator by projection
let by_len = Comparator::new(|a: &i32, b: &i32| a.cmp(b))
    .contramap(|s: &String| s.len() as i32);
```
