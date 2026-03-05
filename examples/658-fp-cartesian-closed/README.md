# Cartesian Closed Categories

A CCC provides the categorical semantics for typed lambda calculus.

## Structure

- **Terminal object**: `()` - unit type
- **Products**: `(A, B)` - tuples
- **Exponentials**: `A -> B` - functions

## Key Operations

- `curry`: `(A × B → C) → (A → B → C)`
- `uncurry`: `(A → B → C) → (A × B → C)`
- `eval`: `(B^A × A) → B`

## In Programming

Types form a CCC where:
- Terminal = `()` or `void`
- Products = tuples/structs
- Exponentials = function types

## Usage

```rust
use example_658_fp_cartesian_closed::*;

let add = |(a, b): (i32, i32)| a + b;
let curried = curry(add);
let add_5 = curried(5);
assert_eq!(add_5.apply(3), 8);
```
