# Kleisli Category

The Kleisli category of a monad M provides composition for effectful functions.

## Structure

- **Objects**: Types
- **Morphisms**: `A -> M<B>` (Kleisli arrows)
- **Composition**: `(>=>)` fish operator
- **Identity**: `pure`/`return`

## Laws

1. **Left identity**: `pure >=> f = f`
2. **Right identity**: `f >=> pure = f`
3. **Associativity**: `(f >=> g) >=> h = f >=> (g >=> h)`

## Usage

```rust
use example_657_fp_kleisli_category::*;

let f = |x: i32| if x > 0 { Some(x) } else { None };
let g = |x: i32| Some(x * 2);

let composed = kleisli_compose_option(f, g);
assert_eq!(composed(5), Some(10));
```
