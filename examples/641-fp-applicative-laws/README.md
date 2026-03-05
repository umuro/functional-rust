# Applicative Functor Laws

Demonstrates the four applicative functor laws in Rust.

## Laws

1. **Identity**: `pure id <*> v = v`
2. **Homomorphism**: `pure f <*> pure x = pure (f x)`
3. **Interchange**: `u <*> pure y = pure (|f| f(y)) <*> u`
4. **Composition**: `pure (.) <*> u <*> v <*> w = u <*> (v <*> w)`

## Usage

```rust
use example_641_fp_applicative_laws::{Applicative, option_applicative};

// Using our Applicative wrapper
let value = Applicative::pure(10);
let func = Applicative::pure(|x: i32| x * 2);
let result = value.ap(func); // Applicative(20)

// Using Option as Applicative
use option_applicative::*;
let f = pure(|x: i32| x + 1);
let v = pure(5);
assert_eq!(ap(f, v), Some(6));
```

## Running Tests

```bash
cargo test
```
