## Core Insight

Pattern matching is the backbone of both languages. The compiler enforces exhaustiveness — you must handle every case. Both support destructuring, guards, wildcards, and nested patterns.

## OCaml Approach
- `match expr with | pattern -> body` syntax
- Variant types matched directly
- Tuple destructuring in patterns
- `when` guards for conditional matching
- `_` wildcard, `as` binding

## Rust Approach
- `match expr { pattern => body }` syntax
- `enum` variants matched with `Enum::Variant`
- Tuple and struct destructuring
- `if` guards in match arms
- `_` wildcard, `@` binding
- `matches!` macro for boolean checks

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Syntax | `match x with \| p -> e` | `match x { p => e }` |
| Guard | `when condition` | `if condition` |
| Wildcard | `_` | `_` |
| Binding | `as name` | `name @` |
| Or-pattern | `p1 \| p2` | `p1 \| p2` |
| Exhaustive | Yes (compiler) | Yes (compiler) |
