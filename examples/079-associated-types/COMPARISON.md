## Core Insight

Associated types define a type placeholder inside a trait. Unlike generic params, there's one impl per type (not per type parameter). `Iterator` has `type Item` — each iterator produces one specific type.

## OCaml Approach
- Module types with abstract types
- `type t` in signatures serves similar purpose
- Functors for type-parameterized modules

## Rust Approach
- `type Item;` in trait definition
- Implementor specifies: `type Item = i32;`
- Used in `Iterator`, `Deref`, `Index`, etc.

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Syntax | `type t` in sig | `type Item;` in trait |
| Specify | Module implementation | `type Item = Concrete;` |
| Multiple | Multiple abstract types | Multiple associated types |
| Example | `module type S = sig type t end` | `trait I { type Item; }` |
