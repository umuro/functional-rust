# Adjunction

Adjunctions are pairs of functors with natural transformations unit and counit.

## Key Insight

Every adjunction F ⊣ G gives rise to:
- A **monad**: G ∘ F
- A **comonad**: F ∘ G

## Examples

1. **Product-Exponential**: (A × _) ⊣ (_ → A)
   - Monad: State
   - Comonad: Store

2. **Free-Forgetful**: Free ⊣ U
   - Produces free monads

## Usage

```rust
use example_650_fp_adjunction::{State, Store, curry};

// State monad from adjunction
let computation = State::get()
    .flat_map(|x: i32| State::pure(x * 2));
let (result, _) = computation.run(21);
assert_eq!(result, 42);

// Currying from adjunction
let curried = curry(|a: i32, b: i32| a + b);
```
