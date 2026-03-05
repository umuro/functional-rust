# Natural Transformations

A natural transformation η: F => G maps between functors preserving structure.

## Naturality Condition

For any f: A -> B:
`η_B ∘ F(f) = G(f) ∘ η_A`

## Common Examples

- `Option => Vec`: `Some(x) -> vec![x]`, `None -> vec![]`
- `Vec => Option`: `head` operation
- `Result => Option`: `.ok()`
- `Option => Result`: `.ok_or(e)`

## Usage

```rust
use example_654_fp_natural_transformation::*;

let opt = Some(42);
let vec = option_to_vec(opt); // vec![42]

let nums = vec![1, 2, 3];
let first = vec_to_option(nums); // Some(1)
```

## Composability

Natural transformations compose: (η ∘ θ)_A = η_A ∘ θ_A
