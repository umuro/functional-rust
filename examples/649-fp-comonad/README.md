# Comonad

Comonads are the categorical dual of monads.

## Core Operations

- `extract`: Pull out the focused value (dual of `pure`)
- `extend`: Apply contextual computation (dual of `bind`)
- `duplicate`: Nest contexts (dual of `join`)

## Examples

- `NonEmpty<T>`: Non-empty list with focus on head
- `Zipper<T>`: List with movable focus
- `Store<S, A>`: Value with context

## Use Cases

- Cellular automata
- Image processing
- Moving averages
- Contextual operations

## Usage

```rust
use example_649_fp_comonad::{Zipper, moving_average};

let z = Zipper::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
let smoothed = z.extend(moving_average(3));
```
