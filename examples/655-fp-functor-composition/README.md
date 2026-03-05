# Functor Composition

Functors compose: F ∘ G is a functor when F and G are functors.

## Law

`map_{F∘G}(f) = map_F(map_G(f))`

## Common Compositions

- `Option<Vec<A>>`: Maybe a list
- `Vec<Option<A>>`: List of maybes
- `Result<Vec<A>, E>`: Fallible list
- `Option<Option<A>>`: Nested optionality

## Usage

```rust
use example_655_fp_functor_composition::*;

let ov = Some(vec![1, 2, 3]);
let doubled = map_option_vec(ov, |x| x * 2);
// Some(vec![2, 4, 6])

let vo = vec![Some(1), None, Some(3)];
let result = map_vec_option(vo, |x| x.to_string());
// vec![Some("1"), None, Some("3")]
```
