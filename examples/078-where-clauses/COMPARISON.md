## Core Insight

Where clauses move trait bounds after the function signature for clarity. Essential when bounds involve associated types, multiple type parameters, or complex relationships.

## OCaml Approach
- No direct equivalent — module functors handle complex constraints
- Type constraints in module signatures

## Rust Approach
- `where T: Trait, U: Trait` after signature
- Required for associated type bounds: `where I::Item: Display`
- Cleaner than inline bounds for complex cases

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Simple bound | N/A | `<T: Display>` |
| Complex bound | Functor signature | `where T: A + B, U: C` |
| Associated type | Module type | `where I::Item: Display` |
