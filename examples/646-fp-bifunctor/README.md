# Bifunctor

A bifunctor maps over two type parameters simultaneously.

## Operations

- `bimap`: Map both sides
- `first`: Map first parameter
- `second`: Map second parameter

## Instances

- `(A, B)`: Tuple
- `Result<T, E>`: Maps Ok with second, Err with first
- `Either<L, R>`: Explicit sum type

## Usage

```rust
use example_646_fp_bifunctor::{Bifunctor, Either};

let pair = (10, "hello");
let (doubled, len) = pair.bimap(|x| x * 2, |s| s.len());
// (20, 5)

let e: Either<i32, &str> = Either::Left(5);
let mapped = e.bimap(|x| x * 2, |s| s.len());
// Either::Left(10)
```
