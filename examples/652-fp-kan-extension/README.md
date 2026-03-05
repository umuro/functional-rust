# Kan Extensions

Kan extensions are universal constructions that generalize many FP concepts.

## Types

- **Right Kan Extension (Ran)**: Generalizes limits
- **Left Kan Extension (Lan)**: Generalizes colimits

## Derived Structures

- **Codensity monad**: `Ran_Id Id` - improves left-associative binds
- **Density comonad**: `Lan_Id Id`
- **Free monad**: `Lan_F Id`
- **Yoneda**: `Ran_K (Hom(K-, -))`

## Usage

```rust
use example_652_fp_kan_extension::{Codensity, ran_option};

// Codensity for efficient monadic composition
let result = Codensity::pure(10)
    .flat_map(|x| Codensity::pure(x + 5))
    .run();

// Ran for Option
let f = ran_option(0, |x: i32| x * 2);
assert_eq!(f(Some(21)), 42);
```
