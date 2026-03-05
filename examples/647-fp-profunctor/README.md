# Profunctor

A profunctor is contravariant in input and covariant in output.

## Operations

- `dimap`: Map both input and output
- `lmap`: Map input (contravariant)
- `rmap`: Map output (covariant)

## Key Insight

Functions `Fn(A) -> B` are the canonical profunctor:
- Contravariant in A: can pre-compose with `C -> A`
- Covariant in B: can post-compose with `B -> D`

## Usage

```rust
use example_647_fp_profunctor::{Func, Profunctor, dimap_fn};

let f = Func::new(|x: i32| x * 2);
let g = f.dimap(
    |s: &str| s.len() as i32,
    |x| format!("Result: {}", x)
);
println!("{}", g.call("hello")); // "Result: 10"
```
