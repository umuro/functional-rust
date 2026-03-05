# Topos Theory

A topos is a category that behaves like the category of sets.

## Key Components

- **Subobject classifier** (Ω): Generalizes `bool`
- **Power object** P(A): Set of subobjects
- **Internal logic**: Intuitionistic logic within the topos

## In Programming

The category Set (with types as objects) is a topos:
- Ω = `bool`
- P(A) = `A -> bool` or `Set<A>`
- Pullbacks = type-level constraints

## Usage

```rust
use example_659_fp_topos::*;

let subset = vec![1, 3, 5];
let chi = char_fn(&subset);
assert_eq!(chi(&3), Omega::True);
assert_eq!(chi(&2), Omega::False);
```
