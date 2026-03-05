# Traversable

Traversable allows mapping with effects while preserving structure.

## Core Operations

- `traverse`: Map with effect, collecting results
- `sequence`: Turn `F<G<A>>` into `G<F<A>>`

## Usage

```rust
use example_645_fp_traversable::{traverse_option, sequence_result};

// Parse all strings or fail
let strings = vec!["1", "2", "3"];
let nums = traverse_option(strings, |s| s.parse::<i32>().ok());
assert_eq!(nums, Some(vec![1, 2, 3]));

// Sequence results
let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2)];
assert_eq!(sequence_result(results), Ok(vec![1, 2]));
```
