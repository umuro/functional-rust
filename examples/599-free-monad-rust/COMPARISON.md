# Free Monad

## Key Idea

Separate DSL structure from interpretation:
1. Define operations as a functor
2. Wrap in Free to get monadic sequencing
3. Interpret with different backends

## Benefits

- **Testable** - Mock interpretation
- **Composable** - Combine DSLs
- **Analyzable** - Inspect/optimize before running
