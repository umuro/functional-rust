# Yoneda Lemma

The Yoneda lemma enables powerful optimization through map fusion.

## Key Insight

Instead of `F<A>`, store `forall B. (A -> B) -> F<B>`.
This defers computation, allowing maps to compose without traversals.

## Benefits

- **Map fusion**: Multiple maps become one traversal
- **Deferred execution**: Compute only when needed
- **Coyoneda**: Add functor instance to any type

## Usage

```rust
use example_651_fp_yoneda::{YonedaVec, Coyoneda};

// Fused traversal - only one pass!
let result = YonedaVec::lift(vec![1, 2, 3])
    .map(|x| x + 1)
    .map(|x| x * 2)
    .lower();

// Coyoneda - accumulate transforms
let c = Coyoneda::lift(10)
    .map(|x| x + 5)
    .map(|x| x * 2);
assert_eq!(c.lower(), 30);
```
