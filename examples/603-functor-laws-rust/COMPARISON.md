# Functor Laws

A Functor wraps a value and lets you map functions over it.

## Laws

1. **Identity**: `fmap id == id`
2. **Composition**: `fmap (g . f) == fmap g . fmap f`

## Rust Functors

- `Option<T>` - map over Some
- `Vec<T>` - map over elements
- `Result<T, E>` - map over Ok
