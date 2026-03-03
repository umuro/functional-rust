# Space Age — Comparison

## Core Insight
Simple enum + pattern matching translates almost identically between OCaml and Rust. When all data is Copy (enums, floats), ownership is invisible. The languages converge on the same clean pattern.

## OCaml Approach
- `type planet = Mercury | Venus | ...` — simple variant type
- `let orbital_period = function | Mercury -> 0.24...` — pattern match function
- `/.` for float division (separate operator from integer `/`)
- No method syntax — free functions

## Rust Approach
- `enum Planet { Mercury, Venus, ... }` with `#[derive(Copy, Clone)]`
- `impl Planet { fn orbital_period(self) -> f64 { match self { ... } } }`
- `/` for both int and float division
- `const ALL` array for iteration over all variants

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Variant | `type planet = Mercury \| ...` | `enum Planet { Mercury, ... }` |
| Match | `function \| pat -> ...` | `match self { pat => ... }` |
| Float ops | `/.` `*.` | `/` `*` |
| Constants | `let x = 31557600.0` | `const X: f64 = 31_557_600.0` |
| Iterate variants | Manual list | `const ALL` array |

## Learner Notes
- Rust has no separate float operators — type inference handles it
- `const` in Rust is compile-time; OCaml `let` at module level is similar
- Rust numeric literals support `_` separators: `31_557_600.0`
- Both languages guarantee exhaustive matching — add a planet, compiler tells you
