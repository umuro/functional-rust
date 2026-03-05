## Core Insight

Rust generics require explicit bounds: `<T: Display>` means "T must implement Display". OCaml generics are unconstrained — any type works if the operations typecheck structurally.

## OCaml Approach
- `'a` is universally polymorphic — no constraints
- Functors for module-level constraints
- No trait bounds — structural typing suffices

## Rust Approach
- `<T: Trait>` syntax for inline bounds
- Multiple bounds: `<T: Display + Clone>`
- Bounds required to call methods on generic T

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Syntax | `'a` (unconstrained) | `<T: Bound>` |
| Multiple | N/A | `T: A + B` |
| Required? | No | Yes, to use methods |
| Checked | At use site | At declaration |
