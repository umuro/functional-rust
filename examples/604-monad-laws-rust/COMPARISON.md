# Monad Laws

A Monad has `pure` (wrap) and `bind` (chain) operations.

## Laws

1. **Left Identity**: `pure(a).bind(f) == f(a)`
2. **Right Identity**: `m.bind(pure) == m`
3. **Associativity**: `m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))`

## Rust Monads

- `Option<T>` - `Some`/`and_then`
- `Result<T, E>` - `Ok`/`and_then`
- `Vec<T>` - singleton/`flat_map`
