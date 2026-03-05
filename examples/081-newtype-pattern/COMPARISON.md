## Core Insight

A newtype wraps a primitive to create a distinct type. `Meters(5.0)` and `Feet(5.0)` are different types — the compiler prevents accidental mixing. Zero runtime overhead in both languages.

## OCaml Approach
- Private types in module signatures
- `type meters = Meters of float` (single-variant)
- Module abstraction hides constructor

## Rust Approach
- Tuple struct: `struct Meters(f64)`
- Can implement traits on the newtype
- Deref for ergonomic access (use sparingly)

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Syntax | `type t = T of inner` | `struct T(inner);` |
| Access | Pattern match | `.0` field access |
| Overhead | Zero | Zero |
| Trait impl | N/A | Can impl traits |
