# Foldable

Foldable represents data structures that can be reduced to a summary value.

## Core Operations

- `fold_left`: Left-associative fold
- `fold_right`: Right-associative fold

## Derived Operations

- `length`: Count elements
- `sum`: Sum numeric elements
- `any`/`all`: Check predicates
- `to_list`: Convert to list

## Implementations

- `Vec<T>`: Standard vector
- `Option<T>`: Optional values
- `Tree<T>`: Binary tree

## Usage

```rust
use example_644_fp_foldable::{Foldable, Tree, sum, length};

let nums = vec![1, 2, 3, 4];
assert_eq!(sum(nums.clone()), 10);
assert_eq!(length(nums), 4);
```
