# Monad Composition

Monads don't compose directly like functors. We need monad transformers.

## The Problem

If M and N are monads, M<N<A>> is not automatically a monad.

## Solutions

1. **Monad Transformers**: OptionT, ResultT, StateT
2. **Explicit stacking**: `Result<Option<A>, E>`
3. **Effect systems**: Algebraic effects

## Common Stacks

- `OptionResult<A, E>` = `Result<Option<A>, E>`
- `ResultOption<A, E>` = `Option<Result<A, E>>`

## Usage

```rust
use example_656_fp_monad_composition::*;

let x: OptionResult<i32, &str> = Ok(Some(21));
let result = bind_option_result(x, |n| Ok(Some(n * 2)));
assert_eq!(result, Ok(Some(42)));
```
